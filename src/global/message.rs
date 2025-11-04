use std::sync::Arc;

use ratatui_image::protocol::StatefulProtocol;
use rust_ffmpeg::FFmpegProcess;
use tokio::sync::Mutex;

use crate::{
    global::logic::confirmation::Response, player::message::PlayerMessage,
    playlist::message::PlaylistMessage, queue::message::QueueMessage,
    user_input::message::UserInputMessage,
};

pub enum Message {
    Tick,
    Quit,
    Player(PlayerMessage),
    Queue(QueueMessage),
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
