use std::sync::mpsc::Sender;

use crate::{
    global::message::Message,
    model::Model,
    player::{cmd::*, message::PlayerMessage},
};

pub fn update_player(
    model: &mut Model,
    msg: PlayerMessage,
    _msg_tx: &Sender<Message>,
) -> Option<Message> {
    match msg {
        PlayerMessage::LoadNow => load_now(&mut model.player),
        PlayerMessage::TogglePlay => toggle_play(model),
        PlayerMessage::QueueDir => queue_dir(model),
        PlayerMessage::QueueFiles => queue_files(model),
        PlayerMessage::MoveQueueUp => move_queue_up(&mut model.player),
        PlayerMessage::MoveQueueDown => move_queue_down(&mut model.player),
        PlayerMessage::Rewind => rewind(model),
        PlayerMessage::Seek => seek(model),
        PlayerMessage::Skip => skip(model),
        PlayerMessage::PreviousTrack => previous_track(model),
        PlayerMessage::ToggleArrange => toggle_arrange(&mut model.player),
        PlayerMessage::ToggleLoop => toggle_loop(&mut model.player),
        PlayerMessage::DecreaseVolume => decrease_volume(model),
        PlayerMessage::IncreaseVolume => increase_volume(model),
        PlayerMessage::ShuffleQueue => shuffle_queue(&mut model.player),
        PlayerMessage::ClearQueue => clear_queue(&mut model.player),
        PlayerMessage::CreatedMiniTrack(mini_track) => {
            queue_mini_track(mini_track, &mut model.player)
        }
        PlayerMessage::ScrollToStart => scroll_to_start(&mut model.player.queue),
        PlayerMessage::ScrollToEnd => scroll_to_end(&mut model.player.queue),
        PlayerMessage::RemoveSelectedQueuedTrack => remove_selected_queued_track(&mut model.player),
    }
}
