use std::{
    fs::{create_dir_all, read_dir},
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::{Result, eyre};
use lofty::{file::TaggedFile, probe::Probe};
use rfd::FileDialog;

use crate::global::logic::data::get_data_dir;

pub const RODIO_SUPPORTED_FORMATS: [&str; 5] = ["flac", "mp3", "ogg", "wav", "opus"];
pub const TESTED_FORMATS: [&str; 6] = ["mp3", "flac", "wav", "ogg", "opus", "oga"];
pub const UNTESTED_FORMATS: [&str; 5] = ["pcm", "aiff", "aac", "wma", "alac"];
pub const AUDIO_FORMATS: [&str; 11] = [
    "mp3", "flac", "wav", "ogg", "opus", "oga", "pcm", "aiff", "aac", "wma", "alac",
];

pub fn is_rodio_supported(path: &Path) -> Result<bool> {
    if path.is_file() {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if RODIO_SUPPORTED_FORMATS.contains(&extension) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(eyre!("file has no extension"))
        }
    } else {
        Err(eyre!("path is not a file"))
    }
}

pub fn is_opus(path: &Path) -> Result<bool> {
    if path.is_file() {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if extension == "opus" {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(eyre!("file has no extension"))
        }
    } else {
        Err(eyre!("path is not a file"))
    }
}

pub fn choose_audio_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Tested audio formats", &TESTED_FORMATS)
        .add_filter("Untested audio formats", &UNTESTED_FORMATS)
        .set_directory("~/")
        .pick_file()
}

pub fn choose_multiple_audio_files() -> Option<Vec<PathBuf>> {
    FileDialog::new()
        .add_filter("Tested audio formats", &TESTED_FORMATS)
        .add_filter("Untested audio formats", &UNTESTED_FORMATS)
        .set_directory("~/")
        .pick_files()
}

pub fn choose_dir() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}

pub fn filter_dir_for_audio_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let path_vec: Vec<PathBuf> = read_dir(dir)?
        .filter_map(|r| r.ok())
        .map(|p| p.path())
        .filter(|p| p.is_file())
        .filter_map(|p| {
            p.clone()
                .extension()
                .and_then(|e| e.to_str())
                .filter(|e| AUDIO_FORMATS.contains(e))
                .map(|_| p)
        })
        .collect();

    Ok(path_vec)
}

pub fn read_metadata(track: &Path, track_temp: &Path) -> Result<TaggedFile> {
    match Probe::open(track)?.read() {
        Ok(f) => Ok(f),
        Err(_) => Ok(Probe::open(track_temp)?.read()?),
    }
}

pub fn ffmpeg_available() -> bool {
    let output = Command::new("ffmpeg").arg("-version").output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn get_playlists_path() -> PathBuf {
    let playlists_path = get_data_dir().join("playlists");
    if !playlists_path.exists() {
        log::info!(
            "Attempting to create directory {}",
            playlists_path.to_str().unwrap()
        );
        create_dir_all(&playlists_path).expect("Failed to create playlist data directory.");
    }

    playlists_path
}
