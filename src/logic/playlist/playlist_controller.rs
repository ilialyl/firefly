use crate::logic::playlist::{Playlist, playlist_collection::PlaylistCollection};

#[derive(Default)]
pub enum PlaylistTabFocus {
    #[default]
    Playlists,
    Tracks,
}

pub struct PlaylistController {
    pub selected_playlist: Option<usize>,
    pub playlist_collection: PlaylistCollection,
    pub tab_focus: PlaylistTabFocus,
}

impl Default for PlaylistController {
    fn default() -> Self {
        PlaylistController {
            selected_playlist: None,
            playlist_collection: PlaylistCollection::default(),
            tab_focus: PlaylistTabFocus::default(),
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

    pub fn rename_selected_playlist(&mut self, name: &str) {
        if !self.playlist_collection.is_empty() {
            let selected = self.get_selected_playlist().unwrap();
            selected.rename(name);
        }
    }
}
