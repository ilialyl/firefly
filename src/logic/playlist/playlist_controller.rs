use crate::logic::playlist::{Playlist, playlist_collection::PlaylistCollection};

pub struct PlaylistController {
    pub selected_playlist: Option<usize>,
    pub playlist_collection: PlaylistCollection,
}

impl Default for PlaylistController {
    fn default() -> Self {
        PlaylistController {
            selected_playlist: None,
            playlist_collection: PlaylistCollection::default(),
        }
    }
}

impl PlaylistController {
    pub fn next_playlist(&mut self) {
        self.selected_playlist =
            Some((self.selected_playlist.unwrap_or(0) + 1).max(self.playlist_collection.len() - 1));
    }

    pub fn prev_playlist(&mut self) {
        if self.playlist_collection.is_empty() {
            self.selected_playlist = None;
        } else {
            self.selected_playlist = Some((self.selected_playlist.unwrap_or(0) - 1).min(0));
        }
    }

    pub fn create_playlist(&mut self) {
        self.selected_playlist = Some(self.playlist_collection.create_playlist());
    }

    pub fn get_selected_playlist(&mut self) -> Option<&mut Playlist> {
        if let Some(idx) = self.selected_playlist {
            return self.playlist_collection.get_playlist(idx);
        } else {
            None
        }
    }
}
