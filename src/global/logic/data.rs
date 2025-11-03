use std::{
    fs::{self},
    path::PathBuf,
};

use color_eyre::eyre::Result;
use glob::glob;
use log::info;
use platform_dirs::AppDirs;

pub const TEMP_FILE_PREFIX: &str = "firefly";
pub const APP_NAME: &str = "firefly_music";

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
        let temp_pattern = glob(format!("{}/{}*", dir_str, TEMP_FILE_PREFIX).as_str());
        for path in temp_pattern? {
            match path {
                Ok(path) => {
                    if path.is_file() {
                        fs::remove_file(&path)?;
                        println!("Deleted {:?}", path);
                        info!("Deleted {:?}", path);
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    info!("Error: {:?}", e);
                }
            }
        }
    }

    Ok(())
}

pub fn clear_image_cache() -> Result<()> {
    let dir = get_cache_dir()?;
    if let Some(dir_str) = dir.as_os_str().to_str() {
        let temp_pattern = glob(format!("{}/*.jpg", dir_str).as_str());
        for path in temp_pattern? {
            match path {
                Ok(path) => {
                    if path.is_file() {
                        fs::remove_file(&path)?;
                        info!("Deleted {:?}", path);
                    }
                }
                Err(e) => {
                    info!("Error: {:?}", e);
                }
            }
        }
    }

    Ok(())
}
