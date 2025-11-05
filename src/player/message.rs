use std::{sync::Arc, time::Duration};

use mpris_server::Time;
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
    Seek(Option<Duration>),
    Rewind(Option<Duration>),
    SeekOffset(Time),
    SetPosition(Duration),
    ToggleLoop,
    ConversionStarted(Arc<Mutex<FFmpegProcess>>),
    ConversionEnded,
}
