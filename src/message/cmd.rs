use std::{
    sync::{Arc, Mutex, mpsc::Sender},
    thread,
    time::Duration,
};

use crate::{
    logic::{
        playback_state::PlaybackStatus,
        player::{self},
        session_state::RunningState,
    },
    message::Message,
    model::Model,
};

use rust_ffmpeg::FFmpegProcess;
use tokio::runtime::Runtime;

pub fn tick(
    model: &mut Model,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    // Update playback status
    if model.playback.sink.is_paused() {
        model.playback.status = PlaybackStatus::Paused;
    } else if model.playback.current.path.is_some() && !model.playback.sink.empty() {
        model.playback.status = PlaybackStatus::Playing;
    } else if model.playback.sink.empty() {
        model.playback.status = PlaybackStatus::Idle;
    }

    // Update playback position
    model.playback.current.pos = Some(model.playback.sink.get_pos());

    // Reload track when track ends if looped
    // Set position, duration, and status to default if not looped
    if let (Some(path), Some(dur), Some(pos)) = (
        model.playback.current.path.clone(),
        model.playback.current.duration,
        model.playback.current.pos,
    ) && model.playback.sink.empty()
        && dur.saturating_sub(pos) < Duration::from_secs(3)
    {
        if model.playback.looping {
            if let Err(e) = player::load_track(&path, &mut model.playback) {
                log::error!("{}", e);
            }
        } else {
            model.playback.current.pos = None;
            model.playback.current.duration = None;
            model.playback.status = PlaybackStatus::Idle;
        }
    }

    // If playback is idle and queue is not empty,
    // Try playing the next track.
    if model.playback.status == PlaybackStatus::Idle && !model.playback.queue.is_empty() {
        log::info!("[TICK] Trying to load {:?}.", model.playback.queue.front());
        if let Err(e) = player::try_next_track(&mut model.playback, msg_tx, info_tx) {
            log::error!("{}", e);
        };
    }

    None
}

pub fn load_now(
    model: &mut Model,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    if let Some(path) = player::choose_file() {
        if let Some(handle) = model.ffmpeg_handle.take() {
            let runtime = Runtime::new().unwrap();
            runtime.block_on(handle.lock().unwrap().kill()).unwrap();
        }
        if let Err(e) = player::load_now(path, &mut model.playback, msg_tx, info_tx) {
            log::error!("{}", e);
        }
    }

    None
}

pub fn toggle_play(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    if model.playback.status == PlaybackStatus::Playing {
        model.playback.sink.pause();
    } else {
        model.playback.sink.play();
    }

    None
}

pub fn queue_dir(model: &mut Model) -> Option<Message> {
    if let Some(dir) = player::choose_dir() {
        model.playback.queue.enqueue_dir(dir);
    }

    None
}

pub fn queue_file(model: &mut Model) -> Option<Message> {
    if let Some(path_vec) = player::choose_multiple_files() {
        model.playback.queue.enqueue_tracks(path_vec);
    }

    None
}

pub fn queue_up(model: &mut Model) -> Option<Message> {
    if let Err(e) = model.playback.queue.move_selected_up() {
        log::info!("{}", e);
    };

    None
}

pub fn queue_down(model: &mut Model) -> Option<Message> {
    if let Err(e) = model.playback.queue.move_selected_down() {
        log::info!("{}", e);
    };

    None
}

pub fn rewind(model: &mut Model, info_tx: &Sender<String>) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    if let Some(track) = model.playback.current.path.clone()
        && model.playback.current.path.is_some()
    {
        info_tx.send("Rewinding...".to_string()).unwrap();
        if let Err(e) = player::rewind(Duration::from_secs(5), &model.playback) {
            let cloned_info_tx = info_tx.clone();

            log::error!("{}", e);
            thread::spawn(move || {
                cloned_info_tx.send(e.to_string()).unwrap();
                thread::sleep(Duration::from_secs(2));
                cloned_info_tx.send("".to_string()).unwrap();
            });
        };
        model.playback.current.duration =
            player::read_track_duration(&track, &mut model.playback).ok();
        info_tx.send("".to_string()).unwrap();
    }

    None
}

pub fn seek(model: &mut Model, info_tx: &Sender<String>) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    if let Some(track_dur) = &model.playback.current.duration
        && model.playback.current.path.is_some()
        && let Err(e) = player::seek(&mut model.playback.sink, track_dur, Duration::from_secs(5))
    {
        let cloned_info_tx = info_tx.clone();

        log::error!("{}", e);
        thread::spawn(move || {
            cloned_info_tx.send(e.to_string()).unwrap();
            thread::sleep(Duration::from_secs(2));
            cloned_info_tx.send("".to_string()).unwrap();
        });
    }

    None
}

pub fn skip(
    model: &mut Model,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    model.session.state = RunningState::Running;

    if model.playback.queue.is_empty() {
        return None;
    }
    log::info!("Skipping...");

    if let Some(handle) = model.ffmpeg_handle.take() {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(handle.lock().unwrap().kill()).unwrap();
    }

    if let Err(e) = player::try_next_track(&mut model.playback, msg_tx, info_tx) {
        log::error!("{}", e);
    };

    None
}

pub fn toggle_arrange(model: &mut Model) -> Option<Message> {
    model.playback.queue.toggle_arrange();

    None
}

pub fn toggle_loop(model: &mut Model) -> Option<Message> {
    model.playback.looping = !model.playback.looping;

    None
}

pub fn volume_down(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    player::decrease_volume(&mut model.playback.sink, 0.05);

    None
}

pub fn volume_up(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::Busy {
        return None;
    }

    player::increase_volume(&mut model.playback.sink, 0.05);

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
    if let Some(path) = model.playback.current.path.clone()
        && let Err(e) = player::play_next_track(&path, &mut model.playback)
    {
        log::error!("{}", e);
    };

    None
}

pub fn quit(model: &mut Model) -> Option<Message> {
    model.session.state = RunningState::Done;

    None
}
