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
        PlayerMessage::Rewind => rewind(model),
        PlayerMessage::Seek => seek(model),
        PlayerMessage::Skip => skip(model),
        PlayerMessage::PreviousTrack => previous_track(model),
        PlayerMessage::ToggleLoop => toggle_loop(&mut model.player),
        PlayerMessage::DecreaseVolume => decrease_volume(model),
        PlayerMessage::IncreaseVolume => increase_volume(model),
    }
}
