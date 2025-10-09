use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        ExecutableCommand,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::{
    io::{Stdout, stdout},
    panic,
    sync::atomic::Ordering,
};

use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};

use crate::{
    global::{message::Message, view_logic::keybinds::handle_key_inputs},
    model::Model,
};

#[derive(Clone, Copy)]
pub enum CursorMovementDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn init_terminal() -> color_eyre::Result<Terminal<CrosstermBackend<Stdout>>> {
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

pub fn handle_events(model: &Model) -> Result<Option<Message>> {
    let poll_rate = if model.queuing.load(Ordering::Relaxed) {
        Duration::ZERO
    } else {
        Duration::from_millis(50)
    };

    if event::poll(poll_rate)? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                return Ok(handle_key_inputs(key_event, model));
            }
            _ => {}
        };
    }
    Ok(None)
}
