use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;
use glob::glob;
use log::info;
use platform_dirs::AppDirs;

pub const TEMP_FILE_PREFIX: &str = "firefly";
pub const APP_NAME: &str = "firefly_music";

pub fn get_data_dir() -> PathBuf {
    if let Some(app_dirs) = AppDirs::new(Some(APP_NAME), false) {
        app_dirs.data_dir
    } else {
        PathBuf::new()
    }
}

pub fn get_cache_dir() -> PathBuf {
    if let Some(app_dirs) = AppDirs::new(Some(APP_NAME), false) {
        app_dirs.cache_dir
    } else {
        PathBuf::new()
    }
}

pub fn clear_cache(dir: &Path) -> Result<()> {
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
