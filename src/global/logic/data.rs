use std::path::PathBuf;

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
