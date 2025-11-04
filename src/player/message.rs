use std::sync::Arc;

use rust_ffmpeg::FFmpegProcess;
use tokio::sync::Mutex;

pub enum PlayerMessage {
    LoadNow,
    TogglePlay,
    Skip,
    PreviousTrack,
    IncreaseVolume(f32),
    DecreaseVolume(f32),
    SetVolume(f32),
    Seek,
    Rewind,
    ToggleLoop,
    ConversionStarted(Arc<Mutex<FFmpegProcess>>),
    ConversionEnded,
}
