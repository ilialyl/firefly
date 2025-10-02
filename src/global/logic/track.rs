use std::{
    fs::{self, File},
    io::Cursor,
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
use image::ImageReader;
use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use log::{debug, info};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use rodio::{Decoder, Sink, Source};
use rust_ffmpeg::{AudioFilter, FFmpegBuilder, FFmpegProcess};
use tokio::runtime::Runtime;

use crate::global::{
    logic::{
        data::{TEMP_FILE_PREFIX, get_cache_dir},
        files::{is_opus, is_rodio_supported},
        opus::get_opus_source,
    },
    message::Message,
};

#[derive(PartialEq, Clone, Copy)]
pub enum FormatConversion {
    Running,
    Done,
    Unnecessary,
}

static TRACK_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub struct Track {
    pub real_path: PathBuf,
    temp_path: PathBuf,
    pub pos: Duration,
    pub duration: Option<Duration>,
    pub tagged_file: Option<TaggedFile>,
    pub conversion_status: FormatConversion,
    pub has_title: bool,
    pub img: Option<StatefulProtocol>,
    pub id: u32,
}

impl Track {
    pub fn new(
        path: &Path,
        picker: Arc<Picker>,
        msg_tx: &Sender<Message>,
        info_tx: &Sender<String>,
    ) -> Result<Track> {
        let id = TRACK_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        let temp_path = Self::get_temp_file(path);
        let mut tagged_file = Probe::open(path).unwrap().read().ok();

        let conversion_status;
        if !is_rodio_supported(path)? {
            if temp_path.exists() {
                debug!("Path {:?} exists, skipping conversion", temp_path);
                conversion_status = FormatConversion::Done;
            } else {
                conversion_status = FormatConversion::Running;
            }
        } else {
            conversion_status = FormatConversion::Unnecessary;
        }
        let mut has_title = false;
        let mut duration = None;

        if let Some(ref mut tagged) = tagged_file {
            duration = Some(Self::read_duration(tagged));
            if let Some(ref tag) = tagged.primary_tag() {
                has_title = tag.title().is_some();

                if let Some(picture) = tag.pictures().first() {
                    let picture_data = picture.data().to_vec();
                    let msg_tx_clone = msg_tx.clone();

                    thread::spawn(move || {
                        if let Some(dyn_img) = ImageReader::new(Cursor::new(&picture_data))
                            .with_guessed_format()
                            .ok()
                            .and_then(|r| r.decode().ok())
                        {
                            let protocol = picker.new_resize_protocol(dyn_img);
                            let _ = msg_tx_clone.send(Message::ImageDecoded(protocol, id));
                        }
                    });
                }
            }
        }

        let track = Track {
            real_path: path.to_path_buf(),
            temp_path,
            pos: Duration::from_secs(0),
            duration,
            has_title,
            conversion_status,
            tagged_file,
            img: None,
            id,
        };

        if track.conversion_status == FormatConversion::Running {
            track.convert_format(msg_tx, info_tx);
        }

        Ok(track)
    }

    pub fn reload_after_conversion(&mut self) {
        let tagged_file = Probe::open(&self.temp_path).unwrap().read().unwrap();
        let duration = Self::read_duration(&tagged_file);
        let has_title = tagged_file.primary_tag().unwrap().title().is_some();

        self.tagged_file = Some(tagged_file);
        self.duration = Some(duration);
        self.has_title = has_title;
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

    pub fn read_duration(tagged_file: &TaggedFile) -> Duration {
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

    pub fn sync_pos(&mut self, sink: &Sink) {
        self.pos = sink.get_pos();
    }

    fn convert_format(&self, msg_tx: &Sender<Message>, info_tx: &Sender<String>) {
        let process =
            Self::build_conversion_process(self.real_path.clone(), self.temp_path.clone());

        let cloned_temp_path = self.temp_path.to_path_buf();
        let cloned_msg_tx = msg_tx.clone();
        let cloned_info_tx = info_tx.clone();

        info!("Converting file...");
        cloned_info_tx
            .send("Converting format and normalizing volume...".to_string())
            .unwrap();
        thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            let ffmpeg_handle = Arc::new(Mutex::new(runtime.block_on(process)));
            cloned_msg_tx
                .send(Message::ConversionStarted(ffmpeg_handle.clone()))
                .unwrap();
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
                        cloned_info_tx.send("".to_string()).unwrap();
                        info!("Conversion Complete.");
                        cloned_msg_tx.send(Message::ConversionEnded).unwrap();
                        break;
                    }
                    cloned_info_tx.send("".to_string()).unwrap();
                    if cloned_temp_path.is_file() {
                        fs::remove_file(&cloned_temp_path)
                            .expect("Error deleting half-converted file.");
                        info!("Deleted {:?}", cloned_temp_path);
                    }
                    info!("Conversion killed.");
                    break;
                }
            }
        });
    }

    async fn build_conversion_process(real_path: PathBuf, temp_path: PathBuf) -> FFmpegProcess {
        FFmpegBuilder::convert(real_path, temp_path)
            .audio_filter(AudioFilter::loudnorm())
            .spawn()
            .await
            .unwrap()
    }
}
