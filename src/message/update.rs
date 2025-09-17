use std::sync::mpsc::Sender;

use crate::{
    message::{
        Message,
        cmd::{self, player_cmd, playlist_cmd},
    },
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
        Message::PlayerLoadNow => player_cmd::load_now(model, msg_tx, info_tx),
        Message::PlayerTogglePlay => player_cmd::toggle_play(model),
        Message::PlayerQueueDir => player_cmd::queue_dir(model),
        Message::PlayerQueueFiles => player_cmd::queue_files(model),
        Message::PlayerMoveQueueUp => player_cmd::move_queue_up(model),
        Message::PlayerMoveQueueDown => player_cmd::move_queue_down(model),
        Message::PlayerRewind => player_cmd::rewind(model, info_tx),
        Message::PlayerSeek => player_cmd::seek(model, info_tx),
        Message::PlayerSkip => player_cmd::skip(model, msg_tx, info_tx),
        Message::PlayerPreviousTrack => player_cmd::previous_track(model, msg_tx, info_tx),
        Message::PlayerToggleArrange => player_cmd::toggle_arrange(model),
        Message::PlayerToggleLoop => player_cmd::toggle_loop(model),
        Message::PlayerDecreaseVolume => player_cmd::decrease_volume(model),
        Message::PlayerIncreaseVolume => player_cmd::increase_volume(model),
        Message::SetBusy => cmd::busy(model),
        Message::ConversionStarted(handle) => cmd::conversion_started(handle, model),
        Message::ConversionEnded => cmd::conversion_ended(model),
        Message::UpdateInfo(info) => cmd::update_info(info, model),
        Message::CycleTabs => cmd::cycle_tabs(model),
        Message::PlaylistCreate => playlist_cmd::create_playlist(&mut model.playlist_controller),
        Message::PlaylistAddTracks => playlist_cmd::add_tracks(&mut model.playlist_controller),
        Message::PlaylistToPlayer => {
            playlist_cmd::send_to_player(&mut model.playlist_controller, &mut model.player)
        }
        Message::PlaylistMoveCursor(direction) => {
            playlist_cmd::move_cursor(direction, &mut model.playlist_controller)
        }
        Message::EnterEditMode => cmd::text_input::enter_edit_mode(model),
        Message::ExitEditMode => cmd::text_input::exit_edit_mode(model),
        Message::Quit => cmd::quit(model),
        _ => None,
    }
}
