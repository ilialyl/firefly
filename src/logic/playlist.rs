use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use color_eyre::eyre::{Result, eyre};

use crate::data;

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

impl Playlist {
    pub fn save_to_file(&self) -> Result<()> {
        let json_data = serde_json::to_string(&self.entries)?;
        if let Some(name) = &self.name {
            let playlists_path = data::get_data_dir().join("/playlists");
            if !playlists_path.exists() {
                fs::create_dir_all(&playlists_path)
                    .expect("Failed to create playlist data directory.");
            }

            let mut file = File::create(playlists_path.join(format!("/{}", name)))?;
            file.write(json_data.as_bytes())?;

            Ok(())
        } else {
            return Err(eyre!("Playlist name is not set."));
        }
    }

    pub fn load() {}

    pub fn rename(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}
