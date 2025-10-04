use color_eyre::eyre::{Result, eyre};
use rodio::{OutputStream, Sink};
use rust_ffmpeg::FFmpegProcess;
use std::{
    ops::{Add, Sub},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, mpsc::Sender},
    time::Duration,
};

use crate::{
    global::{
        logic::{track::Track, track_queue::TrackQueue},
        message::Message,
    },
    player::logic::playback_status::PlaybackStatus,
};

pub mod playback_status;

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

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
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
        track: &Path,
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

        self.new_track(&path, msg_tx, info_tx)?;

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
            self.queue.prepend_track(&current.real_path);
        }

        self.new_track(&prev, msg_tx, info_tx)?;

        Ok(())
    }

    pub fn shuffle(&mut self) {
        self.queue.shuffle();
    }
}
