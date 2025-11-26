use std::sync::mpsc::Sender;

use tokio::sync::mpsc::UnboundedSender;

use crate::global::message::Message;

/// Stores mpsc channel senders.
pub struct Senders {
    pub msg: Sender<Message>,
    pub info: Sender<String>,
    pub async_msg: UnboundedSender<Message>,
}
