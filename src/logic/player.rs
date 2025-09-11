use color_eyre::eyre::{Result, eyre};
use lofty::{file::TaggedFile, probe::Probe};
use rfd::FileDialog;
use rodio::{OutputStream, Sink};
use rust_ffmpeg::FFmpegProcess;
use std::{
    ops::{Add, Sub},
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex, mpsc::Sender},
    time::Duration,
};

use crate::{
    logic::{playback_status::PlaybackStatus, track::Track, track_queue::TrackQueue},
    message::Message,
};

const RODIO_SUPPORTED_FORMATS: [&str; 4] = ["flac", "mp3", "ogg", "wav"];
const TESTED_FORMATS: [&str; 6] = ["mp3", "flac", "wav", "ogg", "opus", "oga"];
const UNTESTED_FORMATS: [&str; 5] = ["pcm", "aiff", "aac", "wma", "alac"];
pub const AUDIO_FORMATS: [&str; 11] = [
    "mp3", "flac", "wav", "ogg", "opus", "oga", "pcm", "aiff", "aac", "wma", "alac",
];

pub struct Player {
    pub current: Option<Track>,
    pub queue: TrackQueue,
    pub previous: Vec<PathBuf>,
    pub looping: bool,
    pub status: PlaybackStatus,
    pub stream: OutputStream,
    pub sink: Sink,
    pub ffmpeg_handle: Option<Arc<Mutex<FFmpegProcess>>>,
}

impl Player {
    pub fn new() -> Player {
        let (stream, sink) = Self::get_sink();
        Player {
            current: None,
            queue: TrackQueue::default(),
            previous: Vec::<PathBuf>::new(),
            looping: false,
            status: PlaybackStatus::default(),
            stream,
            sink,
            ffmpeg_handle: None,
        }
    }

    pub fn new_track(
        &mut self,
        track: PathBuf,
        msg_tx: &Sender<Message>,
        info_tx: &Sender<String>,
    ) -> Result<()> {
        self.current = Some(Track::new(track, msg_tx, info_tx)?);

        Ok(())
    }

    pub fn get_sink() -> (OutputStream, Sink) {
        let mut stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("Error obtaining stream");
        let sink = rodio::Sink::connect_new(stream_handle.mixer());

        stream_handle.log_on_drop(false);

        (stream_handle, sink)
    }

    pub fn increase_volume(&mut self, amount: f32) {
        let current_vol = self.sink.volume();
        let increased_vol = f32::min(current_vol + amount, 2.0);
        self.sink.set_volume(increased_vol);
    }

    pub fn decrease_volume(&mut self, amount: f32) {
        let current_vol = self.sink.volume();
        let decreased_vol = f32::max(current_vol - amount, 0.0);
        self.sink.set_volume(decreased_vol);
    }

    pub fn seek(&mut self, track_dur: &Duration, seek_dur: Duration) -> Result<()> {
        let current_pos = self.sink.get_pos();
        if current_pos.add(seek_dur) < *track_dur {
            self.sink
                .try_seek(current_pos.add(seek_dur))
                .expect("Error seeking");
        } else if track_dur.sub(current_pos) < seek_dur
            && track_dur.sub(current_pos) > Duration::from_secs(1)
        {
            self.sink
                .try_seek(track_dur.sub(Duration::from_secs(1)))
                .expect("Error seeking");
        }

        Ok(())
    }

    pub fn rewind(&mut self, rewind_dur: Duration) -> Result<()> {
        if let Some(ref mut current) = self.current {
            let current_pos = self.sink.get_pos();
            let rewinded_pos = match current_pos.checked_sub(rewind_dur) {
                Some(dur) => dur,
                None => {
                    self.sink.clear();
                    let source = current.get_source()?;
                    self.sink.append(source);
                    self.sink.play();

                    return Ok(());
                }
            };

            self.sink.clear();
            let source = current.get_source()?;
            self.sink.append(source);

            self.sink.try_seek(rewinded_pos).expect("Error rewinding");

            self.sink.play();
        }

        Ok(())
    }

    pub fn reload(&mut self) -> Result<()> {
        if let Some(ref mut current) = self.current {
            self.sink.clear();
            let source = current.get_source()?;
            self.sink.append(source);
            self.sink.play();
        }

        Ok(())
    }

    pub fn load_next_track(
        &mut self,
        msg_tx: &Sender<Message>,
        info_tx: &Sender<String>,
    ) -> Result<()> {
        let path = match self.queue.dequeue() {
            Some(path) => path,
            None => return Err(eyre!("Queue is empty.")),
        };

        if let Some(ref mut current) = self.current {
            self.previous.push(current.real_path.clone());
        }

        self.new_track(path, msg_tx, info_tx)?;

        Ok(())
    }

    pub fn load_prev_track(
        &mut self,
        msg_tx: &Sender<Message>,
        info_tx: &Sender<String>,
    ) -> Result<()> {
        let prev = match self.previous.pop() {
            Some(path) => path,
            None => return Err(eyre!("There are no previous tracks.")),
        };

        if let Some(ref mut current) = self.current {
            self.queue.prepend_track(current.real_path.clone());
        }

        self.new_track(prev, msg_tx, info_tx)?;

        Ok(())
    }
}

pub fn is_rodio_supported(path: &Path) -> Result<bool> {
    if path.is_file() {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if RODIO_SUPPORTED_FORMATS.contains(&extension) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(eyre!("file has no extension"))
        }
    } else {
        Err(eyre!("path is not a file"))
    }
}

pub fn choose_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Tested audio formats", &TESTED_FORMATS)
        .add_filter("Untested audio formats", &UNTESTED_FORMATS)
        .set_directory("~/")
        .pick_file()
}

pub fn choose_multiple_files() -> Option<Vec<PathBuf>> {
    FileDialog::new()
        .add_filter("Tested audio formats", &TESTED_FORMATS)
        .add_filter("Untested audio formats", &UNTESTED_FORMATS)
        .set_directory("~/")
        .pick_files()
}

pub fn choose_dir() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}

pub fn get_metadata(track: &Path, track_temp: &Path) -> Result<TaggedFile> {
    match Probe::open(track)?.read() {
        Ok(f) => Ok(f),
        Err(_) => Ok(Probe::open(track_temp)?.read()?),
    }
}

pub fn check_ffmpeg() -> bool {
    let output = Command::new("ffmpeg").arg("-version").output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
