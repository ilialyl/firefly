pub mod handle;
pub mod update;

use std::sync::{Arc, Mutex};

use rust_ffmpeg::FFmpegProcess;

pub enum Message {
    Tick,
    LoadNow,
    PlayPause,
    Skip,
    VolumeUp,
    VolumeDown,
    Seek,
    Rewind,
    ToggleArrange,
    ToggleLoop,
    QueueUp,
    QueueDown,
    QueueFile,
    QueueDir,
    Quit,
    ConversionStarted(Arc<Mutex<FFmpegProcess>>),
    ConversionEnded,
    Busy,
}
