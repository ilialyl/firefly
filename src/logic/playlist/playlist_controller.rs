use crate::logic::playlist::{
    Playlist, playlist_collection::PlaylistCollection, playlist_tab_focus::PlaylistTabFocus,
};

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
        self.selected_playlist = Some(
            (self.selected_playlist.unwrap_or(0) + 1)
                .min(self.playlist_collection.len().checked_sub(1).unwrap_or(0)),
        );
    }

    pub fn prev_playlist(&mut self) {
        if self.playlist_collection.is_empty() {
            self.selected_playlist = None;
        } else {
            self.selected_playlist = Some(
                self.selected_playlist
                    .unwrap_or(0)
                    .checked_sub(1)
                    .unwrap_or(0),
            );
        }
    }

    pub fn create_playlist(&mut self) -> usize {
        // Create a playlist and return index to it
        let index = self.playlist_collection.create_playlist();

        self.selected_playlist = Some(index);

        index
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

    pub fn delete_selected_playlist(&mut self) {
        if !self.playlist_collection.is_empty() {
            if let Some(idx) = self.selected_playlist {
                self.playlist_collection.delete(idx);
                if self.playlist_collection.is_empty() {
                    self.selected_playlist = None;
                }
            }
        }
    }

    pub fn get_all_playlist_names(&self) -> Vec<String> {
        self.playlist_collection
            .get_playlists()
            .iter()
            .map(|p| p.name.clone().unwrap_or("New Playlist".to_string()))
            .collect()
    }
}
