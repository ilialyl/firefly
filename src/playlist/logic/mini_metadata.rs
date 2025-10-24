use std::path::Path;

use lofty::{config::ParseOptions, file::TaggedFileExt, probe::Probe, tag::Accessor};

#[derive(Debug, Default, Clone)]
pub struct MiniMetadata {
    pub file_stem: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

impl MiniMetadata {
    pub fn from(path: &Path) -> Self {
        if let Ok(probe) = Probe::open(path)
            && let Ok(tagged_file) = probe
                .options(ParseOptions::new().read_cover_art(false))
                .read()
            && let Some(primary_tag) = tagged_file.primary_tag()
        {
            MiniMetadata {
                file_stem: Some(
                    path.file_stem()
                        .and_then(|stem| stem.to_str())
                        .unwrap_or("[Invalid UTF-8 name]")
                        .to_string(),
                ),
                title: primary_tag.title().map(|s| s.to_string()),
                artist: primary_tag.artist().map(|s| s.to_string()),
                album: primary_tag.album().map(|s| s.to_string()),
            }
        } else {
            MiniMetadata {
                file_stem: Some(
                    path.file_stem()
                        .and_then(|stem| stem.to_str())
                        .unwrap_or("[Invalid UTF-8 name]")
                        .to_string(),
                ),
                ..Default::default()
            }
        }
    }
}
