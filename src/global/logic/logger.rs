use std::time::SystemTime;

use color_eyre::eyre::Result;

use crate::global::logic::data::get_data_dir;

pub fn setup_logger() -> Result<()> {
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
        .chain(fern::log_file(get_data_dir().join("firefly.log"))?)
        .apply()?;
    Ok(())
}
