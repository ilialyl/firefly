use std::sync::mpsc::Sender;

use crate::{
    global::message::{Message, PlaylistMessage},
    model::Model,
    playlist::cmd::*,
};

pub fn update_playlist(
    model: &mut Model,
    msg: PlaylistMessage,
    _msg_tx: &Sender<Message>,
) -> Option<Message> {
    match msg {
        PlaylistMessage::Create => create_playlist(&mut model.playlist_ctl),
        PlaylistMessage::AddTracks => add_tracks(&mut model.playlist_ctl),
        PlaylistMessage::SendToPlayer => send_to_player(model),
        PlaylistMessage::MoveCursor(direction) => move_cursor(direction, &mut model.playlist_ctl),
        PlaylistMessage::AddDir => add_dir(&mut model.playlist_ctl),
        PlaylistMessage::Delete => {
            delete_playlist(&mut model.user_confirmation.response, &mut model.playlist_ctl)
        }
        PlaylistMessage::RemoveTrack => remove_selected_track(&mut model.playlist_ctl),
        PlaylistMessage::Rename => rename_playlist(&mut model.playlist_ctl),
        PlaylistMessage::SaveSelected => save_selected_playlist(&mut model.playlist_ctl),
        PlaylistMessage::ToggleArrangeTracks => toggle_arrange(&mut model.playlist_ctl),
        PlaylistMessage::LoadPlaylists => load_playlists(&mut model.playlist_ctl),
        PlaylistMessage::AskToSave(then_call) => ask_to_save(
            &mut model.user_confirmation.response,
            *then_call,
            &mut model.playlist_ctl,
        ),
    }
}
