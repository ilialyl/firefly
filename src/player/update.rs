use std::sync::mpsc::Sender;

use crate::{
    global::message::{Message, PlayerMessage},
    model::Model,
    player::cmd::*,
};

pub fn update_player(
    model: &mut Model,
    msg: PlayerMessage,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    match msg {
        PlayerMessage::LoadNow => load_now(model, msg_tx, info_tx),
        PlayerMessage::TogglePlay => toggle_play(model),
        PlayerMessage::QueueDir => queue_dir(model),
        PlayerMessage::QueueFiles => queue_files(model),
        PlayerMessage::MoveQueueUp => move_queue_up(model),
        PlayerMessage::MoveQueueDown => move_queue_down(model),
        PlayerMessage::Rewind => rewind(model, info_tx),
        PlayerMessage::Seek => seek(model, info_tx),
        PlayerMessage::Skip => skip(model),
        PlayerMessage::PreviousTrack => previous_track(model),
        PlayerMessage::ToggleArrange => toggle_arrange(model),
        PlayerMessage::ToggleLoop => toggle_loop(model),
        PlayerMessage::DecreaseVolume => decrease_volume(model),
        PlayerMessage::IncreaseVolume => increase_volume(model),
        PlayerMessage::ShuffleQueue => shuffle_queue(model),
        PlayerMessage::ClearQueue => clear_queue(model),
    }
}
