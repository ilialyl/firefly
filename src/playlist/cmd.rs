use std::path::PathBuf;

use crate::{
    global::{
        logic::{
            confirmation::Response,
            files::{choose_dirs, choose_multiple_audio_files, filter_dir_for_audio_files},
        },
        message::Message,
        view_logic::terminal::CursorMovementDirection,
    },
    model::Model,
    playlist::{
        logic::{
            mini_metadata::MiniMetadata, playlist_controller::PlaylistController,
            playlist_tab_focus::PlaylistTabFocus,
        },
        message::PlaylistMessage,
    },
    user_input::{logic::InputTarget, message::UserInputMessage},
};

pub fn playlist_save_confirm_then_resume(
    to_resume: Message,
    playlist_ctl: &mut PlaylistController,
) -> Option<Message> {
    if let Some(selected_playlist) = playlist_ctl.get_selected_playlist()
        && selected_playlist.is_dirty()
    {
        return Some(Message::Playlist(PlaylistMessage::AskToSave(Box::new(
            Some(to_resume),
        ))));
    }

    None
}

pub fn create_playlist(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    if let Some(to_resume) =
        playlist_save_confirm_then_resume(Message::Playlist(PlaylistMessage::Create), playlist_ctl)
    {
        return Some(to_resume);
    }

    let index = playlist_ctl.create_playlist();

    Some(Message::UserInput(UserInputMessage::EnterEditMode(
        "Playlist".to_string(),
        InputTarget::PlaylistName(index),
    )))
}

pub fn name_playlist(index: usize, model: &mut Model) -> Option<Message> {
    let current_playlist_names: Vec<String> = model
        .playlist_ctl
        .get_all_playlist_names()
        .iter()
        .map(|&s| s.to_string())
        .collect();

    if let Some(name) = model.user_input.input_history.pop()
        && let Some(playlist) = model.playlist_ctl.playlist_coll.get_playlist(index)
        && !current_playlist_names.contains(&name)
    {
        playlist.rename(&name);
        return Some(Message::UserInput(UserInputMessage::Exit));
    }

    Some(Message::UserInput(UserInputMessage::EnterEditMode(
        "Playlist - Name Taken".to_string(),
        InputTarget::PlaylistName(index),
    )))
}

pub fn add_tracks(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    if let Some(selected) = playlist_ctl.selected_playlist
        && let Some(path_vec) = choose_multiple_audio_files()
    {
        let new_tracks: Vec<PathBuf> = path_vec.into_iter().filter(|p| p.is_file()).collect();
        new_tracks.iter().for_each(|p| {
            playlist_ctl
                .playlist_coll
                .add_tracks_to_playlist(p, selected)
        });
    }

    None
}

pub fn remove_selected_track(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    if matches!(playlist_ctl.tab_focus, PlaylistTabFocus::Tracks)
        && let Some(playlist) = playlist_ctl.get_selected_playlist()
    {
        playlist.remove_selected();
        if playlist.is_empty() {
            playlist_ctl.tab_focus = PlaylistTabFocus::Playlists;
        }
    }

    None
}

pub fn add_dir(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    if let Some(selected_playlist_idx) = playlist_ctl.selected_playlist
        && let Some(dir_paths) = choose_dirs()
    {
        for dir in dir_paths {
            match filter_dir_for_audio_files(&dir) {
                Ok(vec_path) => {
                    let new_tracks: Vec<PathBuf> =
                        vec_path.into_iter().filter(|p| p.is_file()).collect();
                    new_tracks.into_iter().for_each(|t| {
                        playlist_ctl
                            .playlist_coll
                            .add_tracks_to_playlist(t.as_path(), selected_playlist_idx)
                    });
                }
                Err(e) => log::error!("{}", e),
            }
        }
    }

    None
}

pub fn send_to_player(model: &mut Model) -> Option<Message> {
    match model.playlist_ctl.tab_focus {
        PlaylistTabFocus::Playlists => {
            if let Some(selected) = model.playlist_ctl.get_selected_playlist() {
                if let Err(e) = model
                    .queue
                    .tx
                    .send(selected.tracks.iter().map(|p| p.path.clone()).collect())
                {
                    log::error!("Error sending Path Vec to queue processing worker: {e}");
                };

                Some(Message::DisplayInfoMsg(
                    "Sent Playlist to Player".to_string(),
                ))
            } else {
                None
            }
        }
        PlaylistTabFocus::Tracks => {
            if let Some(playlist) = model.playlist_ctl.get_selected_playlist()
                && let Some(index) = playlist.selected_track
                && let Some(track) = playlist.tracks.get(index)
            {
                if let Err(e) = model.queue.tx.send(vec![track.path.clone()]) {
                    log::error!("Error sending Path Vec to queue processing worker: {e}");
                };
                Some(Message::DisplayInfoMsg("Sent Track to Player".to_string()))
            } else {
                None
            }
        }
    }
}

