use std::path::Path;

use lofty::{file::TaggedFileExt, probe::Probe, tag::Accessor};

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
            && let Ok(tagged_file) = probe.read()
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
            MiniMetadata::default()
        }
    }

    pub fn format(&self) -> String {
        let mut s = String::new();
        if let Some(artist) = self.artist.as_ref() {
            s.push_str(&format!(
                "{} - {}",
                artist,
                self.title.as_ref().unwrap_or(&"Unnamed".to_string())
            ));

            if let Some(album) = self.album.as_ref() {
                s.push_str(&format!(" [{}]", album));
            }
        } else {
            s.push_str(self.title.as_ref().unwrap_or(&"Unnamed".to_string()));
        }

        s
    }
}
