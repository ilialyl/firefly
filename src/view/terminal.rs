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

use crate::{message::Message, model::Model, view::tabs::SelectedTab};

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

fn handle_keys(key_event: KeyEvent, model: &Model) -> Option<Message> {
    match key_event.code {
        KeyCode::Esc => Some(Message::Quit),
        KeyCode::Char('n') => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerLoadNow),
            SelectedTab::Playlist => Some(Message::PlaylistCreate),
        },
        KeyCode::Char(' ') => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerTogglePlay),
            SelectedTab::Playlist => None,
        },
        KeyCode::Char('s') => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerSkip),
            SelectedTab::Playlist => None,
        },
        KeyCode::Char('=') => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerIncreaseVolume),
            SelectedTab::Playlist => None,
        },
        KeyCode::Char('-') => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerDecreaseVolume),
            SelectedTab::Playlist => None,
        },
        KeyCode::Right => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerSeek),
            SelectedTab::Playlist => None,
        },
        KeyCode::Left => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerRewind),
            SelectedTab::Playlist => None,
        },
        KeyCode::Char('l') => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerToggleLoop),
            SelectedTab::Playlist => None,
        },
        KeyCode::Char('q') => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerQueueFiles),
            SelectedTab::Playlist => Some(Message::PlaylistAddTracks),
        },
        KeyCode::Char('Q') => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerQueueDir),
            SelectedTab::Playlist => None,
        },
        KeyCode::Up => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerMoveQueueUp),
            SelectedTab::Playlist => None,
        },
        KeyCode::Down => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerMoveQueueDown),
            SelectedTab::Playlist => None,
        },
        KeyCode::Char('a') => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerToggleArrange),
            SelectedTab::Playlist => None,
        },
        KeyCode::Char('p') => match model.selected_tab {
            SelectedTab::Main => Some(Message::PlayerPreviousTrack),
            SelectedTab::Playlist => None,
        },
        KeyCode::Tab => Some(Message::CycleTabs),
        _ => None,
    }
}

pub fn handle_events(model: &Model) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(16))? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                return Ok(handle_keys(key_event, model));
            }
            _ => {}
        };
    }
    Ok(None)
}
