use std::sync::mpsc::Sender;

use crate::global::message::Message;

pub struct Senders {
    pub msg: Sender<Message>,
    pub info: Sender<String>,
    pub async_msg: tokio::sync::mpsc::Sender<Message>,
}
