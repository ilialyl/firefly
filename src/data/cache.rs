use std::path::PathBuf;

use platform_dirs::AppDirs;

use crate::data::APP_NAME;

pub fn get_cache_dir() -> PathBuf {
    let app_dirs = AppDirs::new(Some(APP_NAME), false).unwrap();

    app_dirs.cache_dir
}
