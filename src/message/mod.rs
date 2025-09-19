pub mod cmd;
pub mod cursor_movement;
pub mod update;

use std::sync::{Arc, Mutex};

use rust_ffmpeg::FFmpegProcess;

use crate::{message::cursor_movement::CursorMovementDirection, view::terminal::ToEdit};

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
    EnterEditMode(ToEdit),
    ExitEditMode,
    NamePlaylist(usize),
    InputSubmit(ToEdit),
    InputInsert(char),
    InputApply(ToEdit),
    InputDelete,
    InputMoveCursorLeft,
    InputMoveCursorRight,
}
