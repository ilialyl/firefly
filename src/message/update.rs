use std::sync::mpsc::Sender;

use crate::{
    message::{Message, cmd},
    model::Model,
};

use color_eyre::eyre::Result;

pub fn update(
    model: &mut Model,
    msg: Message,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> (Option<Message>, Result<()>) {
    match msg {
        Message::Tick => (None, cmd::tick(model, msg_tx, info_tx)),
        Message::LoadNow => (None, cmd::load_now(model, msg_tx, info_tx)),
        Message::TogglePlay => (None, cmd::toggle_play(model)),
        Message::QueueDir => (None, cmd::queue_dir(model)),
        Message::QueueFile => (None, cmd::queue_file(model)),
        Message::QueueUp => (None, cmd::queue_up(model)),
        Message::QueueDown => (None, cmd::queue_down(model)),
        Message::Rewind => (None, cmd::rewind(model, info_tx)),
        Message::Seek => (None, cmd::seek(model, info_tx)),
        Message::Skip => (None, cmd::skip(model, msg_tx, info_tx)),
        Message::ToggleArrange => (None, cmd::toggle_arrange(model)),
        Message::ToggleLoop => (None, cmd::toggle_loop(model)),
        Message::VolumeDown => (None, cmd::volume_down(model)),
        Message::VolumeUp => (None, cmd::volume_up(model)),
        Message::Busy => (None, cmd::busy(model)),
        Message::ConversionStarted(handle) => {
            (Some(Message::Busy), cmd::conversion_started(handle, model))
        }
        Message::ConversionEnded => (None, cmd::conversion_ended(model)),
        Message::Quit => (None, cmd::quit(model)),
    }
}
