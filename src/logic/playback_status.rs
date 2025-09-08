#[derive(PartialEq, Debug, Default)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    #[default]
    Idle,
}
