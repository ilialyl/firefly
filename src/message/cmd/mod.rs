use std::{
    sync::{Arc, Mutex, mpsc::Sender},
    time::Duration,
};

use log::debug;
use rust_ffmpeg::FFmpegProcess;

use crate::{
    logic::{
        player::playback_status::PlaybackStatus, session_state::RunningState,
        track::FormatConversion, user_input::InputMode,
    },
    message::{Message, cmd::player_cmd::skip},
    model::Model,
    view::player_tab::queue_view,
};

pub mod player_cmd;
pub mod playlist_cmd;
pub mod userinput_cmd;

pub enum Confirmation {
    Yes,
    No,
}

pub fn tick(
    model: &mut Model,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    // Scroll queue view
    if let Some(area_height) = model.queue_view.area_height.take() {
        model.queue_view.scroll_offset = queue_view::scroll(
            model.player.queue.get_selected(),
            model.queue_view.scroll_offset,
            model.player.queue.get().len(),
            area_height,
        );
    }

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
            skip(model, msg_tx, info_tx);
        }

        // Load first track (player.current is None)
    } else if model.player.current.is_none() && !model.player.queue.is_empty() {
        debug!("Load first track (player.current is None)");
        skip(model, msg_tx, info_tx);
    }

    None
}

pub fn ask_for_confirmation(msg: Message, model: &mut Model) -> Option<Message> {
    model.ask_confirmation = Some(msg);
    model.input_mode = InputMode::Confirmation;

    None
}

pub fn confirmed(answer: Confirmation, model: &mut Model) -> Option<Message> {
    let message = model.ask_confirmation.take();
    model.input_mode = InputMode::default();

    match answer {
        Confirmation::Yes => message,
        Confirmation::No => None,
    }
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
    model.session.state = RunningState::Done;

    None
}

pub fn set_busy(model: &mut Model) -> Option<Message> {
    model.session.state = RunningState::Busy;

    None
}

pub fn cycle_tabs(model: &mut Model) -> Option<Message> {
    model.selected_tab.cycle_right();

    None
}
