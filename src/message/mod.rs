pub mod cmd;
pub mod update;

use std::sync::{Arc, Mutex};

use rust_ffmpeg::FFmpegProcess;

pub enum Message {
    Tick,
    PlayerLoadNow,
    PlayerTogglePlay,
    PlayerSkip,
    PlayerIncreaseVolume,
    PlayerDecreaseVolume,
    PlayerSeek,
    PlayerRewind,
    PlayerToggleArrange,
    PlayerToggleLoop,
    PlayerMoveQueueUp,
    PlayerMoveQueueDown,
    PlayerQueueFiles,
    PlayerQueueDir,
    Quit,
    ConversionStarted(Arc<Mutex<FFmpegProcess>>),
    ConversionEnded,
    Busy,
    UpdateInfo(String),
    PlayerPreviousTrack,
    CycleTabs,
    PlaylistMoveEntryUp,
    PlaylistMoveEntryDown,
    PlaylistCreate,
    PlaylistsNavUp,
    PlaylistsNavDown,
    PlaylistRemove,
    PlaylistSave,
    PlaylistQueueUp,
}
