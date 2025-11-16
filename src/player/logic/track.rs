use std::{
    fs::File,
    io::Write,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
        mpsc::Sender,
    },
    time::Duration,
};

use color_eyre::eyre::Result;
use lofty::{
    config::ParseOptions,
    file::{AudioFile, TaggedFile, TaggedFileExt},
    picture::Picture,
    probe::Probe,
    tag::Accessor,
};
use mpris_server::{Metadata, Time, TrackId};
use ratatui_image::protocol::StatefulProtocol;
use rodio::{Decoder, Source};
use rust_ffmpeg::{AudioFilter, FFmpegBuilder};
use tokio::sync::Mutex;

use crate::{
    global::{
        logic::{
            cover_art_server::COVER_ART_ROUTE,
            data::{get_art_cache_path, get_cache_dir},
            files::{is_opus, is_rodio_supported},
            opus::get_opus_source,
        },
        message::Message,
    },
    player::{logic::format_conversion::FormatConversion, message::PlayerMessage},
};

// So that the program can identify whose cover art is whose after decoding in background.
static TRACK_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub struct Track {
    pub id: u32,
    pub real_path: PathBuf,
    pub temp_path: PathBuf,
    pub duration: Option<Duration>,
    pub tagged_file: Option<TaggedFile>,
    pub picture: Option<Picture>,
    pub protocol: Option<StatefulProtocol>,
    pub has_title: bool,
    pub started_decoding: bool,
    pub conversion_status: FormatConversion,
    pub metadata: Metadata,
    cover_server_addr: Option<SocketAddr>,
}

impl Track {
    pub fn new(path: &Path, cover_server_addr: Option<SocketAddr>) -> Result<Track> {
        log::debug!("Creating a new track from path: {:?}", path);
        let id = TRACK_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        let temp_path = Self::get_temp_file(path)?;
        let mut tagged_file = Probe::open(path)?.read().ok();
        if tagged_file.is_none() && temp_path.exists() {
            tagged_file = Probe::open(&temp_path)?.read().ok();
        }

        let conversion_status = if !is_rodio_supported(path)? {
            if temp_path.exists() {
                log::debug!("Path {:?} exists, skipping conversion", temp_path);
                FormatConversion::Done
            } else {
                FormatConversion::Idle
            }
        } else {
            FormatConversion::Unnecessary
        };

        let mut has_title = false;
        let mut duration = None;
        let mut picture = None;

        if let Some(tagged) = tagged_file.as_mut() {
            duration = Some(tagged.properties().duration());
            if let Some(tag) = tagged.primary_tag() {
                has_title = tag.title().is_some();

                if let Some(pic) = tag.pictures().first() {
                    picture = Some(pic.clone());
                }
            }
        };

        let metadata = if temp_path.exists() {
            Self::metadata_from_path(&temp_path, &picture, cover_server_addr, id)
        } else {
            Self::metadata_from_path(path, &picture, cover_server_addr, id)
        }?;

        let track = Track {
            id,
            real_path: path.to_path_buf(),
            temp_path,
            tagged_file,
            duration,
            picture,
            protocol: None,
            has_title,
            started_decoding: false,
            conversion_status,
            metadata,
            cover_server_addr,
        };

        Ok(track)
    }

    pub fn reload_after_conversion(&mut self) -> Result<()> {
        if let Ok(probe) = Probe::open(&self.temp_path)
            && let Ok(tagged_file) = probe.read()
        {
            self.duration = Some(tagged_file.properties().duration());
            if let Some(tag) = tagged_file.primary_tag() {
                self.has_title = tag.title().is_some();
                if let Some(pic) = tag.pictures().first() {
                    self.picture = Some(pic.clone());
                }
            }

            self.tagged_file = Some(tagged_file);

            self.metadata = if self.temp_path.exists() {
                Self::metadata_from_path(
                    &self.temp_path,
                    &self.picture,
                    self.cover_server_addr,
                    self.id,
                )?
            } else {
                Self::metadata_from_path(
                    &self.real_path,
                    &self.picture,
                    self.cover_server_addr,
                    self.id,
                )?
            };
        }

        Ok(())
    }

    pub fn get_temp_file(path: &Path) -> Result<PathBuf> {
        let file_name = path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        Ok(PathBuf::from(format!(
            "{}/{}.flac",
            get_cache_dir()?.to_str().unwrap(),
            file_name
        )))
    }

