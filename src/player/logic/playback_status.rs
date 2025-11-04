#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub enum PlaybackStatus {
    #[default]
    Idle,
    Playing,
    Paused,
}

impl PlaybackStatus {
    pub fn as_mpris_playback_status(&self) -> mpris_server::PlaybackStatus {
        match self {
            PlaybackStatus::Idle => mpris_server::PlaybackStatus::Stopped,
            PlaybackStatus::Paused => mpris_server::PlaybackStatus::Playing,
            PlaybackStatus::Playing => mpris_server::PlaybackStatus::Paused,
        }
    }
}
