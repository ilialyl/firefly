use std::sync::mpsc::Sender;

use crate::{
    message::{Message, cmd},
    model::Model,
};

pub fn update(
    model: &mut Model,
    msg: Message,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    match msg {
        Message::Tick => cmd::tick(model, msg_tx, info_tx),
        Message::PlayerLoadNow => cmd::load_now(model, msg_tx, info_tx),
        Message::PlayerTogglePlay => cmd::toggle_play(model),
        Message::PlayerQueueDir => cmd::queue_dir(model),
        Message::PlayerQueueFiles => cmd::queue_files(model),
        Message::PlayerMoveQueueUp => cmd::move_queue_up(model),
        Message::PlayerMoveQueueDown => cmd::move_queue_down(model),
        Message::PlayerRewind => cmd::rewind(model, info_tx),
        Message::PlayerSeek => cmd::seek(model, info_tx),
        Message::PlayerSkip => cmd::skip(model, msg_tx, info_tx),
        Message::PlayerPreviousTrack => cmd::previous_track(model, msg_tx, info_tx),
        Message::PlayerToggleArrange => cmd::toggle_arrange(model),
        Message::PlayerToggleLoop => cmd::toggle_loop(model),
        Message::PlayerDecreaseVolume => cmd::decrease_volume(model),
        Message::PlayerIncreaseVolume => cmd::increase_volume(model),
        Message::Busy => cmd::busy(model),
        Message::ConversionStarted(handle) => cmd::conversion_started(handle, model),
        Message::ConversionEnded => cmd::conversion_ended(model),
        Message::UpdateInfo(info) => cmd::update_info(info, model),
        Message::CycleTabs => cmd::cycle_tabs(model),
        Message::PlaylistCreate => cmd::playlist::create_playlist(&mut model.playlist_controller),
        Message::PlaylistAddTracks => cmd::playlist::add_tracks(&mut model.playlist_controller),
        Message::PlaylistToPlayer => {
            cmd::playlist::send_to_player(&mut model.playlist_controller, &mut model.player)
        }
        Message::PlaylistCycleCursorFocus(direction) => {
            cmd::playlist::cycle_playlist_focus(direction, &mut model.playlist_controller)
        }
        Message::Quit => cmd::quit(model),
        _ => None,
    }
}
