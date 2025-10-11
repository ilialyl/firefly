use std::sync::{Arc, Mutex};

use ratatui_image::protocol::StatefulProtocol;
use rust_ffmpeg::FFmpegProcess;

use crate::{
    global::{logic::confirmation::Response, view_logic::terminal::CursorMovementDirection},
    player::logic::mini_track::MiniTrack,
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
    UpdateInfoMsg(String),
    CycleTabs,
    AcknowledgeInfo,
    DisplayInfoMsg(String),
    ProtocolCreated(StatefulProtocol, u32),
    ShowHelp,
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
    ClearQueue,
    RemoveSelectedQueuedTrack,
    CreatedMiniTrack(MiniTrack),
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
    SendToPlayer,
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
