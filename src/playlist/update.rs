use crate::{
    global::message::Message,
    model::Model,
    playlist::{cmd::*, message::PlaylistMessage},
};

pub fn update_playlist(model: &mut Model, msg: PlaylistMessage) -> Option<Message> {
    match msg {
        PlaylistMessage::Create => create_playlist(&mut model.playlist_ctl),
        PlaylistMessage::AddTracks => add_tracks(&mut model.playlist_ctl),
        PlaylistMessage::SendToPlayer => send_to_player(model),
        PlaylistMessage::MoveCursor(direction) => move_cursor(direction, &mut model.playlist_ctl),
        PlaylistMessage::AddDir => add_dir(&mut model.playlist_ctl),
        PlaylistMessage::Delete => delete_playlist(
            &mut model.user_confirmation.response,
            &mut model.playlist_ctl,
        ),
        PlaylistMessage::RemoveTrack => remove_selected_track(&mut model.playlist_ctl),
        PlaylistMessage::Rename => rename_playlist(&mut model.playlist_ctl),
        PlaylistMessage::SaveSelected => save_selected_playlist(&mut model.playlist_ctl),
        PlaylistMessage::ToggleArrangeTracks => toggle_arrange(&mut model.playlist_ctl),
        PlaylistMessage::AskToSave(then_call) => ask_to_save(
            &mut model.user_confirmation.response,
            *then_call,
            &mut model.playlist_ctl,
        ),
        PlaylistMessage::ScrollToStart => scroll_to_start(&mut model.playlist_ctl),
        PlaylistMessage::ScrollToEnd => scroll_to_end(&mut model.playlist_ctl),
        PlaylistMessage::LoadedMetadata(index, mini_metadata) => {
            append_metadata(index, mini_metadata, &mut model.playlist_ctl)
        }
    }
}
