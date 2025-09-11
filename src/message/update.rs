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
        Message::QueueFiles => cmd::queue_files(model),
        Message::MoveQueueUp => cmd::move_queue_up(model),
        Message::MoveQueueDown => cmd::move_queue_down(model),
        Message::Rewind => cmd::rewind(model, info_tx),
        Message::Seek => cmd::seek(model, info_tx),
        Message::Skip => cmd::skip(model, msg_tx, info_tx),
        Message::PreviousTrack => cmd::previous_track(model, msg_tx, info_tx),
        Message::ToggleArrange => cmd::toggle_arrange(model),
        Message::ToggleLoop => cmd::toggle_loop(model),
        Message::DecreaseVolume => cmd::decrease_volume(model),
        Message::IncreaseVolume => cmd::increase_volume(model),
        Message::Busy => cmd::busy(model),
        Message::ConversionStarted(handle) => cmd::conversion_started(handle, model),
        Message::ConversionEnded => cmd::conversion_ended(model),
        Message::UpdateInfo(info) => cmd::update_info(info, model),
        Message::CycleTabs => cmd::cycle_tabs(model),
        Message::Quit => cmd::quit(model),
    }
}
