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

use crate::{
    message::{Message, cursor_movement::CursorMovementDirection},
    model::Model,
    view::{input_box::InputMode, tabs::SelectedTab},
};

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
    match model.input_mode {
        InputMode::Normal => match model.selected_tab {
            SelectedTab::Main => match key_event.code {
                KeyCode::Esc => Some(Message::Quit),
                KeyCode::Char('n') => Some(Message::PlayerLoadNow),
                KeyCode::Char(' ') => Some(Message::PlayerTogglePlay),
                KeyCode::Char('s') => Some(Message::PlayerSkip),
                KeyCode::Char('=') => Some(Message::PlayerIncreaseVolume),
                KeyCode::Char('-') => Some(Message::PlayerDecreaseVolume),
                KeyCode::Right => Some(Message::PlayerSeek),
                KeyCode::Left => Some(Message::PlayerRewind),
                KeyCode::Char('l') => Some(Message::PlayerToggleLoop),
                KeyCode::Char('q') => Some(Message::PlayerQueueFiles),
                KeyCode::Char('Q') => Some(Message::PlayerQueueDir),
                KeyCode::Up => Some(Message::PlayerMoveQueueUp),
                KeyCode::Down => Some(Message::PlayerMoveQueueDown),
                KeyCode::Char('a') => Some(Message::PlayerToggleArrange),
                KeyCode::Char('p') => Some(Message::PlayerPreviousTrack),
                KeyCode::Tab => Some(Message::CycleTabs),
                _ => None,
            },

            SelectedTab::Playlist => match key_event.code {
                KeyCode::Esc => Some(Message::Quit),
                KeyCode::Char('a') => Some(Message::PlaylistToggleArrangeTracks),
                KeyCode::Char('n') => Some(Message::PlaylistCreate),
                KeyCode::Char('Q') => Some(Message::PlaylistAddDir),
                KeyCode::Char('q') => Some(Message::PlaylistAddTracks),
                KeyCode::Delete => Some(Message::PlaylistRemoveTrack),
                KeyCode::Right => Some(Message::PlaylistMoveCursor(CursorMovementDirection::Right)),
                KeyCode::Left => Some(Message::PlaylistMoveCursor(CursorMovementDirection::Left)),
                KeyCode::Up => Some(Message::PlaylistMoveCursor(CursorMovementDirection::Up)),
                KeyCode::Down => Some(Message::PlaylistMoveCursor(CursorMovementDirection::Down)),
                KeyCode::F(1) => Some(Message::PlaylistToPlayer),
                KeyCode::F(2) => Some(Message::PlaylistRename),
                KeyCode::F(5) => Some(Message::PlaylistDelete),
                KeyCode::F(9) => Some(Message::EnterEditMode),
                KeyCode::Tab => Some(Message::CycleTabs),
                _ => None,
            },
        },
        InputMode::Editing => match key_event.code {
            KeyCode::F(10) => Some(Message::ExitEditMode),
            _ => None,
        },
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
