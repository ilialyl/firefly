use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;
use config::Config;
use glob::glob;
use platform_dirs::AppDirs;
use serde::Deserialize;
use strum_macros::Display;

use crate::app::SESSION_ID;

pub const APP_NAME: &str = "firefly_music";
pub const COVER_ART_DIR: &str = "cover_art/";
const CONFIG_FILE_NAME: &str = "config.toml";

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
        let temp_pattern = glob(format!("{}/{}*", dir_str, *SESSION_ID).as_str());
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

    Ok(())
}

pub fn get_art_cache_path() -> Result<PathBuf> {
    let dir = get_cache_dir()?.join(COVER_ART_DIR);
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

pub fn get_config_path() -> Result<PathBuf> {
    let path = get_data_dir()?.join(CONFIG_FILE_NAME);
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        create_default_config_file(&path)?;
    }

    Ok(path)
}

pub fn create_default_config_file(path: &Path) -> Result<()> {
    let mut file = File::create_new(path)?;
    let default_config = format!("{} = 48000", ConfigKeys::SampleRate);
    file.write(default_config.as_bytes())?;

    Ok(())
}

pub fn load_config() -> Result<HashMap<ConfigKeys, String>> {
    let config = Config::builder()
        .add_source(config::File::from(get_config_path()?))
        .build()?
        .try_deserialize::<HashMap<ConfigKeys, String>>()?;

    Ok(config)
}

#[derive(Deserialize, Hash, PartialEq, Eq, Display)]
pub enum ConfigKeys {
    SampleRate,
}
