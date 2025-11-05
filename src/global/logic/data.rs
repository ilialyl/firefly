use std::{
    fs::{self},
    path::PathBuf,
};

use color_eyre::eyre::Result;
use glob::glob;
use platform_dirs::AppDirs;

pub const APP_NAME: &str = "firefly_music";
pub const COVER_ART_DIR: &str = "cover_art/";
pub const ADDRESS_FILE_NAME: &str = "address.txt";

pub fn get_data_dir() -> Result<PathBuf> {
    if let Some(app_dirs) = AppDirs::new(Some(APP_NAME), false) {
        let data_dir = app_dirs.data_dir;
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }

        Ok(data_dir)
    } else {
        Ok(PathBuf::from("."))
    }
}

pub fn get_cache_dir() -> Result<PathBuf> {
    if let Some(app_dirs) = AppDirs::new(Some(APP_NAME), false) {
        let cache_dir = app_dirs.cache_dir;
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }

        Ok(cache_dir)
    } else {
        Ok(PathBuf::from("."))
    }
}

pub fn clear_all_cache() -> Result<()> {
    let dir = get_cache_dir()?;
    if let Some(dir_str) = dir.as_os_str().to_str() {
        let temp_pattern = glob(format!("{}/*", dir_str).as_str());
        for path in temp_pattern? {
            match path {
                Ok(path) => {
                    if path.is_file() {
                        fs::remove_file(&path)?;
                        println!("Deleted {:?}", path);
                        log::info!("Deleted {:?}", path);
                    }
                }
                Err(e) => {
                    log::error!("Error: {:?}", e);
                }
            }
        }
    }

    Ok(())
}

pub fn clear_art_cache() -> Result<()> {
    let dir = get_art_cache_path()?;
    if let Some(dir_str) = dir.as_os_str().to_str() {
        let temp_pattern = glob(format!("{}/*", dir_str).as_str());
        for path in temp_pattern? {
            match path {
                Ok(path) => {
                    if path.is_file() {
                        fs::remove_file(&path)?;
                        log::info!("Deleted {:?}", path);
                    }
                }
                Err(e) => {
                    log::error!("Error: {:?}", e);
                }
            }
        }
    }

    if dir.exists() && dir.is_dir() {
        fs::remove_dir(dir)?;
    }

    Ok(())
}

pub fn get_art_cache_path() -> Result<PathBuf> {
    let dir = get_cache_dir()?.join(COVER_ART_DIR);
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}
