use std::{
    fs::{self, File},
    path::PathBuf,
    sync::{Arc, Mutex, mpsc::Sender},
    thread,
    time::Duration,
};

use color_eyre::eyre::Result;
use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use log::{debug, info};
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

pub struct Track {
    pub real_path: PathBuf,
    temp_path: PathBuf,
    pub pos: Duration,
    pub duration: Duration,
    pub tagged_file: TaggedFile,
    pub conversion_status: FormatConversion,
    pub has_title: bool,
}

impl Track {
    pub fn new(path: PathBuf, msg_tx: &Sender<Message>, info_tx: &Sender<String>) -> Result<Track> {
        let temp_path = Self::get_temp_file(path.clone());
        let tagged_file = Probe::open(path.clone()).unwrap().read().unwrap();

        let conversion_status;
        if !is_rodio_supported(&path)? {
            if temp_path.exists() {
                debug!("Path {:?} exists, skipping conversion", temp_path);
                conversion_status = FormatConversion::Done;
            } else {
                conversion_status = FormatConversion::Running;
            }
        } else {
            conversion_status = FormatConversion::Unnecessary;
        }

        let track = Track {
            real_path: path.clone(),
            temp_path,
            pos: Duration::from_secs(0),
            duration: Self::read_duration(&tagged_file),
            has_title: tagged_file.primary_tag().unwrap().title().is_some(),
            conversion_status,
            tagged_file,
        };

        if track.conversion_status == FormatConversion::Running {
            track.convert_format(msg_tx, info_tx);
        }

        Ok(track)
    }

    pub fn get_temp_file(path: PathBuf) -> PathBuf {
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
            return Ok(source);
        } else {
            let source = Decoder::new(file)?;
            return Ok(Box::new(source));
        }
    }

    pub fn get_path(&self) -> Result<PathBuf> {
        let mut path = &self.real_path;
        if !is_rodio_supported(&path)? {
            path = &self.temp_path;
        }

        Ok(path.clone())
    }

    pub fn sync_pos(&mut self, sink: &Sink) {
        self.pos = sink.get_pos();
    }

    fn convert_format(&self, msg_tx: &Sender<Message>, info_tx: &Sender<String>) {
        let process = Self::build_conversion_process(
            self.real_path.to_path_buf(),
            self.temp_path.to_path_buf(),
        );

        let cloned_temp_path = self.temp_path.to_path_buf();
        let cloned_msg_tx = msg_tx.clone();
        let cloned_info_tx = info_tx.clone();

        info!("Converting file...");
        cloned_info_tx
            .send("Converting format and normalizing volume...".to_string())
            .unwrap();
        thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            let ffmpeg_handle = Arc::new(Mutex::new(runtime.block_on(async { process.await })));
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

    fn build_conversion_process<'a>(
        real_path: PathBuf,
        temp_path: PathBuf,
    ) -> impl Future<Output = FFmpegProcess> + 'a {
        async move {
            FFmpegBuilder::convert(real_path, temp_path)
                .audio_filter(AudioFilter::loudnorm())
                .spawn()
                .await
                .unwrap()
        }
    }
}
