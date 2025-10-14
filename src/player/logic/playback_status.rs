#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub enum PlaybackStatus {
    #[default]
    Idle,
    Playing,
    Paused,
}
