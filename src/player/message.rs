pub enum PlayerMessage {
    LoadNow,
    TogglePlay,
    Skip,
    PreviousTrack,
    IncreaseVolume(f32),
    DecreaseVolume(f32),
    Seek,
    Rewind,
    ToggleLoop,
}
