use rodio::{OutputStream, Sink};
use rust_ffmpeg::FFmpegProcess;
use std::sync::{Arc, Mutex};

use crate::{
    logic::player::{self, Track},
    logic::track_queue::TrackQueue,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Busy,
    Done,
}

pub struct Model {
    pub running_state: RunningState,
    pub _stream: OutputStream,
    pub sink: Sink,
    pub status: player::Status,
    pub info_display: String,
    pub current_track: Track,
    pub track_queue: TrackQueue,
    pub looping: bool,
    pub ffmpeg_handle: Option<Arc<Mutex<FFmpegProcess>>>,
}

impl Default for Model {
    fn default() -> Self {
        let (stream, sink) = player::get_sink().expect("Error creating sink");
        Self {
            running_state: RunningState::Running,
            _stream: stream,
            sink,
            status: player::Status::Idle,
            info_display: String::new(),
            current_track: Track {
                path: None,
                pos: None,
                duration: None,
                tagged_file: None,
                has_metadata: false,
            },
            track_queue: TrackQueue::new(),
            looping: false,
            ffmpeg_handle: None,
        }
    }
}
