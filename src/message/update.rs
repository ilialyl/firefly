use std::sync::mpsc::Sender;

use crate::{
    message::{Message, cmd},
    model::Model,
};

pub fn update(
    model: &mut Model,
    msg: Message,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    match msg {
        Message::Tick => cmd::tick(model, msg_tx, info_tx),
        Message::LoadNow => cmd::load_now(model, msg_tx, info_tx),
        Message::TogglePlay => cmd::toggle_play(model),
        Message::QueueDir => cmd::queue_dir(model),
        Message::QueueFile => cmd::queue_file(model),
        Message::QueueUp => cmd::queue_up(model),
        Message::QueueDown => cmd::queue_down(model),
        Message::Rewind => cmd::rewind(model, info_tx),
        Message::Seek => cmd::seek(model, info_tx),
        Message::Skip => cmd::skip(model, msg_tx, info_tx),
        Message::ToggleArrange => cmd::toggle_arrange(model),
        Message::ToggleLoop => cmd::toggle_loop(model),
        Message::VolumeDown => cmd::volume_down(model),
        Message::VolumeUp => cmd::volume_up(model),
        Message::Busy => cmd::busy(model),
        Message::ConversionStarted(handle) => cmd::conversion_started(handle, model),
        Message::ConversionEnded => cmd::conversion_ended(model),
        Message::Quit => cmd::quit(model),
    }
}
