use std::sync::mpsc::Sender;

use crate::{
    global::message::Message,
    model::Model,
    queue::{cmd::*, message::QueueMessage},
};

pub fn update_queue(
    model: &mut Model,
    msg: QueueMessage,
    _msg_tx: &Sender<Message>,
) -> Option<Message> {
    match msg {
        QueueMessage::Clear => clear(&mut model.player),
        QueueMessage::MoveDown => move_queue_down(&mut model.player),
        QueueMessage::MoveUp => move_queue_up(&mut model.player),
        QueueMessage::QueueDir => queue_dir(model),
        QueueMessage::QueueFiles => queue_files(model),
        QueueMessage::RemoveSelected => remove_selected(&mut model.player),
        QueueMessage::ScrollToEnd => scroll_to_end(&mut model.player.queue),
        QueueMessage::ScrollToStart => scroll_to_start(&mut model.player.queue),
        QueueMessage::Shuffle => shuffle(&mut model.player),
        QueueMessage::ToggleArrange => toggle_arrange(&mut model.player),
        QueueMessage::CreatedMiniTrack(mini_track) => {
            queue_mini_track(mini_track, &mut model.player)
        }
    }
}
