use std::path::PathBuf;

use crate::{
    logic::{
        player::{self, Player},
        playlist::playlist_controller::PlaylistController,
    },
    message::Message,
};

pub fn create_playlist(playlist_controller: &mut PlaylistController) -> Option<Message> {
    playlist_controller.create_playlist();

    None
}

pub fn add_tracks(playlist_controller: &mut PlaylistController) -> Option<Message> {
    if let Some(selected) = playlist_controller.get_selected_playlist() {
        if let Some(path_vec) = player::choose_multiple_files() {
            let playlist = selected;

            let new_tracks: Vec<PathBuf> = path_vec.into_iter().filter(|p| p.is_file()).collect();
            new_tracks.iter().for_each(|p| playlist.add(p));
        }
    }
    None
}

pub fn send_to_player(
    playlist_controller: &mut PlaylistController,
    player: &mut Player,
) -> Option<Message> {
    if let Some(selected) = playlist_controller.get_selected_playlist() {
        player
            .queue
            .enqueue_tracks(selected.entries.iter().map(|e| e.to_path_buf()).collect());
    }

    None
}
