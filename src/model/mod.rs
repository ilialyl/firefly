pub mod player;

use rodio::{OutputStream, Sink};
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::model::player::Track;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

pub struct Model {
    pub running_state: RunningState,
    pub busy: Arc<Mutex<bool>>,
    pub selected_track: usize,
    pub arrange_mode: bool,
    pub _stream: OutputStream,
    pub sink: Arc<Mutex<Sink>>,
    pub status: player::Status,
    pub info: Vec<String>,
    pub current_track: Track,
    pub track_queue: VecDeque<PathBuf>,
    pub volume: f32,
    pub looping: bool,
}

impl Default for Model {
    fn default() -> Self {
        let (stream, sink) = player::get_sink().expect("Error creating sink");
        Self {
            running_state: RunningState::Running,
            busy: Arc::new(Mutex::new(false)),
            selected_track: 0,
            arrange_mode: false,
            _stream: stream,
            sink: Arc::new(Mutex::new(sink)),
            status: player::Status::Idle,
            info: vec![String::new()],
            current_track: Track {
                path: None,
                pos: None,
                duration: None,
                tagged_file: None,
                has_metadata: false,
            },
            track_queue: VecDeque::new(),
            volume: 1.0,
            looping: false,
        }
    }
}
