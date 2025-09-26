use std::{
    sync::{Arc, Mutex, mpsc::Sender},
    time::Duration,
};

use log::debug;
use rust_ffmpeg::FFmpegProcess;

use crate::{
    global::{
        logic::{session_state::RunningState, track::FormatConversion},
        message::Message,
        view::scroll,
    },
    model::Model,
    player::{self, logic::playback_status::PlaybackStatus},
    playlist::cmd::playlist_save_confirm_then_resume,
    user_input::logic::InputMode,
};

pub enum Confirmation {
    Yes,
    No,
}

pub fn tick(
    model: &mut Model,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    update_scroll_offsets(model);

    if model.session.state == RunningState::Busy {
        return None;
    }

    if model.player.sink.is_paused() && !model.player.looping {
        model.player.status = PlaybackStatus::Paused;
    } else if !model.player.sink.empty() {
        model.player.status = PlaybackStatus::Playing;
    }

    if model.player.sink.empty() & !model.player.looping {
        model.player.status = PlaybackStatus::Idle;
    }

    if let Some(ref mut current_track) = model.player.current {
        let status = current_track.conversion_status;
        // Update playback status

        // Update playback position
        current_track.sync_pos(&model.player.sink);

        // Reload track when track ends if looped
        // Set position, duration, and status to default if not looped
        if model.player.sink.empty()
            && current_track.duration.saturating_sub(current_track.pos) < Duration::from_secs(3)
        {
            if model.player.looping {
                if let Err(e) = model.player.reload() {
                    log::error!("{}", e);
                }
            } else {
                model.player.status = PlaybackStatus::Idle;
            }
        }

        // Load the next track after current track ends.
        if model.player.status == PlaybackStatus::Idle
            && !model.player.queue.is_empty()
            && !model.player.looping
            && status != FormatConversion::Running
        {
            debug!("Load the next track after current track ends.");
            player::cmd::skip(model, msg_tx, info_tx);
        }

        // Load first track (player.current is None)
    } else if model.player.current.is_none() && !model.player.queue.is_empty() {
        debug!("Load first track (player.current is None)");
        player::cmd::skip(model, msg_tx, info_tx);
    }

    None
}

pub fn ask_for_confirmation(prompt: String, msg: Message, model: &mut Model) -> Option<Message> {
    model.ask_confirmation = Some(msg);
    model.confirmation_prompt = prompt;
    model.input_mode = InputMode::Confirmation;

    None
}

pub fn confirmed(answer: Confirmation, model: &mut Model) -> Option<Message> {
    let message = model.ask_confirmation.take();
    model.input_mode = InputMode::default();
    model.confirmation_prompt.clear();
    model.confirmation = Some(answer);

    message
}

pub fn conversion_started(handle: Arc<Mutex<FFmpegProcess>>, model: &mut Model) -> Option<Message> {
    model.player.ffmpeg_handle = Some(handle);

    Some(Message::SetBusy)
}

pub fn conversion_ended(model: &mut Model) -> Option<Message> {
    model.session.state = RunningState::Running;
    if let Some(ref mut current_track) = model.player.current {
        current_track.conversion_status = FormatConversion::Done;
        model
            .player
            .reload()
            .expect("Error reloading after conversion finished.");
    }

    None
}

pub fn update_info(info: String, model: &mut Model) -> Option<Message> {
    model.info_display = info;

    None
}

pub fn quit(model: &mut Model) -> Option<Message> {
    if let Some(to_resume) =
        playlist_save_confirm_then_resume(Message::Quit, &mut model.playlist_ctl)
    {
        return Some(to_resume);
    }

    model.session.state = RunningState::Done;

    None
}

pub fn set_busy(model: &mut Model) -> Option<Message> {
    model.session.state = RunningState::Busy;

    None
}

pub fn cycle_tabs(model: &mut Model) -> Option<Message> {
    if let Some(to_resume) =
        playlist_save_confirm_then_resume(Message::CycleTabs, &mut model.playlist_ctl)
    {
        return Some(to_resume);
    }

    model.selected_tab.cycle_right();

    None
}

fn update_scroll_offsets(model: &mut Model) {
    if let Some(queue_area_h) = model.queue_view.area_height.take() {
        model.queue_view.scroll_offset = scroll(
            model.player.queue.get_selected(),
            model.queue_view.scroll_offset,
            model.player.queue.get().len(),
            queue_area_h,
        );
    }

    if let Some(playlist_area_h) = model.playlist_view.playlists_area_height.take() {
        model.playlist_view.playlists_scroll_offset = scroll(
            model.playlist_ctl.selected_playlist.unwrap_or(0),
            model.playlist_view.playlists_scroll_offset,
            model.playlist_ctl.playlist_coll.len(),
            playlist_area_h,
        );
    }

    if let Some(playlist_tracks_area_h) = model.playlist_view.tracks_area_height.take()
        && let Some(current_playlist) = model.playlist_ctl.get_selected_playlist() {
            model.playlist_view.tracks_scroll_offset = scroll(
                current_playlist.selected_track.unwrap_or(0),
                model.playlist_view.tracks_scroll_offset,
                current_playlist.len(),
                playlist_tracks_area_h,
            );
        }
}
