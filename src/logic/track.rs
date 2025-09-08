use std::{
    fs::File,
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
use log::info;
use rodio::{Decoder, Sink};
use rust_ffmpeg::{AudioFilter, FFmpegBuilder, FFmpegProcess};
use tokio::runtime::Runtime;

use crate::{TEMP_DIR, logic::player::is_rodio_supported, message::Message};

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
    pub session_code: String,
}

impl Track {
    pub fn new(
        path: PathBuf,
        session_code: String,
        msg_tx: &Sender<Message>,
        info_tx: &Sender<String>,
    ) -> Result<Track> {
        let temp_path = Self::get_temp_file(path.clone(), &session_code);
        let tagged_file = Probe::open(path.clone()).unwrap().read().unwrap();

        let conversion_status;
        if !is_rodio_supported(&path)? {
            conversion_status = FormatConversion::Running;
        } else {
            conversion_status = FormatConversion::Unnecessary;
        }

        let track = Track {
            real_path: path.clone(),
            temp_path,
            session_code,
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

    pub fn get_temp_file(path: PathBuf, session_code: &str) -> PathBuf {
        let file_name = path
            .file_stem()
            .expect("Original path has no file name")
            .to_str()
            .expect("File name is not valid UTF-8");

        PathBuf::from(format!("{}/{}_{}.flac", TEMP_DIR, session_code, file_name))
    }

    pub fn read_duration(tagged_file: &TaggedFile) -> Duration {
        tagged_file.properties().duration()
    }

    pub fn get_source(&self) -> Result<Decoder<File>> {
        let file = File::open(self.get_path()?)?;
        let source = Decoder::new(file)?;

        Ok(source)
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
        Self::wait_conversion(process, msg_tx, info_tx);
    }

    fn wait_conversion<F>(process: F, msg_tx: &Sender<Message>, info_tx: &Sender<String>)
    where
        F: Future<Output = FFmpegProcess> + Send + 'static,
    {
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
                        cloned_msg_tx.send(Message::ConversionEnded).unwrap();
                    }
                    cloned_info_tx.send("".to_string()).unwrap();
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

    // pub async fn convert_format(track_path: &Path, temp_path: &Path) -> FFmpegProcess {
    //     FFmpegBuilder::convert(track_path.to_path_buf(), temp_path.to_path_buf())
    //         .audio_filter(AudioFilter::loudnorm())
    //         .spawn()
    //         .await
    //         .unwrap()
    // }

    // pub fn convert_format_in_bg(
    //     to_convert: &Path,
    //     output: &Path,
    //     msg_tx: &Sender<Message>,
    //     info_tx: &Sender<String>,
    // ) {
    //     let path = to_convert.to_path_buf();
    //     let output = output.to_path_buf();
    //     let cloned_msg_tx = msg_tx.clone();
    //     let cloned_info_tx = info_tx.clone();

    //     info!("Converting file {}.", path.display());
    //     cloned_info_tx
    //         .send("Converting format and normalizing volume...".to_string())
    //         .unwrap();

    //     thread::spawn(move || {
    //         let runtime = Runtime::new().unwrap();
    //         let ffmpeg_handle =
    //             Arc::new(Mutex::new(runtime.block_on(convert_format(&path, &output))));
    //         cloned_msg_tx
    //             .send(Message::ConversionStarted(ffmpeg_handle.clone()))
    //             .unwrap();
    //         loop {
    //             if ffmpeg_handle.lock().unwrap().try_wait().unwrap().is_some() {
    //                 if ffmpeg_handle
    //                     .lock()
    //                     .unwrap()
    //                     .try_wait()
    //                     .unwrap()
    //                     .unwrap()
    //                     .success()
    //                 {
    //                     cloned_info_tx.send("".to_string()).unwrap();
    //                     cloned_msg_tx.send(Message::ConversionEnded).unwrap();
    //                 }
    //                 cloned_info_tx.send("".to_string()).unwrap();
    //                 info!("Conversion killed.");
    //                 break;
    //             }
    //         }
    //     });
    // }
}
