use std::{path::PathBuf, time::SystemTime};

use color_eyre::eyre::Result;

use crate::global::logic::data::get_data_dir;

pub fn setup_logger() -> Result<()> {
    let data_dir = get_data_dir()?;
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
        .level(log::LevelFilter::Info)
        .level_for("lofty", log::LevelFilter::Error)
        .level_for("zbus", log::LevelFilter::Error)
        .level_for("axum", log::LevelFilter::Error)
        .level_for("tracing", log::LevelFilter::Error)
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}

pub fn get_log_path() -> Result<PathBuf> {
    Ok(get_data_dir()?.join("firefly.log"))
}
