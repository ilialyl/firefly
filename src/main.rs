use color_eyre::eyre::Result;

pub mod player;
pub mod tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = tui::App::new().run(&mut terminal);
    ratatui::restore();
    result
}
