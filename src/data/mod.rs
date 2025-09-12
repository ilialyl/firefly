use std::path::PathBuf;

use platform_dirs::AppDirs;

pub mod cache;

pub const TEMP_FILE_PREFIX: &str = "firefly";
pub const APP_NAME: &str = "firefly_music";

pub fn get_data_dir() -> PathBuf {
    let app_dirs = AppDirs::new(Some(APP_NAME), false).unwrap();

    app_dirs.data_dir
}
