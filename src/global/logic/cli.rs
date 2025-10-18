use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::Path,
};

use clap::{ArgAction, Command, arg};
use color_eyre::eyre::Result;
use glob::glob;
use log::info;
use tail::BackwardsReader;

use crate::global::logic::data::{TEMP_FILE_PREFIX, get_data_dir};

pub fn cli() -> Command {
    Command::new("firefly")
        .about("Firefly, a terminal music player.")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("clean").about("Remove FLAC files generated from format conversion."),
        )
        .subcommand(
            Command::new("log").about("Display lines from log.").arg(
                arg!(-n --nlines "Display recent n number of lines from log.")
                    .action(ArgAction::Set)
                    .value_parser(clap::value_parser!(usize))
                    .default_value("10"),
            ),
        )
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

pub fn display_nlog(number_of_lines: usize) {
    let filename = get_data_dir().join("firefly.log");
    let fd = File::open(filename).unwrap();
    let mut fd = BufReader::new(fd);
    let mut reader = BackwardsReader::new(number_of_lines, &mut fd);

    let mut out = BufWriter::new(std::io::stdout());
    reader.read_all(&mut out);
}
