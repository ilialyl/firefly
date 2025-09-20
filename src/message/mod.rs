pub mod cmd;
pub mod update;

use std::sync::{Arc, Mutex};

use rust_ffmpeg::FFmpegProcess;

use crate::{
    logic::user_input::{InputTarget, PromptMsg},
    message::cmd::Confirmation,
    view::terminal::CursorMovementDirection,
};

pub enum Message {
    Tick,
    Quit,
    Player(PlayerMessage),
    Playlist(PlaylistMessage),
    UserInput(UserInputMessage),
    ConversionStarted(Arc<Mutex<FFmpegProcess>>),
    ConversionEnded,
    AskConfirmation(Box<Message>),
    Confirm(Confirmation),
    SetBusy,
    UpdateInfo(String),
    CycleTabs,
}

pub enum PlayerMessage {
    LoadNow,
    TogglePlay,
    Skip,
    PreviousTrack,
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
}

pub enum PlaylistMessage {
    LoadPlaylists,
    MoveCursor(CursorMovementDirection),
    Create,
    Rename,
    Delete(Confirmation),
    SaveSelected,
    AddTracks,
    AddDir,
    RemoveTrack,
    ToggleArrangeTracks,
    ToPlayer,
}

pub enum UserInputMessage {
    Submit(InputTarget),
    Insert(char),
    Apply(InputTarget),
    Delete,
    MoveCursorLeft,
    MoveCursorRight,
    EnterEditMode(PromptMsg, InputTarget),
    Exit,
    ExitEarly(InputTarget),
}
