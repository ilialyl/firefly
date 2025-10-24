use std::{fs::create_dir_all, path::PathBuf, time::SystemTime};

use color_eyre::eyre::Result;

use crate::global::logic::data::get_data_dir;

pub fn setup_logger() -> Result<()> {
    let data_dir = get_data_dir();
    if !data_dir.exists() {
        create_dir_all(&data_dir).expect("Failed to create data path.");
    }
    let log_path = data_dir.join("firefly.log");

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}

pub fn get_log_path() -> PathBuf {
    get_data_dir().join("firefly.log")
}
