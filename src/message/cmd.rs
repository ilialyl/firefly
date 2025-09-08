use std::{
    sync::{Arc, Mutex, mpsc::Sender},
    time::Duration,
};

use rust_ffmpeg::FFmpegProcess;
use tokio::runtime::Runtime;

use crate::{
    logic::{
        playback_status::PlaybackStatus,
        player::{self},
        session_state::RunningState,
        track::FormatConversion,
    },
    message::Message,
    model::Model,
};

pub fn tick(
    model: &mut Model,
    _msg_tx: &Sender<Message>,
    _info_tx: &Sender<String>,
) -> Option<Message> {
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

        // Load first track (player.current is None)
    } else if !model.player.queue.is_empty() {
        return Some(Message::Skip);
    }

    // Load the next track after current track ends.
    if model.player.status == PlaybackStatus::Idle
        && !model.player.queue.is_empty()
        && !model.player.looping
    {
        return Some(Message::Skip);
    }

    None
}

pub fn load_now(
    model: &mut Model,
    _msg_tx: &Sender<Message>,
    _info_tx: &Sender<String>,
) -> Option<Message> {
    if let Some(path) = player::choose_file() {
        model.player.queue.prepend_track(path);
        return Some(Message::Skip);
    }

    None
}

pub fn toggle_play(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    if model.player.status == PlaybackStatus::Playing {
        model.player.sink.pause();
    } else {
        model.player.sink.play();
    }

    None
}

pub fn queue_dir(model: &mut Model) -> Option<Message> {
    if let Some(dir) = player::choose_dir() {
        model.player.queue.enqueue_dir(dir);
    }

    None
}

pub fn queue_files(model: &mut Model) -> Option<Message> {
    if let Some(path_vec) = player::choose_multiple_files() {
        model.player.queue.enqueue_tracks(path_vec);
    }

    None
}

pub fn move_queue_up(model: &mut Model) -> Option<Message> {
    if let Err(e) = model.player.queue.move_selected_up() {
        log::info!("{}", e);
    };

    None
}

pub fn move_queue_down(model: &mut Model) -> Option<Message> {
    if let Err(e) = model.player.queue.move_selected_down() {
        log::info!("{}", e);
    };

    None
}

pub fn rewind(model: &mut Model, _info_tx: &Sender<String>) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    model
        .player
        .rewind(Duration::from_secs(5))
        .expect("Error rewinding.");

    None
}

pub fn seek(model: &mut Model, _info_tx: &Sender<String>) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    if let Some(current_track) = &model.player.current {
        let duration = current_track.duration;
        model
            .player
            .seek(&duration, Duration::from_secs(5))
            .expect("Error seeking.");
    }

    None
}

pub fn skip(
    model: &mut Model,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    if model.player.queue.is_empty() {
        return None;
    }

    if let Some(handle) = model.ffmpeg_handle.take() {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(handle.lock().unwrap().kill()).unwrap();
    }

    log::info!("Trying to load {:?}.", model.player.queue.front());
    model.player.sink.clear();
    model
        .player
        .load_next_track(msg_tx, info_tx)
        .expect("Error loading next track.");

    if let Some(ref mut current_track) = model.player.current {
        if current_track.conversion_status != FormatConversion::Running {
            model.player.reload().expect("Error reloading track.");
        }
    }

    None
}

pub fn toggle_arrange(model: &mut Model) -> Option<Message> {
    model.player.queue.toggle_arrange();

    None
}

pub fn toggle_loop(model: &mut Model) -> Option<Message> {
    model.player.looping = !model.player.looping;

    None
}

pub fn decrease_volume(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    model.player.decrease_volume(0.05);

    None
}

pub fn increase_volume(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    model.player.increase_volume(0.05);

    None
}

pub fn busy(model: &mut Model) -> Option<Message> {
    model.session.state = RunningState::Busy;

    None
}

pub fn conversion_started(handle: Arc<Mutex<FFmpegProcess>>, model: &mut Model) -> Option<Message> {
    model.ffmpeg_handle = Some(handle);

    Some(Message::Busy)
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
