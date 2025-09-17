use std::path::PathBuf;

use crate::{
    logic::{
        player::{self, Player},
        playlist::{playlist_controller::PlaylistController, playlist_tab_focus::PlaylistTabFocus},
    },
    message::{Message, cursor_movement::CursorMovementDirection},
};

pub fn create_playlist(playlist_controller: &mut PlaylistController) -> Option<Message> {
    playlist_controller.create_playlist();

    None
}

pub fn add_tracks(playlist_controller: &mut PlaylistController) -> Option<Message> {
    if let Some(selected) = playlist_controller.get_selected_playlist() {
        if let Some(path_vec) = player::choose_multiple_files() {
            let playlist = selected;

            let new_tracks: Vec<PathBuf> = path_vec.into_iter().filter(|p| p.is_file()).collect();
            new_tracks.iter().for_each(|p| playlist.add(p));
        }
    }
    None
}

pub fn send_to_player(
    playlist_controller: &mut PlaylistController,
    player: &mut Player,
) -> Option<Message> {
    if let Some(selected) = playlist_controller.get_selected_playlist() {
        player
            .queue
            .enqueue_tracks(selected.tracks.iter().map(|e| e.to_path_buf()).collect());
    }

    None
}

pub fn cycle_playlist_focus(
    direction: CursorMovementDirection,
    playlist_controller: &mut PlaylistController,
) -> Option<Message> {
    match direction {
        CursorMovementDirection::Left => playlist_controller.tab_focus.cycle_focus_left(),
        CursorMovementDirection::Right => playlist_controller.tab_focus.cycle_focus_right(),
        _ => {}
    }

    None
}

pub fn navigate_playlists(
    direction: CursorMovementDirection,
    playlist_controller: &mut PlaylistController,
) -> Option<Message> {
    if !matches!(playlist_controller.tab_focus, PlaylistTabFocus::Playlists) {
        return None;
    }

    match direction {
        CursorMovementDirection::Up => {
            playlist_controller.prev_playlist();
        }
        CursorMovementDirection::Down => {
            playlist_controller.next_playlist();
        }
        _ => {}
    }

    None
}

pub fn navigate_tracks(
    direction: CursorMovementDirection,
    playlist_controller: &mut PlaylistController,
) -> Option<Message> {
    if !matches!(playlist_controller.tab_focus, PlaylistTabFocus::Tracks) {
        return None;
    }

    if let Some(selected_playlist) = playlist_controller.get_selected_playlist() {
        match direction {
            CursorMovementDirection::Up => selected_playlist.select_prev_track(),
            CursorMovementDirection::Down => selected_playlist.select_next_track(),
            _ => {}
        }
    }

    None
}

pub fn move_cursor(
    direction: CursorMovementDirection,
    playlist_controller: &mut PlaylistController,
) -> Option<Message> {
    match direction {
        CursorMovementDirection::Left => {
            playlist_controller.tab_focus = PlaylistTabFocus::Playlists;
        }
        CursorMovementDirection::Right => {
            playlist_controller.tab_focus = PlaylistTabFocus::Tracks;
        }
        _ => match playlist_controller.tab_focus {
            PlaylistTabFocus::Playlists => {
                navigate_playlists(direction, playlist_controller);
            }
            PlaylistTabFocus::Tracks => {
                navigate_tracks(direction, playlist_controller);
            }
        },
    }

    None
}
