pub mod message;
pub mod model;
pub mod view;

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        ExecutableCommand,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::{io::stdout, panic};

use crate::ui::{
    message::{Message, update},
    model::Model,
    view::view,
};

pub fn refresh_frame(model: &mut Model, terminal: &mut DefaultTerminal) -> Result<()> {
    update(model, Message::Tick);
    terminal.draw(|f| view(model, f))?;

    Ok(())
}

pub fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

pub fn restore_terminal() -> color_eyre::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}
