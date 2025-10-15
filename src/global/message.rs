use std::sync::{Arc, Mutex};

use ratatui_image::protocol::StatefulProtocol;
use rust_ffmpeg::FFmpegProcess;

use crate::{
    global::logic::confirmation::Response, player::message::PlayerMessage,
    playlist::message::PlaylistMessage, user_input::message::UserInputMessage,
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
