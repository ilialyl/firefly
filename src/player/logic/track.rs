use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
        mpsc::Sender,
    },
    thread,
    time::Duration,
};

use color_eyre::eyre::Result;
use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    picture::Picture,
    probe::Probe,
    tag::Accessor,
};
use ratatui_image::protocol::StatefulProtocol;
use rodio::{Decoder, Sink, Source};
use rust_ffmpeg::{AudioFilter, FFmpegBuilder};
use tokio::runtime::Runtime;

use crate::{
    global::{
        logic::{
            data::{TEMP_FILE_PREFIX, get_cache_dir},
            files::{is_opus, is_rodio_supported},
            opus::get_opus_source,
        },
        message::Message,
    },
    player::logic::format_conversion::FormatConversion,
};

// So that the program can identify whose cover art is whose after decoding in background.
static TRACK_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub struct Track {
    pub id: u32,
    pub real_path: PathBuf,
    pub temp_path: PathBuf,
    pub pos: Duration,
    pub duration: Option<Duration>,
    pub tagged_file: Option<TaggedFile>,
    pub picture: Option<Picture>,
    pub protocol: Option<StatefulProtocol>,
    pub has_title: bool,
    pub started_decoding: bool,
    pub conversion_status: FormatConversion,
}

impl Track {
    pub fn new(path: &Path) -> Result<Track> {
        log::debug!("Creating a new track from path: {:?}", path);
        let id = TRACK_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        let temp_path = Self::get_temp_file(path);
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
            duration = Some(Self::read_duration_from_tag(tagged));
            if let Some(tag) = tagged.primary_tag() {
                has_title = tag.title().is_some();

                if let Some(pic) = tag.pictures().first() {
                    picture = Some(pic.clone());
                }
            }
        };

        let track = Track {
            id,
            real_path: path.to_path_buf(),
            temp_path,
            tagged_file,
            pos: Duration::default(),
            duration,
            picture,
            protocol: None,
            has_title,
            started_decoding: false,
            conversion_status,
        };

        Ok(track)
    }

    pub fn reload_after_conversion(&mut self) {
        if let Ok(probe) = Probe::open(&self.temp_path)
            && let Ok(tagged_file) = probe.read()
        {
            self.duration = Some(Self::read_duration_from_tag(&tagged_file));
            if let Some(tag) = tagged_file.primary_tag() {
                self.has_title = tag.title().is_some();
                if let Some(pic) = tag.pictures().first() {
                    self.picture = Some(pic.clone());
                }
            }

            self.tagged_file = Some(tagged_file);
        }
    }

    pub fn get_temp_file(path: &Path) -> PathBuf {
        let file_name = path
            .file_stem()
            .expect("Original path has no file name")
            .to_str()
            .expect("File name is not valid UTF-8");

        PathBuf::from(format!(
            "{}/{}_{}.flac",
            get_cache_dir().to_str().unwrap(),
            TEMP_FILE_PREFIX,
            file_name
        ))
    }

    pub fn read_duration_from_tag(tagged_file: &TaggedFile) -> Duration {
        log::debug!("Reading duration from tag...");
        tagged_file.properties().duration()
    }

    pub fn get_source(&self) -> Result<Box<dyn Source<Item = f32> + Send>> {
        let path = self.get_path()?;
        let file = File::open(&path)?;
        if is_opus(&self.get_path()?)? {
            let source = get_opus_source(&path);
            Ok(source)
        } else {
            let source = Decoder::new(file)?;
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

    pub fn sync_pos_from_sink(&mut self, sink: &Sink) {
        self.pos = sink.get_pos();
    }

    pub fn convert_format(
        real_path: &Path,
        temp_path: &Path,
        msg_tx: &Sender<Message>,
        info_tx: &Sender<String>,
    ) {
        let real_path = real_path.to_path_buf();
        let temp_path = temp_path.to_path_buf();
        let msg_tx = msg_tx.clone();
        let info_tx = info_tx.clone();

        log::info!("Converting file...");
        if let Err(e) = info_tx.send("Converting format and normalizing volume...".to_string()) {
            log::error!("Error sending info message: {e}");
        }
        thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            let ffmpeg_handle = Arc::new(Mutex::new(runtime.block_on(async {
                FFmpegBuilder::convert(real_path.to_path_buf(), temp_path.to_path_buf())
                    .audio_filter(AudioFilter::loudnorm())
                    .spawn()
                    .await
                    .unwrap()
            })));
            if let Err(e) = msg_tx.send(Message::ConversionStarted(ffmpeg_handle.clone())) {
                log::error!("Error sending FFmpegProcess back to main thread: {e}");
            }

            loop {
                if ffmpeg_handle.lock().unwrap().try_wait().unwrap().is_some() {
                    if ffmpeg_handle
                        .lock()
                        .unwrap()
                        .try_wait()
                        .unwrap()
                        .unwrap()
                        .success()
                    {
                        if let Err(e) = info_tx.send("".to_string()) {
                            log::error!("Error sending info message: {e}");
                        }
                        log::info!("Conversion Complete.");
                        if let Err(e) = msg_tx.send(Message::ConversionEnded) {
                            log::error!("Error sending ConversionEnded Message: {e}");
                        }
                        break;
                    }
                    if let Err(e) = info_tx.send("".to_string()) {
                        log::error!("Error sending info message: {e}");
                    }
                    if temp_path.is_file() {
                        fs::remove_file(&temp_path).expect("Error deleting half-converted file.");
                        log::info!("Deleted {:?}", temp_path);
                    }
                    log::info!("Conversion killed.");
                    break;
                }
            }
        });
    }
}
