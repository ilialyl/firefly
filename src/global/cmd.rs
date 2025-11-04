use std::{
    io::Cursor,
    sync::{Arc, mpsc::Sender},
    thread,
    time::Duration,
};

use image::ImageReader;
use lofty::picture::Picture;
use log::debug;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};

use crate::{
    global::{
        logic::{confirmation::Response, image::crop_to_square, session_state::RunningState},
        message::Message,
    },
    model::Model,
    player::{
        self,
        logic::{
            format_conversion::FormatConversion, playback_status::PlaybackStatus, track::Track,
        },
    },
    playlist::cmd::playlist_save_confirm_then_resume,
    user_input::logic::InputMode,
};

pub async fn tick(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::RunningFFmpeg {
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

    if let Some(current_track) = model.player.current.as_mut() {
        // Update playback status
        let status = current_track.conversion_status;

        // Update playback position
        current_track.sync_pos_from_sink(&model.player.sink);

        // Create Protocol if the track has cover art, if not already.
        if !current_track.started_decoding
            && let Some(picture) = current_track.picture.as_mut()
        {
            current_track.started_decoding = true;
            create_protocol(
                picture,
                current_track.id,
                model.picker.clone(),
                &model.senders.msg,
                &model.senders.info,
            );
        }

        // Convert file format to FLAC if current format is not supported, if not already.
        if current_track.conversion_status == FormatConversion::Idle {
            current_track.conversion_status = FormatConversion::Running;
            Track::convert_format(
                &current_track.real_path,
                &current_track.temp_path,
                &model.senders.async_msg,
                &model.senders.info,
            )
            .await;
        }

        // Reload track when track ends if looped
        // Set position, duration, and status to default if not looped
        if model.player.sink.empty()
            && let Some(dur) = current_track.duration
            && dur.saturating_sub(current_track.pos) < Duration::from_secs(3)
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
            && !model.queue.is_empty()
            && !model.player.looping
            && (status == FormatConversion::Done || status == FormatConversion::Unnecessary)
        {
            debug!("Load the next track after current track ends.");
            player::cmd::skip(model).await;
        }

        // Load first track if no current track and there is something in the queue.
    } else if model.player.current.is_none() && !model.queue.is_empty() {
        debug!("Load first track (player.current is None)");
        player::cmd::skip(model).await;
    }

    None
}

pub fn ask_for_confirmation(prompt: String, msg: Message, model: &mut Model) -> Option<Message> {
    model.user_confirmation.msg = Some(msg);
    model.user_confirmation.prompt = prompt;
    model.input_mode = InputMode::Confirmation;

    None
}

pub fn confirmed(answer: Response, model: &mut Model) -> Option<Message> {
    let message = model.user_confirmation.msg.take();
    model.input_mode = InputMode::default();
    model.user_confirmation.prompt.clear();
    model.user_confirmation.response = Some(answer);

    message
}

pub fn update_info_msg(info: String, model: &mut Model) -> Option<Message> {
    model.info_msg = info;
    debug!("Updated info message to {}", model.info_msg);
    debug!("Info message length: {}", model.info_msg.len());

    None
}

pub fn quit(model: &mut Model) -> Option<Message> {
    if let Some(to_resume) =
        playlist_save_confirm_then_resume(Message::Quit, &mut model.playlist_ctl)
    {
        return Some(to_resume);
    }

    model.session.state = RunningState::Exit;

    None
}

pub fn cycle_tabs(model: &mut Model) -> Option<Message> {
    if let Some(to_resume) =
        playlist_save_confirm_then_resume(Message::CycleTabs, &mut model.playlist_ctl)
    {
        return Some(to_resume);
    }

    model.focused_view_area.cycle_right();

    None
}

pub fn acknowledge_info(model: &mut Model) -> Option<Message> {
    model.input_mode = InputMode::default();

    None
}

pub fn display_info_msg(info: String, model: &mut Model) -> Option<Message> {
    if let Err(e) = model.senders.info.send(info.clone()) {
        log::error!("Error sending info to display: {e}");
    }

    let cloned_tx = model.senders.info.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        debug!("Clearing info message.");
        if let Err(e) = cloned_tx.send(String::new()) {
            log::error!("Error clearing info message: {e}");
        }
    });

    None
}

pub fn set_track_protocol(
    protocol: StatefulProtocol,
    id: u32,
    model: &mut Model,
) -> Option<Message> {
    if let Some(current_track) = model.player.current.as_mut()
        && id == current_track.id
    {
        log::debug!("Setting protocol...");
        current_track.protocol = Some(protocol);
        if let Err(e) = model.senders.info.send(String::new()) {
            log::error!("{e}");
        };
    }

    None
}

pub fn create_protocol(
    picture: &Picture,
    id: u32,
    picker: Arc<Picker>,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) {
    if let Err(e) = info_tx.send("Loading Cover Art...".to_string()) {
        log::error!("{e}");
    };

    log::debug!("Creating protocol...");
    let picture_data = picture.data().to_vec();
    let msg_tx = msg_tx.clone();

    thread::spawn(move || {
        if let Some(dyn_img) = ImageReader::new(Cursor::new(&picture_data))
            .with_guessed_format()
            .ok()
            .and_then(|r| r.decode().ok())
        {
            log::debug!("Created Dynamic Image.");
            let protocol = picker.new_resize_protocol(crop_to_square(dyn_img));
            log::debug!("Cropped to square.");
            if let Err(e) = msg_tx.send(Message::ProtocolCreated(protocol, id)) {
                log::error!("Error sending Protocol back to main thread: {e}");
            }
        }
    });
}

pub fn toggle_show_help(model: &mut Model) -> Option<Message> {
    model.show_help = !model.show_help;

    None
}
