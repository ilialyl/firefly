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
        QueueMessage::Clear => clear(&mut model.queue),
        QueueMessage::MoveDown => move_queue_down(&mut model.queue),
        QueueMessage::MoveUp => move_queue_up(&mut model.queue),
        QueueMessage::QueueDir => queue_dir(&mut model.queue),
        QueueMessage::QueueFiles => queue_files(&mut model.queue, &mut model.player),
        QueueMessage::RemoveSelected => remove_selected(&mut model.queue),
        QueueMessage::ScrollToEnd => scroll_to_end(&mut model.queue),
        QueueMessage::ScrollToStart => scroll_to_start(&mut model.queue),
        QueueMessage::Shuffle => shuffle(&mut model.queue),
        QueueMessage::ToggleArrange => toggle_arrange(&mut model.queue),
        QueueMessage::CreatedMiniTrack(mini_track) => {
            queue_mini_track(mini_track, &mut model.queue)
        }
        QueueMessage::SkipToSelected => skip_to_selected(model),
    }
}
