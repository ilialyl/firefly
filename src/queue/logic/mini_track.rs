use std::path::{Path, PathBuf};

use crate::playlist::logic::mini_metadata::MiniMetadata;

/// Stores minimal info about a track. Used in playlists and queues to display metadata while being light to create.
#[derive(Clone, Debug)]
pub struct MiniTrack {
    pub path: PathBuf,
    pub metadata: Option<MiniMetadata>,
}

impl MiniTrack {
    pub fn new(path: &Path) -> Self {
        MiniTrack {
            path: path.to_path_buf(),
            metadata: Some(MiniMetadata::from(path)),
        }
    }
}
