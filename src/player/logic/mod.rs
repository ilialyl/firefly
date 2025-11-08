pub mod format_conversion;
pub mod playback_status;
pub mod track;

use color_eyre::eyre::{Result, eyre};
use mpris_server::{LoopStatus, Property, Server, Signal, Time};
use rodio::{OutputStream, Sink};
use rust_ffmpeg::FFmpegProcess;
use std::{
    net::SocketAddr,
    ops::{Add, Sub},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::sync::{Mutex, RwLock, mpsc::Sender};

use crate::{
    global::{
        logic::mpris::{MprisPlayer, MprisPlayerState},
        message::Message,
    },
    player::logic::{playback_status::PlaybackStatus, track::Track},
    queue::logic::TrackQueue,
};

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 2.0;
pub const DEFAULT_VOLUME_CHANGE_AMOUNT: f32 = 0.05;

/// Deals with stuff like current track, previous tracks, looping bool, sink, and MPRIS server.
pub struct Player {
    pub current: Option<Track>,
    // Previous is Vec instead of VecDeque because it's a stack, not queue.
    pub previous: Vec<PathBuf>,
    pub looping: bool,
    pub status: PlaybackStatus,
    pub stream: OutputStream,
    pub sink: Sink,
    pub ffmpeg_handle: Option<Arc<Mutex<FFmpegProcess>>>,
    pub mpris_server: Option<Server<MprisPlayer>>,
    pub mpris_state: Option<Arc<RwLock<MprisPlayerState>>>,
    pub cover_server_addr: Option<SocketAddr>,
}

impl Player {
    // Sender is needed because it deals with threads.
    pub async fn new(
        async_msg_tx: Sender<Message>,
        cover_server_addr: Option<SocketAddr>,
    ) -> Result<Player> {
        let (stream, sink) = Self::get_sink()?;
        let mpris_state = Arc::new(RwLock::new(MprisPlayerState::default()));
        let mpris_server = Server::new_with_all(
            "Firefly",
            MprisPlayer {
                tx: async_msg_tx,
                state: mpris_state.clone(),
            },
        )
        .await
        .ok();
        let mpris_state = if mpris_server.is_some() {
            Some(mpris_state)
        } else {
            None
        };

        Ok(Player {
            current: None,
            previous: Vec::<PathBuf>::new(),
            looping: false,
            status: PlaybackStatus::default(),
            stream,
            sink,
            ffmpeg_handle: None,
            mpris_server,
            mpris_state,
            cover_server_addr,
        })
    }

    pub fn new_track(&mut self, path: &Path) -> Result<()> {
        self.current = Some(Track::new(path, self.cover_server_addr)?);

        Ok(())
    }

    pub fn get_sink() -> Result<(OutputStream, Sink)> {
        let mut stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(stream_handle.mixer());

        stream_handle.log_on_drop(false);

        Ok((stream_handle, sink))
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub async fn set_volume(&mut self, amount: f32) -> Result<()> {
        self.sink.set_volume(amount.clamp(MIN_VOLUME, MAX_VOLUME));
        if let Some(mpris_server) = self.mpris_server.as_ref()
            && let Some(mpris_state) = self.mpris_state.as_ref()
        {
            let vol = self.volume() as f64;
            mpris_server
                .properties_changed([Property::Volume(vol)])
                .await?;

            mpris_state.write().await.volume = vol;
        }

        Ok(())
    }

    pub async fn set_position(&mut self, position: Duration) -> Result<()> {
        if let Some(current_track) = self.current.as_ref()
            && let Some(dur) = current_track.duration
            && dur > position
        {
            if let Err(e) = self.sink.try_seek(position) {
                return Err(eyre!("{e}"));
            };
        }

        Ok(())
    }

    pub fn seek(&mut self, track_dur: &Duration, seek_dur: Duration) -> Result<()> {
        let current_pos = self.sink.get_pos();
        if current_pos.add(seek_dur) < *track_dur {
            if let Err(e) = self.sink.try_seek(current_pos.add(seek_dur)) {
                return Err(eyre!("{e}"));
            };
        } else if track_dur.sub(current_pos) < seek_dur
            && track_dur.sub(current_pos) > Duration::from_secs(1)
            && let Err(e) = self.sink.try_seek(track_dur.sub(Duration::from_secs(1)))
        {
            return Err(eyre!("{e}"));
        };

        Ok(())
    }

    pub fn rewind(&mut self, rewind_dur: Duration) -> Result<()> {
        if self.current.is_some() {
            let current_pos = self.sink.get_pos();
            let rewinded_pos = match current_pos.checked_sub(rewind_dur) {
                Some(dur) => dur,
                None => return self.reload(),
            };

            if let Err(e) = self.sink.try_seek(rewinded_pos) {
                return Err(eyre!("{e}"));
            };
        }

        Ok(())
    }

    pub fn reload(&mut self) -> Result<()> {
        if let Some(current) = self.current.as_mut() {
            self.sink.clear();
            let source = current.get_source()?;
            self.sink.append(source);

            self.update_sink_playback_status()?;
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

        self.update_mpris_metadata().await?;

        Ok(())
    }

    pub async fn load_prev_track(&mut self, queue: &mut TrackQueue) -> Result<()> {
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

        self.update_mpris_metadata().await?;

        Ok(())
    }

    pub async fn update_mpris_metadata(&mut self) -> Result<()> {
        if let Some(mpris_server) = self.mpris_server.as_ref()
            && let Some(current) = self.current.as_ref()
            && let Some(mpris_state) = self.mpris_state.as_ref()
        {
            mpris_server
                .properties_changed([Property::Metadata(current.metadata.clone())])
                .await?;

            mpris_state.write().await.metadata = current.metadata.clone();
        }

        Ok(())
    }

    pub fn update_sink_playback_status(&mut self) -> Result<()> {
        match self.status {
            PlaybackStatus::Idle => self.sink.play(),
            PlaybackStatus::Paused => self.sink.pause(),
            PlaybackStatus::Playing => self.sink.play(),
        }

        Ok(())
    }

    pub async fn update_mpris_pos(&mut self) -> Result<()> {
        if self.current.is_some()
            && let Some(mpris_server) = self.mpris_server.as_ref()
            && let Some(mpris_state) = self.mpris_state.as_ref()
        {
            mpris_server
                .emit(Signal::Seeked {
                    position: Time::from_secs(self.sink.get_pos().as_secs() as i64),
                })
                .await?;

            mpris_state.write().await.position =
                Time::from_secs(self.sink.get_pos().as_secs() as i64);
        }

        Ok(())
    }

    pub async fn update_mpris_playback_status(&mut self) -> Result<()> {
        if self.current.is_some()
            && let Some(mpris_server) = self.mpris_server.as_ref()
            && let Some(mpris_state) = self.mpris_state.as_ref()
        {
            mpris_server
                .properties_changed([Property::PlaybackStatus(
                    self.status.as_mpris_playback_status(),
                )])
                .await?;

            mpris_state.write().await.playback_status = self.status.as_mpris_playback_status();
        }

        Ok(())
    }

    pub async fn toggle_loop(&mut self) -> Result<()> {
        self.looping = !self.looping;

        if let Some(mpris_server) = self.mpris_server.as_ref()
            && let Some(mpris_state) = self.mpris_state.as_ref()
        {
            let loop_status = if self.looping {
                LoopStatus::Track
            } else {
                LoopStatus::None
            };

            mpris_server
                .properties_changed([Property::LoopStatus(loop_status)])
                .await?;

            mpris_state.write().await.loop_status = loop_status;
        }

        Ok(())
    }
}
