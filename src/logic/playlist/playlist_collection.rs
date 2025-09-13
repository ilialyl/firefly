use std::{fs, path::PathBuf};

use color_eyre::eyre::Result;

use crate::logic::playlist::{Playlist, get_playlists_path};

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

    pub fn create_playlist(&mut self) -> usize {
        // Create a new playlist and returns index to it
        self.playlists.push(Playlist::default());

        (self.playlists.len() - 1) as usize
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
}
