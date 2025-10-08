use crate::{
    global::message::{Message, PlayerMessage},
    model::Model,
    player::cmd::*,
};

pub fn update_player(model: &mut Model, msg: PlayerMessage) -> Option<Message> {
    match msg {
        PlayerMessage::LoadNow => load_now(&mut model.player),
        PlayerMessage::TogglePlay => toggle_play(model),
        PlayerMessage::QueueDir => queue_dir(&mut model.player),
        PlayerMessage::QueueFiles => queue_files(&mut model.player),
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
    }
}