    pub fn get_source(&self) -> Result<Box<dyn Source<Item = f32> + Send>> {
        let path = self.get_path()?;
        let file = File::open(&path)?;
        if is_opus(&self.get_path()?)? {
            let source = get_opus_source(&path);
            Ok(source)
        } else {
            let source = Decoder::try_from(file)?;
            Ok(Box::new(source))
        }
    }

    pub fn get_path(&self) -> Result<PathBuf> {
        let mut path = &self.real_path;
        if !is_rodio_supported(path)? {
            path = &self.temp_path;
        }

        Ok(path.clone())
    }

    pub fn metadata_from_path(
        path: &Path,
        picture: &Option<Picture>,
        cover_server_addr: Option<SocketAddr>,
        id: u32,
    ) -> Result<Metadata> {
        if let Ok(probe) = Probe::open(path)
            && let Ok(tagged_file) = probe
                .options(ParseOptions::new().read_cover_art(false))
                .read()
            && let Some(primary_tag) = tagged_file.primary_tag()
        {
            let mut metadata = Metadata::new();
            metadata.set_trackid(Some(
                TrackId::try_from(format!("/org/mpris/MediaPlayer2/Firefly/track/{}", id)).unwrap(),
            ));
            metadata.set_title(primary_tag.title());
            metadata.set_album(primary_tag.album());
            metadata.set_artist(primary_tag.artist().map(|s| vec![s.to_string()]));
            metadata.set_length(Some(Time::from_secs(
                tagged_file.properties().duration().as_secs() as i64,
            )));
            metadata.set_track_number(primary_tag.track().map(|n| n as i32));
            metadata.set_genre(primary_tag.genre().map(|s| vec![s.to_string()]));
            if let Some(pic) = picture {
                let file_name = format!("{}.jpg", id);
                log::info!("Cover art: {file_name}");
                let image_path = get_art_cache_path()?.join(&file_name);

                let mut file = File::create(&image_path)?;
                file.write_all(pic.data())?;

                if let Some(addr) = cover_server_addr {
                    metadata.set_art_url(Some(format!(
                        "http://{}{}/{}",
                        addr, COVER_ART_ROUTE, file_name,
                    )));
                }
            }

            if metadata.title().is_none() {
                metadata.set_title(Some(
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("No Title"),
                ));
            }

            return Ok(metadata);
        }

        Ok(Metadata::new())
    }

    pub async fn convert_format(
        real_path: &Path,
        temp_path: &Path,
        async_msg_tx: &tokio::sync::mpsc::Sender<Message>,
        info_tx: &Sender<String>,
    ) {
        let real_path = real_path.to_path_buf();
        let temp_path = temp_path.to_path_buf();
        let msg_tx = async_msg_tx.clone();
        let info_tx = info_tx.clone();

        log::info!("Converting file...");
        if let Err(e) = info_tx.send("Converting format and normalizing volume...".to_string()) {
            log::error!("Error sending info message: {e}");
        }
        tokio::spawn(async move {
            let ffmpeg_handle = Arc::new(Mutex::new(
                FFmpegBuilder::convert(real_path.to_path_buf(), temp_path.to_path_buf())
                    .audio_filter(AudioFilter::loudnorm())
                    .spawn()
                    .await
                    .unwrap(),
            ));
            if let Err(e) = msg_tx
                .send(Message::Player(PlayerMessage::ConversionStarted(
                    ffmpeg_handle.clone(),
                )))
                .await
            {
                log::error!("Error sending FFmpegProcess back to main thread: {e}");
            }

            loop {
                if let Ok(exit) = ffmpeg_handle.lock().await.try_wait()
                    && let Some(exit_status) = exit
                {
                    if exit_status.success() {
                        if let Err(e) = info_tx.send("".to_string()) {
                            log::error!("Error sending info message: {e}");
                        }
                        log::info!("Conversion Complete.");
                        if let Err(e) = msg_tx
                            .send(Message::Player(PlayerMessage::ConversionEnded))
                            .await
                        {
                            log::error!("Error sending ConversionEnded Message: {e}");
                        }
                        break;
                    }
                    if let Err(e) = info_tx.send("".to_string()) {
                        log::error!("Error sending info message: {e}");
                    }
                    if temp_path.is_file() {
                        if let Err(e) = tokio::fs::remove_file(&temp_path).await {
                            log::error!("Error deleting half-converted file: {e}");
                        }
                        log::info!("Deleted {:?}", temp_path);
                    }
                    log::info!("Conversion killed.");
                    break;
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });
    }
}
