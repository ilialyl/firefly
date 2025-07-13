use clap::{Args, Parser, Subcommand};

use crate::player;

#[derive(Parser)]
#[command(name = "firefly")]
#[command(about = "CLI audio player", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
#[command(rename_all = "snake_case")]
pub enum Command {
    Play(PlayArgs),
}

#[derive(Args)]
pub struct PlayArgs {
    #[arg(short = 'f', long = "name")]
    pub file: String,
}

impl Command {
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Play(args) => player::play(args),
        }
    }
}
