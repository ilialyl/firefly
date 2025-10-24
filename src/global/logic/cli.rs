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
            Command::new("with")
                .about("Open with paths of audio files or directories enqueued.")
                .arg(
                    Arg::new("paths")
                        .help("Paths of directories or audio files, works with glob patterns.")
                        .required(true)
                        .num_args(1..)
                        .value_parser(clap::value_parser!(String)),
                ),
        )
}
