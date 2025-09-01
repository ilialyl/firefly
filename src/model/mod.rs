use rust_ffmpeg::FFmpegProcess;
use std::sync::{Arc, Mutex};

use crate::logic::{playback_state::PlaybackState, session_state::Session};

pub struct Model {
    pub session: Session,
    pub playback: PlaybackState,
    pub info_display: String,
    pub ffmpeg_handle: Option<Arc<Mutex<FFmpegProcess>>>,
}

impl Default for Model {
    fn default() -> Self {
        let session = Session::new();
        Self {
            playback: PlaybackState::new(session.get_temp()),
            session,
            info_display: String::new(),
            ffmpeg_handle: None,
        }
    }
}
