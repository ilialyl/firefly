use std::path::{Path, PathBuf};

use lofty::{file::TaggedFileExt, probe::Probe, tag::Accessor};

#[derive(Clone)]
pub struct MiniTrack {
    pub path: PathBuf,
    pub title: String,
}

impl MiniTrack {
    pub fn new(path: &Path) -> Self {
        MiniTrack {
            path: path.to_path_buf(),
            title: Self::get_title(path),
        }
    }

    pub fn get_title(path: &Path) -> String {
        if let Ok(tagged_file) = Probe::open(path).unwrap().read() {
            tagged_file
                .primary_tag()
                .and_then(|tag| tag.title())
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    path.file_stem()
                        .and_then(|stem| stem.to_str())
                        .unwrap_or("[Invalid UTF-8 name]")
                        .to_string()
                })
        } else {
            return path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("[Invalid UTF-8 name]")
                .to_string();
        }
    }
}
