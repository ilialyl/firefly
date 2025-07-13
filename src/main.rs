use color_eyre::eyre::Result;

pub mod player;
pub mod tui;

// use clap::Parser;

fn main() -> Result<()> {
    // let cli = cli::Cli::parse();
    // cli.command.execute()?;
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = tui::App::new().run(&mut terminal);
    ratatui::restore();
    result
}
