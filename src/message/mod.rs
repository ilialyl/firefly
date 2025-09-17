pub mod cmd;
pub mod cursor_movement;
pub mod update;

use std::sync::{Arc, Mutex};

use rust_ffmpeg::FFmpegProcess;

use crate::message::cursor_movement::CursorMovementDirection;

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
    SetBusy,
    UpdateInfo(String),
    PlayerPreviousTrack,
    CycleTabs,
    PlaylistMoveCursor(CursorMovementDirection),
    PlaylistCreate,
    PlaylistRename,
    PlaylistDelete,
    PlaylistSave,
    PlaylistQueueUp,
    PlaylistAddTracks,
    PlaylistAddDir,
    PlaylistRemoveTrack,
    PlaylistToggleArrangeTracks,
    PlaylistToPlayer,
    EnterEditMode,
    ExitEditMode,
}
