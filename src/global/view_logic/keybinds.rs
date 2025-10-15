use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    global::{
        logic::confirmation::Response,
        message::{Message, PlayerMessage, PlaylistMessage, UserInputMessage},
        view_logic::{focused_area::FocusedArea, terminal::CursorMovementDirection},
    },
    model::Model,
    user_input::logic::InputMode,
};

pub fn handle_key_inputs(key_event: KeyEvent, model: &Model) -> Option<Message> {
    match model.input_mode {
        InputMode::Commands => match model.focused_view_area {
            FocusedArea::Playlist => match key_event.code {
                KeyCode::Esc => Some(Message::Quit),
                KeyCode::Char('a') => Some(Message::Playlist(PlaylistMessage::ToggleArrangeTracks)),
                KeyCode::Char('n') => Some(Message::Playlist(PlaylistMessage::Create)),
                KeyCode::Char('W') => Some(Message::Playlist(PlaylistMessage::AddDir)),
                KeyCode::Char('w') => Some(Message::Playlist(PlaylistMessage::AddTracks)),
                KeyCode::Char(' ') => Some(Message::Player(PlayerMessage::TogglePlay)),
                KeyCode::Enter => Some(Message::Playlist(PlaylistMessage::SendToPlayer)),
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
                KeyCode::F(1) => Some(Message::Playlist(PlaylistMessage::SendToPlayer)),
                KeyCode::F(2) => Some(Message::Playlist(PlaylistMessage::Rename)),
                KeyCode::F(9) => Some(Message::Playlist(PlaylistMessage::Delete)),
                KeyCode::F(5) => Some(Message::Playlist(PlaylistMessage::SaveSelected)),
                KeyCode::Home => Some(Message::Playlist(PlaylistMessage::ScrollToStart)),
                KeyCode::End => Some(Message::Playlist(PlaylistMessage::ScrollToEnd)),
                KeyCode::Char('h') => Some(Message::ShowHelp),
                KeyCode::Tab => Some(Message::CycleTabs),
                _ => None,
            },

            _ => match key_event.code {
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
                KeyCode::Char('m') => Some(Message::Player(PlayerMessage::ShuffleQueue)),
                KeyCode::Char('h') => Some(Message::ShowHelp),
                KeyCode::Delete => Some(Message::Player(PlayerMessage::ClearQueue)),
                KeyCode::Backspace => {
                    Some(Message::Player(PlayerMessage::RemoveSelectedQueuedTrack))
                }
                KeyCode::Home => Some(Message::Player(PlayerMessage::ScrollToStart)),
                KeyCode::End => Some(Message::Player(PlayerMessage::ScrollToEnd)),
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
            KeyCode::Char('y') => Some(Message::Confirm(Response::Yes)),
            KeyCode::Char('n') => Some(Message::Confirm(Response::No)),
            KeyCode::Enter => Some(Message::Confirm(Response::Yes)),
            KeyCode::Esc => Some(Message::Confirm(Response::No)),
            _ => None,
        },
    }
}
