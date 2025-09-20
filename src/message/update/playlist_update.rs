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
        PlaylistMessage::Delete => todo!(),
        PlaylistMessage::NamePlaylist(_) => todo!(),
        PlaylistMessage::QueueUp => todo!(),
        PlaylistMessage::RemoveTrack => todo!(),
        PlaylistMessage::Rename => todo!(),
        PlaylistMessage::Save => todo!(),
        PlaylistMessage::ToggleArrangeTracks => todo!(),
    }
}
