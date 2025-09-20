use std::path::PathBuf;

use crate::{
    logic::{
        player::{self, Player},
        playlist::{playlist_controller::PlaylistController, playlist_tab_focus::PlaylistTabFocus},
        user_input::InputTarget,
    },
    message::Message,
    model::Model,
    view::terminal::CursorMovementDirection,
};

pub fn create_playlist(model: &mut Model) -> Option<Message> {
    let index = model.playlist_ctl.create_playlist();

    Some(Message::EnterEditMode(
        "Playlist".to_string(),
        InputTarget::PlaylistName(index),
    ))
}

pub fn name_playlist(index: usize, model: &mut Model) -> Option<Message> {
    let current_playlist_names = model.playlist_ctl.get_all_playlist_names();

    if let Some(name) = model.user_input.input_history.pop()
        && let Some(playlist) = model.playlist_ctl.playlist_collection.get_playlist(index)
        && !current_playlist_names.contains(&name)
    {
        playlist.rename(name.as_str());
        return Some(Message::ExitEditMode);
    }

    Some(Message::EnterEditMode(
        "Playlist - Name Taken".to_string(),
        InputTarget::PlaylistName(index),
    ))
}

pub fn add_tracks(playlist_controller: &mut PlaylistController) -> Option<Message> {
    if let Some(selected) = playlist_controller.get_selected_playlist() {
        if let Some(path_vec) = player::choose_multiple_audio_files() {
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

pub fn navigate_playlists(
    direction: CursorMovementDirection,
    playlist_controller: &mut PlaylistController,
) {
    if !matches!(playlist_controller.tab_focus, PlaylistTabFocus::Playlists) {
        return;
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
}

pub fn navigate_tracks(
    direction: CursorMovementDirection,
    playlist_controller: &mut PlaylistController,
) {
    if !matches!(playlist_controller.tab_focus, PlaylistTabFocus::Tracks) {
        return;
    }

    if let Some(selected_playlist) = playlist_controller.get_selected_playlist() {
        match direction {
            CursorMovementDirection::Up => selected_playlist.select_prev_track(),
            CursorMovementDirection::Down => selected_playlist.select_next_track(),
            _ => {}
        }
    }
}

pub fn move_cursor(
    direction: CursorMovementDirection,
    playlist_controller: &mut PlaylistController,
) -> Option<Message> {
    if playlist_controller.playlist_collection.is_empty() {
        return None;
    }

    match direction {
        CursorMovementDirection::Left => {
            playlist_controller.tab_focus = PlaylistTabFocus::Playlists;
        }
        CursorMovementDirection::Right => {
            if let Some(selected_playlist) = playlist_controller.get_selected_playlist()
                && !selected_playlist.is_empty()
            {
                playlist_controller.tab_focus = PlaylistTabFocus::Tracks;
            }
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
