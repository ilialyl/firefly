use crate::{
    global::message::{Message, PlaylistMessage},
    model::Model,
    playlist::cmd::*,
};

pub fn update_playlist(model: &mut Model, msg: PlaylistMessage) -> Option<Message> {
    match msg {
        PlaylistMessage::Create => create_playlist(model),
        PlaylistMessage::AddTracks => add_tracks(&mut model.playlist_ctl),
        PlaylistMessage::ToPlayer => send_to_player(&mut model.playlist_ctl, &mut model.player),
        PlaylistMessage::MoveCursor(direction) => move_cursor(direction, model),
        PlaylistMessage::AddDir => add_dir(&mut model.playlist_ctl),
        PlaylistMessage::Delete => {
            delete_playlist(&mut model.confirmation.response, &mut model.playlist_ctl)
        }
        PlaylistMessage::RemoveTrack => remove_selected_track(&mut model.playlist_ctl),
        PlaylistMessage::Rename => rename_playlist(&mut model.playlist_ctl),
        PlaylistMessage::SaveSelected => save_selected_playlist(&mut model.playlist_ctl),
        PlaylistMessage::ToggleArrangeTracks => toggle_arrange(&mut model.playlist_ctl),
        PlaylistMessage::LoadPlaylists => load_playlists(&mut model.playlist_ctl),
        PlaylistMessage::AskToSave(then_call) => ask_to_save(
            &mut model.confirmation.response,
            *then_call,
            &mut model.playlist_ctl,
        ),
    }
}
