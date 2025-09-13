use std::path::PathBuf;

use crate::logic::{player, playlist::playlist_controller::PlaylistController};

pub fn create_playlist(playlist_controller: &mut PlaylistController) {
    playlist_controller.create_playlist();
}

pub fn add_tracks(playlist_controller: &mut PlaylistController) {
    if let Some(path_vec) = player::choose_multiple_files() {
        let playlist = playlist_controller.get_selected_playlist().unwrap();

        let new_tracks: Vec<PathBuf> = path_vec.into_iter().filter(|p| p.is_file()).collect();
        new_tracks.iter().for_each(|p| playlist.add(p));
    }
}
