use std::path::PathBuf;

use crate::{
    global::{
        cmd::Confirmation,
        logic::{
            files::{choose_dir, choose_multiple_audio_files, filter_dir_for_audio_files},
            terminal::CursorMovementDirection,
        },
        message::{Message, PlaylistMessage, UserInputMessage},
    },
    model::Model,
    player::logic::Player,
    playlist::logic::{
        playlist_controller::PlaylistController, playlist_tab_focus::PlaylistTabFocus,
    },
    user_input::logic::InputTarget,
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

pub fn create_playlist(model: &mut Model) -> Option<Message> {
    if let Some(to_resume) = playlist_save_confirm_then_resume(
        Message::Playlist(PlaylistMessage::Create),
        &mut model.playlist_ctl,
    ) {
        return Some(to_resume);
    }

    let index = model.playlist_ctl.create_playlist();

    Some(Message::UserInput(UserInputMessage::EnterEditMode(
        "Playlist".to_string(),
        InputTarget::PlaylistName(index),
    )))
}

pub fn name_playlist(index: usize, model: &mut Model) -> Option<Message> {
    let current_playlist_names = model.playlist_ctl.get_all_playlist_names();

    if let Some(name) = model.user_input.input_history.pop()
        && let Some(playlist) = model.playlist_ctl.playlist_coll.get_playlist(index)
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
        if let Some(path_vec) = choose_multiple_audio_files() {
            let playlist = selected;

            let new_tracks: Vec<PathBuf> = path_vec.into_iter().filter(|p| p.is_file()).collect();
            new_tracks.iter().for_each(|p| playlist.add(p));
        }
    }
    None
}

pub fn remove_selected_track(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    if let Some(playlist) = playlist_ctl.get_selected_playlist() {
        playlist.remove_selected();
        if playlist.is_empty() {
            playlist_ctl.tab_focus = PlaylistTabFocus::Playlists;
        }
    }

    None
}

pub fn add_dir(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    if let Some(selected) = playlist_ctl.get_selected_playlist() {
        if let Some(dir_path) = choose_dir() {
            let playlist = selected;
            match filter_dir_for_audio_files(dir_path) {
                Ok(vec_path) => {
                    let new_tracks: Vec<PathBuf> =
                        vec_path.into_iter().filter(|p| p.is_file()).collect();
                    new_tracks.iter().for_each(|p| playlist.add(p));
                }
                Err(e) => log::error!("{}", e),
            }
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
    model: &mut Model,
) -> Option<Message> {
    if !matches!(model.playlist_ctl.tab_focus, PlaylistTabFocus::Playlists) {
        return None;
    }

    match direction {
        CursorMovementDirection::Up => {
            if let Some(to_resume) = playlist_save_confirm_then_resume(
                Message::Playlist(PlaylistMessage::MoveCursor(direction)),
                &mut model.playlist_ctl,
            ) {
                return Some(to_resume);
            }

            model.playlist_ctl.prev_playlist();
        }
        CursorMovementDirection::Down => {
            if let Some(to_resume) = playlist_save_confirm_then_resume(
                Message::Playlist(PlaylistMessage::MoveCursor(direction)),
                &mut model.playlist_ctl,
            ) {
                return Some(to_resume);
            }
            model.playlist_ctl.next_playlist();
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

pub fn move_cursor(direction: CursorMovementDirection, model: &mut Model) -> Option<Message> {
    if model.playlist_ctl.playlist_coll.is_empty() {
        return None;
    }

    match direction {
        CursorMovementDirection::Left => {
            model.playlist_ctl.tab_focus = PlaylistTabFocus::Playlists;
        }
        CursorMovementDirection::Right => {
            if let Some(selected_playlist) = model.playlist_ctl.get_selected_playlist()
                && !selected_playlist.is_empty()
            {
                model.playlist_ctl.tab_focus = PlaylistTabFocus::Tracks;
            }
        }
        _ => match model.playlist_ctl.tab_focus {
            PlaylistTabFocus::Playlists => return navigate_playlists(direction, model),
            PlaylistTabFocus::Tracks => navigate_tracks(direction, &mut model.playlist_ctl),
        },
    }

    None
}

pub fn delete_playlist(
    confirmation: &mut Option<Confirmation>,
    playlist_ctl: &mut PlaylistController,
) -> Option<Message> {
    if let Some(_) = playlist_ctl.get_selected_playlist() {
        if let Some(confirm) = confirmation.take() {
            match confirm {
                Confirmation::Yes => {
                    playlist_ctl.delete_selected_playlist();
                }
                Confirmation::No => {}
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

pub fn toggle_arrange(playlist_ctl: &mut PlaylistController) -> Option<Message> {
    playlist_ctl.arrange_mode = !playlist_ctl.arrange_mode;

    None
}

pub fn ask_to_save(
    confirmation: &mut Option<Confirmation>,
    then_call: Option<Message>,
    playlist_ctl: &mut PlaylistController,
) -> Option<Message> {
    if let Some(current_playlist) = playlist_ctl.get_selected_playlist() {
        if current_playlist.is_dirty() {
            if let Some(confirm) = confirmation.take() {
                match confirm {
                    Confirmation::Yes => {
                        playlist_ctl
                            .save_selected_to_file()
                            .expect("Error saving current playlist to file.");

                        return then_call;
                    }
                    Confirmation::No => {
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
    }

    None
}
