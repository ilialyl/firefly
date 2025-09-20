use std::path::PathBuf;

use crate::{
    logic::{
        player::{self, Player},
        playlist::{playlist_controller::PlaylistController, playlist_tab_focus::PlaylistTabFocus},
        user_input::InputTarget,
    },
    message::{Message, PlaylistMessage, UserInputMessage, cmd::Confirmation},
    model::Model,
    view::terminal::CursorMovementDirection,
};

pub fn create_playlist(model: &mut Model) -> Option<Message> {
    let index = model.playlist_ctl.create_playlist();

    Some(Message::UserInput(UserInputMessage::EnterEditMode(
        "Playlist".to_string(),
        InputTarget::PlaylistName(index),
    )))
}

pub fn name_playlist(index: usize, model: &mut Model) -> Option<Message> {
    let current_playlist_names = model.playlist_ctl.get_all_playlist_names();

    if let Some(name) = model.user_input.input_history.pop()
        && let Some(playlist) = model.playlist_ctl.playlist_collection.get_playlist(index)
        && !current_playlist_names.contains(&name)
    {
        playlist.rename(name.as_str());
        return Some(Message::UserInput(UserInputMessage::Exit));
    }

    Some(Message::UserInput(UserInputMessage::EnterEditMode(
        "Playlist - Name Taken".to_string(),
        InputTarget::PlaylistName(index),
    )))
}

pub fn add_tracks(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    if let Some(selected) = playlist_ctl.get_selected_playlist() {
        if let Some(path_vec) = player::choose_multiple_audio_files() {
            let playlist = selected;

            let new_tracks: Vec<PathBuf> = path_vec.into_iter().filter(|p| p.is_file()).collect();
            new_tracks.iter().for_each(|p| playlist.add(p));
        }
    }
    None
}

pub fn send_to_player(
    playlist_ctl: &mut PlaylistController,
    player: &mut Player,
) -> Option<Message> {
    if let Some(selected) = playlist_ctl.get_selected_playlist() {
        player
            .queue
            .enqueue_tracks(selected.tracks.iter().map(|e| e.to_path_buf()).collect());
    }

    None
}

pub fn navigate_playlists(
    direction: CursorMovementDirection,
    playlist_ctl: &mut PlaylistController,
) {
    if !matches!(playlist_ctl.tab_focus, PlaylistTabFocus::Playlists) {
        return;
    }

    match direction {
        CursorMovementDirection::Up => {
            playlist_ctl.prev_playlist();
        }
        CursorMovementDirection::Down => {
            playlist_ctl.next_playlist();
        }
        _ => {}
    }
}

pub fn navigate_tracks(direction: CursorMovementDirection, playlist_ctl: &mut PlaylistController) {
    if !matches!(playlist_ctl.tab_focus, PlaylistTabFocus::Tracks) {
        return;
    }

    if let Some(selected_playlist) = playlist_ctl.get_selected_playlist() {
        match direction {
            CursorMovementDirection::Up => selected_playlist.select_prev_track(),
            CursorMovementDirection::Down => selected_playlist.select_next_track(),
            _ => {}
        }
    }
}

pub fn move_cursor(
    direction: CursorMovementDirection,
    playlist_ctl: &mut PlaylistController,
) -> Option<Message> {
    if playlist_ctl.playlist_collection.is_empty() {
        return None;
    }

    match direction {
        CursorMovementDirection::Left => {
            playlist_ctl.tab_focus = PlaylistTabFocus::Playlists;
        }
        CursorMovementDirection::Right => {
            if let Some(selected_playlist) = playlist_ctl.get_selected_playlist()
                && !selected_playlist.is_empty()
            {
                playlist_ctl.tab_focus = PlaylistTabFocus::Tracks;
            }
        }
        _ => match playlist_ctl.tab_focus {
            PlaylistTabFocus::Playlists => {
                navigate_playlists(direction, playlist_ctl);
            }
            PlaylistTabFocus::Tracks => {
                navigate_tracks(direction, playlist_ctl);
            }
        },
    }

    None
}

pub fn delete_playlist(
    playlist_ctl: &mut PlaylistController,
    confirmation: Confirmation,
) -> Option<Message> {
    if let Some(_) = playlist_ctl.get_selected_playlist() {
        match confirmation {
            Confirmation::Yes => {
                playlist_ctl.delete_selected_playlist();
            }
            Confirmation::No => {
                return Some(Message::AskConfirmation(Box::new(Message::Playlist(
                    PlaylistMessage::Delete(Confirmation::Yes),
                ))));
            }
        }
    }

    None
}

pub fn rename_playlist(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    if let Some(index) = playlist_ctl.selected_playlist {
        Some(Message::UserInput(UserInputMessage::EnterEditMode(
            "Playlist Rename".to_string(),
            InputTarget::PlaylistName(index),
        )))
    } else {
        None
    }
}

pub fn save_selected_playlist(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    playlist_ctl
        .save_selected_to_file()
        .expect("Error saving selected playlist to file");

    None
}

pub fn load_playlists(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    playlist_ctl
        .load_playlists()
        .expect("Error loading playlists from files.");

    None
}
