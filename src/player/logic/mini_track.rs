use std::path::{Path, PathBuf};

use crate::playlist::logic::metadata_cache::MiniMetadata;

#[derive(Clone)]
pub struct MiniTrack {
    pub path: PathBuf,
    pub metadata: MiniMetadata,
}

impl MiniTrack {
    pub fn new(path: &Path) -> Self {
        MiniTrack {
            path: path.to_path_buf(),
            metadata: MiniMetadata::from(path),
        }
    }
}
