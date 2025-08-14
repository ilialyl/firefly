use std::path::Path;

use color_eyre::eyre::Result;

pub mod app;
pub mod player;
pub mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let result = app::App::new().run(&mut terminal);
    ratatui::restore();

    let temp_file = Path::new(player::CONVERTED_TRACK);
    if temp_file.exists() {
        std::fs::remove_file(temp_file).expect("Error removing temporary file.");
    }

    result
}
