use std::sync::{Arc, atomic::AtomicBool, mpsc::Sender};

use color_eyre::eyre::Result;

use crate::{
    global::message::Message,
    playlist::logic::{
        Playlist, playlist_collection::PlaylistCollection, playlist_tab_focus::PlaylistTabFocus,
    },
};

/// Will be merged with PlaylistCollection soon.
pub struct PlaylistController {
    pub selected_playlist: Option<usize>,
    pub playlist_coll: PlaylistCollection,
    pub tab_focus: PlaylistTabFocus,
    pub arrange_mode: bool,
}

impl PlaylistController {
    pub fn new(
        msg_tx: Sender<Message>,
        unlocked_tick_rate: Arc<AtomicBool>,
    ) -> Result<PlaylistController> {
        log::info!("Initializing Playlist Controller...");
        let mut playlist_coll = PlaylistCollection::new(msg_tx, unlocked_tick_rate);
        playlist_coll.load_playlists()?;

        let selected_playlist = if playlist_coll.is_empty() {
            None
        } else {
            Some(0)
        };

        Ok(PlaylistController {
            selected_playlist,
            playlist_coll,
            tab_focus: PlaylistTabFocus::default(),
            arrange_mode: false,
        })
    }
    pub fn next_playlist(&mut self) {
        self.selected_playlist = Some(
            (self.selected_playlist.unwrap_or(0) + 1)
                .min(self.playlist_coll.len().saturating_sub(1)),
        );
    }

    pub fn prev_playlist(&mut self) {
        if self.playlist_coll.is_empty() {
            self.selected_playlist = None;
        } else {
            self.selected_playlist = Some(self.selected_playlist.unwrap_or(0).saturating_sub(1));
        }
    }

    pub fn create_playlist(&mut self) -> usize {
        // Create a playlist and return index to it
        let index = self.playlist_coll.create_playlist();

        self.selected_playlist = Some(index);

        index
    }

    pub fn get_selected_playlist(&mut self) -> Option<&mut Playlist> {
        if let Some(idx) = self.selected_playlist {
            self.playlist_coll.get_playlist(idx)
        } else {
            None
        }
    }

    pub fn rename_selected_playlist(&mut self, name: &str) -> Result<()> {
        if !self.playlist_coll.is_empty()
            && let Some(selected) = self.get_selected_playlist()
        {
            selected.rename(name)?;
        }

        Ok(())
    }

    pub fn delete_selected_playlist(&mut self) -> Result<()> {
        if !self.playlist_coll.is_empty()
            && let Some(index) = self.selected_playlist
        {
            Self::delete_playlist(self, index)?;
        }

        Ok(())
    }

    pub fn delete_playlist(&mut self, index: usize) -> Result<()> {
        if let Some(playlist) = self.playlist_coll.get_playlist(index) {
            playlist.trash_save_file()?;
        }

        self.playlist_coll.delete(index);
        if self.playlist_coll.is_empty() {
            self.selected_playlist = None;
        } else if !self.playlist_coll.is_empty() {
            self.selected_playlist = Some(index.saturating_sub(1));
        }

        Ok(())
    }

    pub fn get_all_playlist_names(&self) -> Vec<&str> {
        self.playlist_coll
            .get_playlists()
            .iter()
            .map(|p| p.get_name().unwrap_or("New Playlist"))
            .collect()
    }

    pub fn save_selected_to_file(&mut self) -> Result<()> {
        if let Some(playlist) = Self::get_selected_playlist(self) {
            playlist.save_to_file()?;
        }

        Ok(())
    }

    pub fn load_playlists(&mut self) -> Result<()> {
        self.playlist_coll.load_playlists()
    }
}
