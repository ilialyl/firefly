use crate::{
    global::message::Message,
    model::Model,
    player::{cmd::*, message::PlayerMessage},
};

pub async fn update_player(model: &mut Model, msg: PlayerMessage) -> Option<Message> {
    match msg {
        PlayerMessage::LoadNow => load_now(model),
        PlayerMessage::TogglePlay => toggle_play(model).await,
        PlayerMessage::Rewind => rewind(model),
        PlayerMessage::Seek => seek(model),
        PlayerMessage::Skip => skip(model).await,
        PlayerMessage::PreviousTrack => previous_track(model).await,
        PlayerMessage::ToggleLoop => toggle_loop(&mut model.player),
        PlayerMessage::IncreaseVolume(amount) => increase_volume(amount, model).await,
        PlayerMessage::DecreaseVolume(amount) => decrease_volume(amount, model).await,
        PlayerMessage::SetVolume(amount) => set_volume(amount, model).await,
        PlayerMessage::ConversionStarted(handle) => conversion_started(handle, model),
        PlayerMessage::ConversionEnded => conversion_ended(model).await,
    }
}
