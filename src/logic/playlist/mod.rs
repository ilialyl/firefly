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
            let playlists_path = get_playlists_path();

            let mut file = File::create(playlists_path.join(format!("{}.json", name)))?;
            file.write(json_data.as_bytes())?;

            Ok(())
        } else {
            return Err(eyre!("Playlist name is not set."));
        }
    }

    pub fn select_next_track(&mut self) {
        self.selected_track =
            Some((self.selected_track.unwrap_or(0) + 1).min(self.tracks.len() - 1));
    }

    pub fn select_prev_track(&mut self) {
        self.selected_track = Some((self.selected_track.unwrap_or(0) - 1).max(0));
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
        self.name = Some(name.to_string());
    }

    pub fn add(&mut self, track: &Path) {
        if track.is_file() {
            self.tracks.push(track.to_path_buf());
            self.update_selected_track();
        }
    }

    pub fn update_selected_track(&mut self) {
        if self.tracks.is_empty() {
            self.selected_track = None;
        } else {
            self.selected_track = Some(0);
        }
    }

    pub fn remove(&mut self, idx: usize) {
        self.tracks.remove(idx);
        self.update_selected_track();
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
