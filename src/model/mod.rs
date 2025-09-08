use rust_ffmpeg::FFmpegProcess;
use std::sync::{Arc, Mutex};

use crate::logic::{player::Player, session_state::Session};

pub struct Model {
    pub session: Session,
    pub player: Player,
    pub info_display: String,
    pub ffmpeg_handle: Option<Arc<Mutex<FFmpegProcess>>>,
}

impl Default for Model {
    fn default() -> Self {
        let session = Session::default();
        Self {
            player: Player::new(session.get_code()),
            session,
            info_display: String::new(),
            ffmpeg_handle: None,
        }
    }
}
