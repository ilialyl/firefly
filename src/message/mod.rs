pub mod cmd;
pub mod update;

// use std::sync::{Arc, Mutex};

// use rust_ffmpeg::FFmpegProcess;

pub enum Message {
    Tick,
    LoadNow,
    TogglePlay,
    Skip,
    IncreaseVolume,
    DecreaseVolume,
    Seek,
    Rewind,
    ToggleArrange,
    ToggleLoop,
    MoveQueueUp,
    MoveQueueDown,
    QueueFiles,
    QueueDir,
    Quit,
    // ConversionStarted(Arc<Mutex<FFmpegProcess>>),
    // ConversionEnded,
    Busy,
    UpdateInfo(String),
}
