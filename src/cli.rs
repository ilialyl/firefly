use std::{fs, path::Path};

use clap::Command;
use color_eyre::eyre::Result;
use glob::glob;
use log::info;

use crate::TEMP_FILE_PREFIX;

pub fn cli() -> Command {
    Command::new("firefly")
        .about("Firefly, a terminal music player.")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("clean").about("Remove FLAC files generated from format conversion."),
        )
}

pub fn clear_cache(dir: &Path) -> Result<()> {
    let temp_pattern = glob(
        format!(
            "{}/{}*",
            dir.as_os_str().to_str().unwrap(),
            TEMP_FILE_PREFIX
        )
        .as_str(),
    );
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

    Ok(())
}
