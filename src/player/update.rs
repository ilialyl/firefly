use crate::{
    app::App,
    global::message::Message,
    player::{cmd::*, message::PlayerMessage},
};

pub async fn update_player(app: &mut App, msg: PlayerMessage) -> Option<Message> {
    match msg {
        PlayerMessage::LoadNow => load_now(app),
        PlayerMessage::TogglePlay => toggle_play(app).await,
        PlayerMessage::Play => play(app).await,
        PlayerMessage::Pause => pause(app).await,
        PlayerMessage::SeekOffset(offset) => seek_offset(offset, app).await,
        PlayerMessage::SetPosition(pos) => set_position(pos, app).await,
        PlayerMessage::Rewind(dur) => rewind(dur, app).await,
        PlayerMessage::Seek(dur) => seek(dur, app).await,
        PlayerMessage::Next => play_next_track(app).await,
        PlayerMessage::PreviousTrack => previous_track(app).await,
        PlayerMessage::ToggleLoop => toggle_loop(&mut app.player).await,
        PlayerMessage::IncreaseVolume(amount) => increase_volume(amount, app).await,
        PlayerMessage::DecreaseVolume(amount) => decrease_volume(amount, app).await,
        PlayerMessage::SetVolume(amount) => set_volume(amount, app).await,
        PlayerMessage::ConversionStarted(handle) => conversion_started(handle, app),
        PlayerMessage::ConversionEnded => conversion_ended(app).await,
        PlayerMessage::SyncMprisPos(done_tx) => sync_mpris_pos(done_tx, app).await,
        PlayerMessage::SyncMprisVolume => sync_mpris_volume(app).await,
        PlayerMessage::ClearCurrentSession => clear_current_session(app).await,
    }
}
