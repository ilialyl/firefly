use std::{fs, path::PathBuf};

use color_eyre::eyre::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::playlist::logic::{Playlist, get_playlists_path};

pub struct PlaylistCollection {
    playlists: Vec<Playlist>,
}

impl Default for PlaylistCollection {
    fn default() -> Self {
        log::debug!("Initialized PlaylistCollection");

        PlaylistCollection {
            playlists: Vec::<Playlist>::new(),
        }
    }
}

impl PlaylistCollection {
    pub fn load_playlists(&mut self) -> Result<()> {
        log::debug!("Loading Playlists");
        let paths = Self::get_playlist_files()?;
        let playlists = paths
            .par_iter()
            .filter_map(|p| Playlist::from(p).ok())
            .collect::<Vec<Playlist>>();

        self.playlists = playlists;

        Ok(())
    }

    pub fn get_playlist_files() -> Result<Vec<PathBuf>> {
        let playlists_path = get_playlists_path();
        log::info!("Playlist Directory: {:?}", playlists_path.to_str());
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

    pub fn create_playlist(&mut self) -> usize {
        // Create a new playlist and returns index to it
        self.playlists.push(Playlist::default());

        self.playlists.len() - 1
    }

    pub fn len(&self) -> usize {
        self.playlists.len()
    }

    pub fn is_empty(&self) -> bool {
        self.playlists.is_empty()
    }

    pub fn get_playlist(&mut self, idx: usize) -> Option<&mut Playlist> {
        self.playlists.get_mut(idx)
    }

    pub fn get_playlists(&self) -> &Vec<Playlist> {
        &self.playlists
    }

    pub fn delete(&mut self, idx: usize) {
        self.playlists.remove(idx);
    }
}