pub fn navigate_playlists(
    direction: CursorMovementDirection,
    playlist_ctl: &mut PlaylistController,
) -> Option<Message> {
    if !matches!(playlist_ctl.tab_focus, PlaylistTabFocus::Playlists) {
        return None;
    }

    match direction {
        CursorMovementDirection::Up => {
            if let Some(to_resume) = playlist_save_confirm_then_resume(
                Message::Playlist(PlaylistMessage::MoveCursor(direction)),
                playlist_ctl,
            ) {
                return Some(to_resume);
            }

            playlist_ctl.prev_playlist();
        }
        CursorMovementDirection::Down => {
            if let Some(to_resume) = playlist_save_confirm_then_resume(
                Message::Playlist(PlaylistMessage::MoveCursor(direction)),
                playlist_ctl,
            ) {
                return Some(to_resume);
            }
            playlist_ctl.next_playlist();
        }
        _ => {}
    }

    None
}

pub fn navigate_tracks(direction: CursorMovementDirection, playlist_ctl: &mut PlaylistController) {
    if !matches!(playlist_ctl.tab_focus, PlaylistTabFocus::Tracks) {
        return;
    }

    let arrange_mode = playlist_ctl.arrange_mode;

    if let Some(selected_playlist) = playlist_ctl.get_selected_playlist() {
        match direction {
            CursorMovementDirection::Up => {
                selected_playlist.select_prev_track(arrange_mode);
            }
            CursorMovementDirection::Down => {
                selected_playlist.select_next_track(arrange_mode);
            }
            _ => {}
        }
    }
}

pub fn move_cursor(
    direction: CursorMovementDirection,
    playlist_ctl: &mut PlaylistController,
) -> Option<Message> {
    if playlist_ctl.playlist_coll.is_empty() {
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
            PlaylistTabFocus::Playlists => return navigate_playlists(direction, playlist_ctl),
            PlaylistTabFocus::Tracks => navigate_tracks(direction, playlist_ctl),
        },
    }

    None
}

pub fn delete_playlist(
    confirmation: &mut Option<Response>,
    playlist_ctl: &mut PlaylistController,
) -> Option<Message> {
    if playlist_ctl.get_selected_playlist().is_some() {
        if let Some(confirm) = confirmation.take() {
            match confirm {
                Response::Yes => {
                    playlist_ctl.delete_selected_playlist();
                }
                Response::No => {}
            }
        } else {
            return Some(Message::AskConfirmation(
                "Delete?".to_string(),
                Box::new(Message::Playlist(PlaylistMessage::Delete)),
            ));
        }
    }

    None
}

pub fn rename_playlist(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    playlist_ctl.selected_playlist.map(|index| {
        Message::UserInput(UserInputMessage::EnterEditMode(
            "Playlist Rename".to_string(),
            InputTarget::PlaylistName(index),
        ))
    })
}

pub fn save_selected_playlist(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    playlist_ctl
        .save_selected_to_file()
        .expect("Error saving selected playlist to file");

    Some(Message::DisplayInfoMsg("Saved successfully".to_string()))
}

pub fn load_playlists(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    playlist_ctl
        .load_playlists()
        .expect("Error loading playlists from files.");

    None
}

pub fn toggle_arrange(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    playlist_ctl.arrange_mode = !playlist_ctl.arrange_mode;

    None
}

pub fn ask_to_save(
    confirmation: &mut Option<Response>,
    then_call: Option<Message>,
    playlist_ctl: &mut PlaylistController,
) -> Option<Message> {
    if let Some(current_playlist) = playlist_ctl.get_selected_playlist()
        && current_playlist.is_dirty()
    {
        if let Some(confirm) = confirmation.take() {
            match confirm {
                Response::Yes => {
                    playlist_ctl
                        .save_selected_to_file()
                        .expect("Error saving current playlist to file.");

                    return then_call;
                }
                Response::No => {
                    current_playlist
                        .reload_from_file()
                        .expect("Error restoring file to the state before modification.");
                    return then_call;
                }
            }
        } else {
            return Some(Message::AskConfirmation(
                "Save? (discard if not)".to_string(),
                Box::new(Message::Playlist(PlaylistMessage::AskToSave(Box::new(
                    then_call,
                )))),
            ));
        }
    }

    None
}

pub fn scroll_to_end(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    match playlist_ctl.tab_focus {
        PlaylistTabFocus::Playlists => {
            if !playlist_ctl.playlist_coll.is_empty() {
                playlist_ctl.selected_playlist = Some(playlist_ctl.playlist_coll.len() - 1);
            }
        }
        PlaylistTabFocus::Tracks => {
            if let Some(selected_playlist) = playlist_ctl.get_selected_playlist()
                && !selected_playlist.is_empty()
            {
                selected_playlist.selected_track = Some(selected_playlist.len() - 1);
            }
        }
    }

    None
}

pub fn scroll_to_start(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    match playlist_ctl.tab_focus {
        PlaylistTabFocus::Playlists => {
            if !playlist_ctl.playlist_coll.is_empty() {
                playlist_ctl.selected_playlist = Some(0);
            }
        }
        PlaylistTabFocus::Tracks => {
            if let Some(selected_playlist) = playlist_ctl.get_selected_playlist()
                && !selected_playlist.is_empty()
            {
                selected_playlist.selected_track = Some(0);
            }
        }
    }

    None
}

pub fn append_metadata(
    index: usize,
    mini_metadata: MiniMetadata,
    playlist_ctl: &mut PlaylistController,
) -> Option<Message> {
    if let Some(playlist) = playlist_ctl.playlist_coll.get_playlist(index) {
        for track in playlist.tracks.iter_mut() {
            if track.metadata.is_none() {
                track.metadata = Some(mini_metadata);
                return None;
            }
        }
    }

    None
}
