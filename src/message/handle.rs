use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::message::Message;

fn keys(key_event: KeyEvent) -> Option<Message> {
    match key_event.code {
        KeyCode::Esc => Some(Message::Quit),
        KeyCode::Char('n') => Some(Message::LoadNow),
        KeyCode::Char(' ') => Some(Message::TogglePlay),
        KeyCode::Char('s') => Some(Message::Skip),
        KeyCode::Char('=') => Some(Message::VolumeUp),
        KeyCode::Char('-') => Some(Message::VolumeDown),
        KeyCode::Right => Some(Message::Seek),
        KeyCode::Left => Some(Message::Rewind),
        KeyCode::Char('l') => Some(Message::ToggleLoop),
        KeyCode::Char('q') => Some(Message::QueueFile),
        KeyCode::Char('Q') => Some(Message::QueueDir),
        KeyCode::Up => Some(Message::QueueUp),
        KeyCode::Down => Some(Message::QueueDown),
        KeyCode::Char('a') => Some(Message::ToggleArrange),
        _ => None,
    }
}

pub fn events() -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(16))? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                return Ok(keys(key_event));
            }
            _ => {}
        };
    }
    Ok(None)
}
