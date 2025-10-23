use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("firefly")
        .about("Firefly, a terminal music player.")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("clean").about("Remove FLAC files generated from format conversion."),
        )
        .subcommand(Command::new("log").about("Print log path"))
        .subcommand(Command::new("playlist").about("Print playlist path"))
        .subcommand(
            Command::new("add")
                .about("Add an audio file or a directory to be enqueued.")
                .arg(
                    Arg::new("path")
                        .help("Path of a directory or an audio file.")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
        )
}
