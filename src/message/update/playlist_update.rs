use crate::{
    message::{Message, PlaylistMessage, cmd::playlist_cmd::*},
    model::Model,
};

pub fn update_playlist(model: &mut Model, msg: PlaylistMessage) -> Option<Message> {
    match msg {
        PlaylistMessage::Create => create_playlist(model),
        PlaylistMessage::AddTracks => add_tracks(&mut model.playlist_ctl),
        PlaylistMessage::ToPlayer => send_to_player(&mut model.playlist_ctl, &mut model.player),
        PlaylistMessage::MoveCursor(direction) => move_cursor(direction, &mut model.playlist_ctl),
        PlaylistMessage::AddDir => todo!(),
        PlaylistMessage::Delete(confirmation) => {
            delete_playlist(&mut model.playlist_ctl, confirmation)
        }
        PlaylistMessage::RemoveTrack => todo!(),
        PlaylistMessage::Rename => rename_playlist(&mut model.playlist_ctl),
        PlaylistMessage::SaveSelected => save_selected_playlist(&mut model.playlist_ctl),
        PlaylistMessage::ToggleArrangeTracks => todo!(),
        PlaylistMessage::LoadPlaylists => load_playlists(&mut model.playlist_ctl),
    }
}
