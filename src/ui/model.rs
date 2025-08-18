use rodio::{OutputStream, Sink};
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::player::{self, Status};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

pub struct Model {
    pub running_state: RunningState,
    pub _stream: OutputStream,
    pub sink: Arc<Mutex<Sink>>,
    pub status: player::Status,
    pub info: Vec<String>,
    pub track_path: Option<PathBuf>,
    pub track_queue: VecDeque<PathBuf>,
    pub track_pos: Option<Duration>,
    pub track_duration: Option<Duration>,
    pub volume: f32,
    pub looping: bool,
}

impl Default for Model {
    fn default() -> Self {
        let (stream, sink) = player::get_sink().expect("Error creating sink");
        Self {
            running_state: RunningState::Running,
            _stream: stream,
            sink: Arc::new(Mutex::new(sink)),
            status: Status::Idle,
            info: vec![String::new()],
            track_path: None,
            track_queue: VecDeque::new(),
            track_pos: None,
            track_duration: None,
            volume: 1.0,
            looping: false,
        }
    }
}
