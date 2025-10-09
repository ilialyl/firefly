use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::{sync::mpsc::Sender, thread};

use tokio::runtime::Runtime;

use crate::global::logic::files::dir_to_audio_paths;
use crate::{
    global::{
        logic::{
            files::{choose_audio_file, choose_dirs, choose_multiple_audio_files},
            mini_track::MiniTrack,
            session_state::RunningState,
        },
        message::{Message, PlayerMessage},
    },
    model::Model,
    player::logic::{Player, playback_status::PlaybackStatus, track::FormatConversion},
};

pub fn load_now(player: &mut Player) -> Option<Message> {
    if let Some(path) = choose_audio_file() {
        player.queue.prepend_track(&path);
        return Some(Message::Player(PlayerMessage::Skip));
    }

    None
}

pub fn toggle_play(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::RunningFFmpeg {
        return None;
    }

    if model.player.status == PlaybackStatus::Playing {
        model.player.sink.pause();
    } else {
        model.player.sink.play();
    }

    None
}

pub fn queue_dir(msg_tx: &Sender<Message>, model: &mut Model) -> Option<Message> {
    if let Some(dirs) = choose_dirs() {
        let mut path_vec = Vec::<PathBuf>::new();
        for dir in dirs {
            path_vec.extend(dir_to_audio_paths(&dir));
        }

        queue_files_multithreaded(path_vec, msg_tx, model);
    }

    None
}

pub fn queue_files(msg_tx: &Sender<Message>, model: &mut Model) -> Option<Message> {
    if let Some(path_vec) = choose_multiple_audio_files() {
        queue_files_multithreaded(path_vec, msg_tx, model)
    } else {
        None
    }
}

pub fn queue_files_multithreaded(
    path_vec: Vec<PathBuf>,
    msg_tx: &Sender<Message>,
    model: &mut Model,
) -> Option<Message> {
    let msg_tx = msg_tx.clone();
    let queuing = model.queuing.clone();
    thread::spawn(move || {
        queuing.store(true, Ordering::Relaxed);
        path_vec.iter().for_each(|p| {
            let mini_track = MiniTrack::new(&p);
            if let Err(e) =
                msg_tx.send(Message::Player(PlayerMessage::CreatedMiniTrack(mini_track)))
            {
                log::error!("Error sending MiniTrack back to main thread: {e}")
            };
        });
        queuing.store(false, Ordering::Relaxed);
    });

    None
}

pub fn queue_mini_track(mini_track: MiniTrack, player: &mut Player) -> Option<Message> {
    player.queue.enqueue_mini_track(mini_track);

    None
}

pub fn move_queue_up(player: &mut Player) -> Option<Message> {
    if let Err(e) = player.queue.move_selected_up() {
        log::error!("{}", e);
    };

    None
}

pub fn move_queue_down(player: &mut Player) -> Option<Message> {
    if let Err(e) = player.queue.move_selected_down() {
        log::error!("{}", e);
    };

    None
}

pub fn rewind(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::RunningFFmpeg {
        return None;
    }

    let track_dur = if let Some(current_track) = model.player.current.as_mut() {
        current_track.duration.unwrap_or(Duration::from_secs(0))
    } else {
        return None;
    };

    // Rewind duration depends on the duration of the track.
    let rewind_dur = if track_dur > Duration::from_secs(36000) {
        Duration::from_secs(1800)
    } else if track_dur > Duration::from_secs(18000) {
        Duration::from_secs(600)
    } else if track_dur > Duration::from_secs(3600) {
        Duration::from_secs(300)
    } else if track_dur > Duration::from_secs(1800) {
        Duration::from_secs(60)
    } else if track_dur > Duration::from_secs(600) {
        Duration::from_secs(10)
    } else {
        Duration::from_secs(5)
    };

    model.player.rewind(rewind_dur).expect("Error rewinding.");

    None
}

pub fn seek(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::RunningFFmpeg {
        return None;
    }

    let track_dur = if let Some(current_track) = model.player.current.as_mut() {
        current_track.duration.unwrap_or(Duration::from_secs(0))
    } else {
        return None;
    };

    // Seek duration depends on the duration of the track.
    let seek_dur = if track_dur > Duration::from_secs(3600) {
        Duration::from_secs(20)
    } else if track_dur > Duration::from_secs(1800) {
        Duration::from_secs(15)
    } else if track_dur > Duration::from_secs(600) {
        Duration::from_secs(10)
    } else {
        Duration::from_secs(5)
    };

    if let Some(current_track) = &model.player.current {
        let duration = current_track.duration;
        model
            .player
            .seek(&duration.unwrap_or(Duration::from_secs(0)), seek_dur)
            .expect("Error seeking.");
    }

    None
}

pub fn skip(model: &mut Model) -> Option<Message> {
    if model.player.queue.is_empty() {
        return None;
    }

    if let Some(handle) = model.player.ffmpeg_handle.take() {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(handle.lock().unwrap().kill()).unwrap();
    }

    model.session.state = RunningState::Running;

    log::info!("Trying to load {:?}.", model.player.queue.front_path());
    model.player.sink.clear();
    model
        .player
        .load_next_track()
        .expect("Error loading next track.");

    if let Some(current_track) = model.player.current.as_mut()
        && (current_track.conversion_status == FormatConversion::Unnecessary
            || current_track.conversion_status == FormatConversion::Done)
    {
        model.player.reload().expect("Error reloading track.");
    }

    None
}

pub fn toggle_arrange(player: &mut Player) -> Option<Message> {
    player.queue.toggle_arrange();

    None
}

pub fn toggle_loop(player: &mut Player) -> Option<Message> {
    player.looping = !player.looping;

    None
}

pub fn decrease_volume(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::RunningFFmpeg {
        return None;
    }

    model.player.decrease_volume(0.05);

    None
}

pub fn increase_volume(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::RunningFFmpeg {
        return None;
    }

    model.player.increase_volume(0.05);

    None
}

pub fn previous_track(model: &mut Model) -> Option<Message> {
    if model.player.previous.is_empty() {
        return None;
    }

    if let Some(handle) = model.player.ffmpeg_handle.take() {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(handle.lock().unwrap().kill()).unwrap();
    }

    model.session.state = RunningState::Running;

    log::info!("Trying to load previous track.");
    model.player.sink.clear();
    model
        .player
        .load_prev_track()
        .expect("Error loading previous track.");

    if let Some(current_track) = model.player.current.as_mut()
        && current_track.conversion_status != FormatConversion::Running
    {
        model.player.reload().expect("Error reloading track.");
    }

    None
}

pub fn shuffle_queue(player: &mut Player) -> Option<Message> {
    player.queue.shuffle();

    None
}

pub fn clear_queue(player: &mut Player) -> Option<Message> {
    player.queue.clear();

    None
}
