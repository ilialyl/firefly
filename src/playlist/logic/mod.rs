pub mod mini_metadata;
pub mod playlist_collection;
pub mod playlist_controller;
pub mod playlist_tab_focus;

use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, eyre};

use crate::{
    global::logic::files::get_playlists_path, playlist::logic::mini_metadata::MiniMetadata,
    queue::logic::mini_track::MiniTrack,
};

#[derive(Debug)]
pub struct Playlist {
    name: Option<String>,
    pub tracks: Vec<MiniTrack>,
    pub selected_track: Option<usize>,
    dirty_flag: bool,
    path: Option<PathBuf>,
}

impl Default for Playlist {
    fn default() -> Self {
        Playlist {
            name: None,
            tracks: Vec::new(),
            selected_track: None,
            dirty_flag: true,
            path: None,
        }
    }
}

impl Playlist {
    pub fn save_to_file(&mut self) -> Result<()> {
        self.dirty_flag = false;

        let json_data = serde_json::to_string(&self.get_pathbuf_vec())?;
        if let Some(name) = &self.name {
            let path = Self::get_path_from_name(name);

            let mut file = File::create(&path)?;
            file.write_all(json_data.as_bytes())?;

            self.path = Some(path);

            Ok(())
        } else {
            Err(eyre!("Playlist name is not set."))
        }
    }

    pub fn trash_save_file(&self) {
        if let Some(name) = self.get_name() {
            let path = Self::get_path_from_name(&name);
            let trash_dir = get_playlists_path().join("deleted");
            let trash_path = trash_dir.join(Self::get_filename(&name));

            if path.exists() {
                if !trash_dir.exists() {
                    fs::create_dir_all(&trash_dir)
                        .expect("Failed to create playlist trash directory.");
                }

                if trash_path.exists() {
                    fs::remove_file(&trash_path).expect("Error removing existing trashed file");
                }

                match fs::rename(&path, &trash_path) {
                    Ok(_) => (),
                    Err(_) => {
                        fs::copy(&path, &trash_path).expect("Error copying file to trash");
                        fs::remove_file(&path).expect("Error removing file");
                    }
                }
            }
        }
    }

    pub fn get_filename(playlist_name: &str) -> String {
        format!("{}.json", playlist_name)
    }

    pub fn get_path_from_name(playlist_name: &str) -> PathBuf {
        let playlists_path = get_playlists_path();
        playlists_path.join(Self::get_filename(playlist_name))
    }

    pub fn select_next_track(&mut self, is_arrange: bool) {
        let mut is_arrange = is_arrange;

        if self.tracks.is_empty() {
            return;
        }

        if let Some(selected_index) = self.selected_track
            && selected_index.eq(&self.tracks.len().saturating_sub(1))
        {
            is_arrange = false;
        }

        self.selected_track =
            Some((self.selected_track.unwrap_or(0) + 1).min(self.tracks.len().saturating_sub(1)));

        if let Some(selected_index) = self.selected_track
            && is_arrange
        {
            self.tracks
                .swap(selected_index, selected_index.saturating_sub(1));

            self.dirty_flag = true;
        }
    }

    pub fn select_prev_track(&mut self, is_arrange: bool) {
        let mut is_arrange = is_arrange;
        if self.tracks.is_empty() {
            return;
        }

        if let Some(selected_index) = self.selected_track
            && selected_index.eq(&0)
        {
            is_arrange = false;
        }

        self.selected_track = Some(self.selected_track.unwrap_or(0).saturating_sub(1));

        if let Some(selected_index) = self.selected_track
            && is_arrange
            && self.tracks.len() > selected_index
        {
            self.tracks.swap(selected_index, selected_index + 1);

            self.dirty_flag = true;
        }
    }

    pub fn set_path_loaded_from(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    pub fn reload_from_file(&mut self) -> Result<()> {
        if let Some(path) = self.path.take() {
            let playlist = Self::from(&path)?;
            *self = playlist;
        }

        Ok(())
    }

    pub fn from(file: &Path) -> Result<Playlist> {
        let json_data = fs::read_to_string(file)?;
        let path_vec: Vec<PathBuf> = serde_json::from_str(&json_data)?;
        let tracks = path_vec
            .into_iter()
            .map(|p| MiniTrack {
                path: p,
                metadata: None,
            })
            .collect();

        let mut playlist = Playlist {
            tracks,
            name: Some(
                file.file_stem()
                    .and_then(|os| os.to_str())
                    .unwrap_or("[Invalid UTF-8 name]")
                    .to_owned(),
            ),
            dirty_flag: false,
            ..Default::default()
        };

        playlist.set_path_loaded_from(file.to_path_buf());

        if !playlist.is_empty() {
            playlist.selected_track = Some(0);
        }

        Ok(playlist)
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn rename(&mut self, name: &str) {
        if let Some(current_name) = self.get_name() {
            let path = Self::get_path_from_name(&current_name);
            if path.exists() {
                std::fs::remove_file(&path).unwrap();
            }
        }

        self.name = Some(name.to_string());

        self.save_to_file().expect("Error saving after rename.");
    }

    pub fn add_track_path(&mut self, track: &Path) {
        if track.is_file() {
            if self.is_empty() {
                self.selected_track = Some(0);
            }

            self.tracks.push(MiniTrack {
                path: track.to_path_buf(),
                metadata: None,
            });

            self.dirty_flag = true;
        }
    }

    pub fn remove_selected(&mut self) {
        if let Some(index) = self.selected_track {
            if index == self.len() - 1 {
                self.selected_track = index.checked_sub(1);
            }

            self.remove(index);
        }
    }

    pub fn remove(&mut self, idx: usize) {
        self.tracks.remove(idx);

        if self.is_empty() {
            self.selected_track = None;
        }

        self.dirty_flag = true;
    }

    pub fn as_vec_str(&self) -> Vec<&str> {
        let mut vec_str: Vec<&str> = Vec::new();
        for track in &self.tracks {
            if let Some(os_name) = track.path.file_name() {
                if let Some(name) = os_name.to_str() {
                    vec_str.push(name);
                } else {
                    vec_str.push("[Invalid UTF-8 name]");
                }
            } else {
                vec_str.push("[No file name]");
            }
        }

        vec_str
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty_flag
    }

    pub fn get_path_vec(&self) -> Vec<&Path> {
        self.tracks.iter().map(|t| t.path.as_path()).collect()
    }

    pub fn get_pathbuf_vec(&self) -> Vec<PathBuf> {
        self.tracks.iter().map(|t| t.path.clone()).collect()
    }

    pub fn get_metadata_vec(&self) -> Vec<&Option<MiniMetadata>> {
        self.tracks.iter().map(|t| &t.metadata).collect()
    }
}
