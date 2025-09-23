#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    #[default]
    Idle,
}
