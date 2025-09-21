use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, eyre};
use serde::{Deserialize, Serialize};

use crate::data;

pub mod playlist_collection;
pub mod playlist_controller;
pub mod playlist_tab_focus;

#[derive(Deserialize, Serialize, Debug)]
pub struct Playlist {
    name: Option<String>,
    pub tracks: Vec<PathBuf>,
    pub selected_track: Option<usize>,
}

impl Default for Playlist {
    fn default() -> Self {
        Playlist {
            name: None,
            tracks: Vec::<PathBuf>::new(),
            selected_track: None,
        }
    }
}

impl Playlist {
    pub fn save_to_file(&self) -> Result<()> {
        let json_data = serde_json::to_string(&self)?;
        if let Some(name) = &self.name {
            let path = Self::get_path_from_name(name);

            let mut file = File::create(path)?;
            file.write(json_data.as_bytes())?;

            Ok(())
        } else {
            return Err(eyre!("Playlist name is not set."));
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
            && selected_index.eq(&self.tracks.len().checked_sub(1).unwrap_or(0))
        {
            is_arrange = false;
        }

        self.selected_track = Some(
            (self.selected_track.unwrap_or(0) + 1)
                .min(self.tracks.len().checked_sub(1).unwrap_or(0)),
        );

        if let Some(selected_index) = self.selected_track
            && is_arrange
        {
            self.tracks
                .swap(selected_index, selected_index.checked_sub(1).unwrap_or(0));
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

        self.selected_track = Some(self.selected_track.unwrap_or(0).checked_sub(1).unwrap_or(0));

        if let Some(selected_index) = self.selected_track
            && is_arrange
            && self.tracks.len() > selected_index
        {
            self.tracks.swap(selected_index, selected_index + 1);
        }
    }

    pub fn from(file: &Path) -> Result<Playlist> {
        let json_data = fs::read_to_string(file)?;
        let playlist: Playlist = serde_json::from_str(&json_data)?;

        Ok(playlist)
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn rename(&mut self, name: &str) {
        if let Some(current_name) = self.get_name() {
            let path = Self::get_path_from_name(&current_name);
            if path.exists() {
                std::fs::remove_file(&path).unwrap();
            }
        }

        self.name = Some(name.to_string());
    }

    pub fn add(&mut self, track: &Path) {
        if track.is_file() {
            if self.is_empty() {
                self.selected_track = Some(0);
            }

            self.tracks.push(track.to_path_buf());
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
    }

    pub fn as_vec_string(&mut self) -> Vec<String> {
        let mut vec_string: Vec<String> = Vec::new();
        for track in &self.tracks {
            if let Some(os_name) = track.file_name() {
                if let Some(name) = os_name.to_str() {
                    vec_string.push(name.to_string());
                } else {
                    vec_string.push("[Invalid UTF-8 name]".to_string());
                }
            } else {
                vec_string.push("[No file name]".to_string());
            }
        }

        vec_string
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }
}

fn get_playlists_path() -> PathBuf {
    let playlists_path = data::get_data_dir().join("playlists");
    if !playlists_path.exists() {
        log::info!(
            "Attempting to create directory {}",
            playlists_path.to_str().unwrap()
        );
        fs::create_dir_all(&playlists_path).expect("Failed to create playlist data directory.");
    }

    playlists_path
}
