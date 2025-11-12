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
        PlayerMessage::Next => play_next_track(model).await,
        PlayerMessage::PreviousTrack => previous_track(model).await,
        PlayerMessage::ToggleLoop => toggle_loop(&mut model.player).await,
        PlayerMessage::IncreaseVolume(amount) => increase_volume(amount, model).await,
        PlayerMessage::DecreaseVolume(amount) => decrease_volume(amount, model).await,
        PlayerMessage::SetVolume(amount) => set_volume(amount, model).await,
        PlayerMessage::ConversionStarted(handle) => conversion_started(handle, model),
        PlayerMessage::ConversionEnded => conversion_ended(model).await,
        PlayerMessage::SyncMprisPos(done_tx) => sync_mpris_pos(done_tx, model).await,
        PlayerMessage::SyncMprisVolume => sync_mpris_volume(model).await,
    }
}
