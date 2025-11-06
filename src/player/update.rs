use crate::{
    global::message::Message,
    model::Model,
    player::{cmd::*, message::PlayerMessage},
};

pub async fn update_player(model: &mut Model, msg: PlayerMessage) -> Option<Message> {
    match msg {
        PlayerMessage::LoadNow => load_now(model),
        PlayerMessage::TogglePlay => toggle_play(model).await,
        PlayerMessage::SeekOffset(offset) => seek_offset(offset, model).await,
        PlayerMessage::SetPosition(pos) => set_position(pos, model).await,
        PlayerMessage::Rewind(dur) => rewind(dur, model).await,
        PlayerMessage::Seek(dur) => seek(dur, model).await,
        PlayerMessage::Skip => skip(model).await,
        PlayerMessage::PreviousTrack => previous_track(model).await,
        PlayerMessage::ToggleLoop => toggle_loop(&mut model.player),
        PlayerMessage::IncreaseVolume(amount) => increase_volume(amount, model),
        PlayerMessage::DecreaseVolume(amount) => decrease_volume(amount, model),
        PlayerMessage::SetVolume(amount) => set_volume(amount, model),
        PlayerMessage::ConversionStarted(handle) => conversion_started(handle, model),
        PlayerMessage::ConversionEnded => conversion_ended(model).await,
    }
}
