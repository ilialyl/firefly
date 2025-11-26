use std::{
    fs::{self, create_dir_all, read_dir},
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::{Result, eyre};
use rfd::FileDialog;

use crate::global::logic::data::get_data_dir;

pub const RODIO_SUPPORTED_FORMATS: [&str; 6] = ["flac", "mp3", "ogg", "wav", "opus", "m4a"];
pub const TESTED_FORMATS: [&str; 7] = ["mp3", "flac", "wav", "ogg", "opus", "oga", "m4a"];
pub const UNTESTED_FORMATS: [&str; 5] = ["pcm", "aiff", "aac", "wma", "alac"];
pub const AUDIO_FORMATS: [&str; 12] = [
    "mp3", "flac", "wav", "ogg", "opus", "oga", "pcm", "aiff", "aac", "wma", "alac", "m4a",
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

pub fn choose_dirs() -> Option<Vec<PathBuf>> {
    FileDialog::new().pick_folders()
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

pub fn filter_paths_for_audio_files(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut valid_paths: Vec<PathBuf> = Vec::new();
    paths.into_iter().for_each(|p| {
        if p.exists() {
            if p.is_dir() {
                let audio_files = audio_paths_from_dir(&p);
                valid_paths.extend(audio_files);
            } else if let Some(valid_p) = p
                .clone()
                .extension()
                .and_then(|e| e.to_str())
                .filter(|e| AUDIO_FORMATS.contains(e))
                .map(|_| p)
            {
                valid_paths.push(valid_p);
            }
        }
    });

    valid_paths
}

pub fn ffmpeg_available() -> bool {
    let output = Command::new("ffmpeg").arg("-version").output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn get_playlists_path() -> Result<PathBuf> {
    let playlists_path = get_data_dir()?.join("playlists");
    if !playlists_path.exists() {
        log::info!("Attempting to create directory {:?}", playlists_path);
        create_dir_all(&playlists_path)?;
    }

    Ok(playlists_path)
}

pub fn audio_paths_from_dir(dir: &Path) -> Vec<PathBuf> {
    if let Ok(entries) = fs::read_dir(dir) {
        entries
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
            .collect::<Vec<PathBuf>>()
    } else {
        Vec::<PathBuf>::new()
    }
}
