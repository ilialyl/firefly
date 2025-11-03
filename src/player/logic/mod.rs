pub mod format_conversion;
pub mod playback_status;
pub mod track;

use color_eyre::eyre::{Result, eyre};
use mpris_server::{Property, Server};
use rodio::{OutputStream, Sink};
use rust_ffmpeg::FFmpegProcess;
use std::{
    ops::{Add, Sub},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc::Sender;

use crate::{
    global::{logic::mpris::MprisPlayer, message::Message},
    player::logic::{playback_status::PlaybackStatus, track::Track},
    queue::logic::TrackQueue,
};

pub struct Player {
    pub current: Option<Track>,
    // Previous is Vec instead of VecDeque because it's a stack, not queue.
    pub previous: Vec<PathBuf>,
    pub looping: bool,
    pub status: PlaybackStatus,
    pub stream: OutputStream,
    pub sink: Sink,
    pub ffmpeg_handle: Option<Arc<Mutex<FFmpegProcess>>>,
    pub mpris_server: Server<MprisPlayer>,
}

impl Player {
    // Sender is needed because it deals with threads.
    pub async fn new(async_msg_tx: Sender<Message>) -> Result<Player> {
        let (stream, sink) = Self::get_sink()?;
        Ok(Player {
            current: None,
            previous: Vec::<PathBuf>::new(),
            looping: false,
            status: PlaybackStatus::default(),
            stream,
            sink,
            ffmpeg_handle: None,
            mpris_server: Server::new_with_all("Firefly", MprisPlayer { tx: async_msg_tx }).await?,
        })
    }

    pub fn new_track(&mut self, path: &Path) -> Result<()> {
        self.current = Some(Track::new(path)?);

        Ok(())
    }

    pub fn get_sink() -> Result<(OutputStream, Sink)> {
        let mut stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(stream_handle.mixer());

        stream_handle.log_on_drop(false);

        Ok((stream_handle, sink))
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
            if let Err(e) = self.sink.try_seek(current_pos.add(seek_dur)) {
                return Err(eyre!("{e}"));
            };
        } else if track_dur.sub(current_pos) < seek_dur
            && track_dur.sub(current_pos) > Duration::from_secs(1)
        {
            if let Err(e) = self.sink.try_seek(track_dur.sub(Duration::from_secs(1))) {
                return Err(eyre!("{e}"));
            };
        }

        Ok(())
    }

    pub fn rewind(&mut self, rewind_dur: Duration) -> Result<()> {
        if let Some(current) = self.current.as_mut() {
            let current_pos = self.sink.get_pos();
            let rewinded_pos = match current_pos.checked_sub(rewind_dur) {
                Some(dur) => dur,
                None => return Self::reload(self),
            };

            self.sink.clear();
            let source = current.get_source()?;
            self.sink.append(source);

            if let Err(e) = self.sink.try_seek(rewinded_pos) {
                return Err(eyre!("{e}"));
            };

            self.sink.play();
        }

        Ok(())
    }

    pub fn reload(&mut self) -> Result<()> {
        if let Some(current) = self.current.as_mut() {
            self.sink.clear();
            let source = current.get_source()?;
            self.sink.append(source);
            self.sink.play();
        }

        Ok(())
    }

    pub async fn load_next_track(&mut self, queue: &mut TrackQueue) -> Result<()> {
        let path = match queue.dequeue() {
            Some(path) => path,
            None => return Err(eyre!("Queue is empty.")),
        };

        if let Some(current) = self.current.as_mut() {
            self.previous.push(current.real_path.clone());
        }

        self.new_track(&path)?;

        if let Some(current) = self.current.as_ref() {
            self.mpris_server
                .properties_changed([Property::Metadata(current.metadata.clone())])
                .await?;
        }

        Ok(())
    }

    pub fn load_prev_track(&mut self, queue: &mut TrackQueue) -> Result<()> {
        let prev = match self.previous.pop() {
            Some(path) => path,
            None => return Err(eyre!("There are no previous tracks.")),
        };

        if let Some(current) = self.current.as_mut() {
            if queue.is_empty() {
                queue.selected_index = Some(0);
            }
            queue.prepend_track(&current.real_path);
        }

        self.new_track(&prev)?;

        Ok(())
    }
}
