use std::sync::{Arc, Mutex};

use rust_ffmpeg::FFmpegProcess;

use crate::{
    global::logic::{confirmation::Response, terminal::CursorMovementDirection},
    user_input::logic::{InputTarget, PromptMsg},
};

pub enum Message {
    Tick,
    Quit,
    Player(PlayerMessage),
    Playlist(PlaylistMessage),
    UserInput(UserInputMessage),
    ConversionStarted(Arc<Mutex<FFmpegProcess>>),
    ConversionEnded,
    AskConfirmation(String, Box<Message>),
    Confirm(Response),
    SetBusy,
    UpdateStatusMsg(String),
    CycleTabs,
    AcknowledgeInfo,
    DisplayInfo(String),
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
    ShuffleQueue,
}

pub enum PlaylistMessage {
    LoadPlaylists,
    MoveCursor(CursorMovementDirection),
    Create,
    Rename,
    Delete,
    SaveSelected,
    AddTracks,
    AddDir,
    RemoveTrack,
    ToggleArrangeTracks,
    ToPlayer,
    AskToSave(Box<Option<Message>>),
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
