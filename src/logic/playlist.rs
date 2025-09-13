use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, eyre};
use serde::{Deserialize, Serialize};

use crate::data;

#[derive(Deserialize, Serialize, Debug)]
pub struct Playlist {
    name: Option<String>,
    entries: Vec<PathBuf>,
}

impl Default for Playlist {
    fn default() -> Self {
        Playlist {
            name: None,
            entries: Vec::<PathBuf>::new(),
        }
    }
}

pub struct PlaylistCollection {
    playlists: Vec<Playlist>,
}

impl Default for PlaylistCollection {
    fn default() -> Self {
        PlaylistCollection {
            playlists: Vec::<Playlist>::new(),
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

    pub fn from(file: &Path) -> Result<Playlist> {
        let json_data = fs::read_to_string(file)?;
        let playlist: Playlist = serde_json::from_str(&json_data)?;

        Ok(playlist)
    }

    pub fn rename(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn add(&mut self, track: &Path) {
        if track.is_file() {
            self.entries.push(track.to_path_buf());
        }
    }

    pub fn remove(&mut self, idx: usize) {
        self.entries.remove(idx);
    }

    pub fn as_vec_string(&mut self) -> Vec<String> {
        let mut vec_string: Vec<String> = Vec::new();
        for track in &self.entries {
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
        self.entries.len()
    }
}

impl PlaylistCollection {
    pub fn load_playlists(&mut self) -> Result<()> {
        let mut playlists: Vec<Playlist> = Vec::new();
        let playlist_paths = Self::get_playlist_files()?;
        for path in playlist_paths {
            let playlist = Playlist::from(&path);
            if let Ok(p) = playlist {
                playlists.push(p);
            }
        }

        self.playlists = playlists;

        Ok(())
    }

    pub fn get_playlist_files() -> Result<Vec<PathBuf>> {
        let playlists_path = get_playlists_path();
        let entries = fs::read_dir(playlists_path)?;

        let json_files: Vec<PathBuf> = entries
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_file() && path.extension().map(|ext| ext == "json").unwrap_or(false) {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        Ok(json_files)
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
