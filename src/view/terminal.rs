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
};

use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::message::Message;

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

fn handle_keys(key_event: KeyEvent) -> Option<Message> {
    match key_event.code {
        KeyCode::Esc => Some(Message::Quit),
        KeyCode::Char('n') => Some(Message::LoadNow),
        KeyCode::Char(' ') => Some(Message::TogglePlay),
        KeyCode::Char('s') => Some(Message::Skip),
        KeyCode::Char('=') => Some(Message::IncreaseVolume),
        KeyCode::Char('-') => Some(Message::DecreaseVolume),
        KeyCode::Right => Some(Message::Seek),
        KeyCode::Left => Some(Message::Rewind),
        KeyCode::Char('l') => Some(Message::ToggleLoop),
        KeyCode::Char('q') => Some(Message::QueueFiles),
        KeyCode::Char('Q') => Some(Message::QueueDir),
        KeyCode::Up => Some(Message::MoveQueueUp),
        KeyCode::Down => Some(Message::MoveQueueDown),
        KeyCode::Char('a') => Some(Message::ToggleArrange),
        _ => None,
    }
}

pub fn handle_events() -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(16))? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                return Ok(handle_keys(key_event));
            }
            _ => {}
        };
    }
    Ok(None)
}
