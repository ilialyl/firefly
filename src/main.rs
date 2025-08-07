use std::path::Path;

use color_eyre::eyre::Result;

pub mod player;
pub mod tui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let result = tui::App::new().run(&mut terminal);
    ratatui::restore();

    let temp_file = Path::new("temp.flac");
    if temp_file.exists() {
        std::fs::remove_file(temp_file).expect("Error removing temporary file.");
    }

    result
}
