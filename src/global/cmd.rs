use std::{
    sync::{Arc, Mutex, mpsc::Sender},
    time::Duration,
};

use log::debug;
use rust_ffmpeg::FFmpegProcess;

use crate::{
    global::{
        logic::{confirmation::Response, session_state::RunningState, track::FormatConversion},
        message::Message,
    },
    model::Model,
    player::{self, logic::playback_status::PlaybackStatus},
    playlist::cmd::playlist_save_confirm_then_resume,
    user_input::logic::InputMode,
};

pub fn tick(
    model: &mut Model,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    if model.player.sink.is_paused() {
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
    model.confirmation.msg = Some(msg);
    model.confirmation.prompt = prompt;
    model.input_mode = InputMode::Confirmation;

    None
}

pub fn confirmed(answer: Response, model: &mut Model) -> Option<Message> {
    let message = model.confirmation.msg.take();
    model.input_mode = InputMode::default();
    model.confirmation.prompt.clear();
    model.confirmation.response = Some(answer);

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

pub fn update_status_msg(info: String, model: &mut Model) -> Option<Message> {
    model.status_msg = info;

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

pub fn acknowledge_info(model: &mut Model) -> Option<Message> {
    model.input_mode = InputMode::default();

    None
}

pub fn display_info(info: String, model: &mut Model) -> Option<Message> {
    model.info_msg = info;
    model.input_mode = InputMode::Info;

    None
}
