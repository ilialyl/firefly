use std::sync::mpsc::{Receiver, Sender};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::global::message::Message;

/// Stores mpsc channel senders.
pub struct Senders {
    pub msg: Sender<Message>,
    pub info: Sender<String>,
    pub async_msg: UnboundedSender<Message>,
}

pub struct Receivers {
    pub msg: Receiver<Message>,
    pub info: Receiver<String>,
    pub async_msg: UnboundedReceiver<Message>,
}

pub fn channels() -> (Senders, Receivers) {
    let (msg_tx, msg_rx) = std::sync::mpsc::channel::<Message>();
    let (info_tx, info_rx) = std::sync::mpsc::channel::<String>();
    let (msg_async_tx, msg_async_rx) = tokio::sync::mpsc::unbounded_channel();

    (
        Senders {
            msg: msg_tx,
            info: info_tx,
            async_msg: msg_async_tx,
        },
        Receivers {
            msg: msg_rx,
            info: info_rx,
            async_msg: msg_async_rx,
        },
    )
}
