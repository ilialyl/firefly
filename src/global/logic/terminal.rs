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
    global::{
        cmd::Confirmation,
        message::{Message, PlayerMessage, PlaylistMessage, UserInputMessage},
        view::tabs::SelectedTab,
    },
    model::Model,
    user_input::logic::InputMode,
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

fn handle_keys(key_event: KeyEvent, model: &Model) -> Option<Message> {
    match model.input_mode {
        InputMode::Commands => match model.selected_tab {
            SelectedTab::Main => match key_event.code {
                KeyCode::Esc => Some(Message::Quit),
                KeyCode::Char('n') => Some(Message::Player(PlayerMessage::LoadNow)),
                KeyCode::Char(' ') => Some(Message::Player(PlayerMessage::TogglePlay)),
                KeyCode::Char('s') => Some(Message::Player(PlayerMessage::Skip)),
                KeyCode::Char('=') => Some(Message::Player(PlayerMessage::IncreaseVolume)),
                KeyCode::Char('-') => Some(Message::Player(PlayerMessage::DecreaseVolume)),
                KeyCode::Right => Some(Message::Player(PlayerMessage::Seek)),
                KeyCode::Left => Some(Message::Player(PlayerMessage::Rewind)),
                KeyCode::Char('l') => Some(Message::Player(PlayerMessage::ToggleLoop)),
                KeyCode::Char('q') => Some(Message::Player(PlayerMessage::QueueFiles)),
                KeyCode::Char('Q') => Some(Message::Player(PlayerMessage::QueueDir)),
                KeyCode::Up => Some(Message::Player(PlayerMessage::MoveQueueUp)),
                KeyCode::Down => Some(Message::Player(PlayerMessage::MoveQueueDown)),
                KeyCode::Char('a') => Some(Message::Player(PlayerMessage::ToggleArrange)),
                KeyCode::Char('p') => Some(Message::Player(PlayerMessage::PreviousTrack)),
                KeyCode::Tab => Some(Message::CycleTabs),
                _ => None,
            },

            SelectedTab::Playlist => match key_event.code {
                KeyCode::Esc => Some(Message::Quit),
                KeyCode::Char('a') => Some(Message::Playlist(PlaylistMessage::ToggleArrangeTracks)),
                KeyCode::Char('n') => Some(Message::Playlist(PlaylistMessage::Create)),
                KeyCode::Char('Q') => Some(Message::Playlist(PlaylistMessage::AddDir)),
                KeyCode::Char('q') => Some(Message::Playlist(PlaylistMessage::AddTracks)),
                KeyCode::Delete => Some(Message::Playlist(PlaylistMessage::RemoveTrack)),
                KeyCode::Right => Some(Message::Playlist(PlaylistMessage::MoveCursor(
                    CursorMovementDirection::Right,
                ))),
                KeyCode::Left => Some(Message::Playlist(PlaylistMessage::MoveCursor(
                    CursorMovementDirection::Left,
                ))),
                KeyCode::Up => Some(Message::Playlist(PlaylistMessage::MoveCursor(
                    CursorMovementDirection::Up,
                ))),
                KeyCode::Down => Some(Message::Playlist(PlaylistMessage::MoveCursor(
                    CursorMovementDirection::Down,
                ))),
                KeyCode::F(1) => Some(Message::Playlist(PlaylistMessage::ToPlayer)),
                KeyCode::F(2) => Some(Message::Playlist(PlaylistMessage::Rename)),
                KeyCode::F(5) => Some(Message::Playlist(PlaylistMessage::SaveSelected)),
                KeyCode::F(9) => Some(Message::Playlist(PlaylistMessage::Delete(Confirmation::No))),
                KeyCode::Tab => Some(Message::CycleTabs),
                _ => None,
            },
        },
        InputMode::Insert(_, to_edit) => match key_event.code {
            KeyCode::Enter => Some(Message::UserInput(UserInputMessage::Submit(to_edit))),
            KeyCode::Char(c) => Some(Message::UserInput(UserInputMessage::Insert(c))),
            KeyCode::Backspace => Some(Message::UserInput(UserInputMessage::Delete)),
            KeyCode::Left => Some(Message::UserInput(UserInputMessage::MoveCursorLeft)),
            KeyCode::Right => Some(Message::UserInput(UserInputMessage::MoveCursorRight)),
            KeyCode::Esc => Some(Message::UserInput(UserInputMessage::ExitEarly(to_edit))),
            _ => None,
        },
        InputMode::Confirmation => match key_event.code {
            KeyCode::Char('y') => Some(Message::Confirm(Confirmation::Yes)),
            KeyCode::Char('n') => Some(Message::Confirm(Confirmation::No)),
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
