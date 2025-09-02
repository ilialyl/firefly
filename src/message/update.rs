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

use color_eyre::eyre::Result;

pub fn update(
    model: &mut Model,
    msg: Message,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> (Option<Message>, Result<()>) {
    match msg {
        Message::Tick => (None, tick(model, msg_tx, info_tx)),
        Message::LoadNow => (None, load_now(model, msg_tx, info_tx)),
        Message::TogglePlay => (None, toggle_play(model)),
        Message::QueueDir => (None, queue_dir(model)),
        Message::QueueFile => (None, queue_file(model)),
        Message::QueueUp => (None, queue_up(model)),
        Message::QueueDown => (None, queue_down(model)),
        Message::Rewind => (None, rewind(model, info_tx)),
        Message::Seek => (None, seek(model, info_tx)),
        Message::Skip => (None, skip(model, msg_tx, info_tx)),
        Message::ToggleArrange => (None, toggle_arrange(model)),
        Message::ToggleLoop => (None, toggle_loop(model)),
        Message::VolumeDown => (None, volume_down(model)),
        Message::VolumeUp => (None, volume_up(model)),
        Message::Busy => (None, busy(model)),
        Message::ConversionStarted(handle) => {
            (Some(Message::Busy), conversion_started(handle, model))
        }
        Message::ConversionEnded => (None, conversion_ended(model)),
        Message::Quit => (None, quit(model)),
    }
}

fn tick(model: &mut Model, msg_tx: &Sender<Message>, info_tx: &Sender<String>) -> Result<()> {
    if model.session.state == RunningState::Busy {
        return Ok(());
    }
    let mut err: Option<color_eyre::eyre::ErrReport> = None;
    {
        // If sink paused, set status to paused
        // If track_path exists in App and sink isn't empty, set status to playing
        if model.playback.sink.is_paused() {
            model.playback.status = PlaybackStatus::Paused;
        } else if model.playback.current.path.is_some() && !model.playback.sink.empty() {
            model.playback.status = PlaybackStatus::Playing;
        }

        if model.playback.sink.empty() {
            model.playback.status = PlaybackStatus::Idle;
        }

        // Get track position
        model.playback.current.pos = Some(model.playback.sink.get_pos());

        // If path, duration, and position are not None,
        // If sink is empty or the track is within 3 seconds away from ending
        // If looping is on, load the same track
        // Else, load next track in queue.

        {
            if let (Some(path), Some(dur), Some(pos)) = (
                model.playback.current.path.clone(),
                model.playback.current.duration,
                model.playback.current.pos,
            ) && model.playback.sink.empty()
                && dur.saturating_sub(pos) < Duration::from_secs(3)
            {
                if model.playback.looping {
                    if let Err(e) = player::load_track(&path, &mut model.playback) {
                        err = Some(e);
                    }
                } else {
                    model.playback.current.pos = None;
                    model.playback.current.duration = None;
                    model.playback.status = PlaybackStatus::Idle;
                }
            }
        }
    }
    if let Some(e) = err {
        log::error!("{}", e);
    }

    if model.playback.status == PlaybackStatus::Idle && !model.playback.queue.is_empty() {
        log::info!("[TICK] Trying to load {:?}.", model.playback.queue.front());
        if let Err(e) = player::try_next_track(&mut model.playback, msg_tx, info_tx) {
            log::error!("{}", e);
        };
    }

    Ok(())
}

fn load_now(model: &mut Model, msg_tx: &Sender<Message>, info_tx: &Sender<String>) -> Result<()> {
    if let Some(path) = player::choose_file() {
        if let Some(handle) = model.ffmpeg_handle.take() {
            let runtime = Runtime::new().unwrap();
            runtime.block_on(handle.lock().unwrap().kill()).unwrap();
        }
        if let Err(e) = player::load_now(path, &mut model.playback, msg_tx, info_tx) {
            log::error!("{}", e);
        }
    }

    Ok(())
}

fn toggle_play(model: &mut Model) -> Result<()> {
    if model.session.state == RunningState::Busy {
        return Ok(());
    }

    if model.playback.status == PlaybackStatus::Playing {
        model.playback.sink.pause();
    } else {
        model.playback.sink.play();
    }

    Ok(())
}

fn queue_dir(model: &mut Model) -> Result<()> {
    if let Some(dir) = player::choose_dir() {
        model.playback.queue.enqueue_dir(dir);
    }

    Ok(())
}

fn queue_file(model: &mut Model) -> Result<()> {
    if let Some(path_vec) = player::choose_multiple_files() {
        model.playback.queue.enqueue_tracks(path_vec);
    }

    Ok(())
}

fn queue_up(model: &mut Model) -> Result<()> {
    if let Err(e) = model.playback.queue.move_selected_up() {
        log::info!("{}", e);
    };

    Ok(())
}

fn queue_down(model: &mut Model) -> Result<()> {
    if let Err(e) = model.playback.queue.move_selected_down() {
        log::info!("{}", e);
    };

    Ok(())
}

fn rewind(model: &mut Model, info_tx: &Sender<String>) -> Result<()> {
    if model.session.state == RunningState::Busy {
        return Ok(());
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

    Ok(())
}

fn seek(model: &mut Model, info_tx: &Sender<String>) -> Result<()> {
    if model.session.state == RunningState::Busy {
        return Ok(());
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

    Ok(())
}

fn skip(model: &mut Model, msg_tx: &Sender<Message>, info_tx: &Sender<String>) -> Result<()> {
    model.session.state = RunningState::Running;

    if model.playback.queue.is_empty() {
        return Ok(());
    }
    log::info!("Skipping...");

    if let Some(handle) = model.ffmpeg_handle.take() {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(handle.lock().unwrap().kill()).unwrap();
    }

    if let Err(e) = player::try_next_track(&mut model.playback, msg_tx, info_tx) {
        log::error!("{}", e);
    };

    Ok(())
}

fn toggle_arrange(model: &mut Model) -> Result<()> {
    model.playback.queue.toggle_arrange();

    Ok(())
}

fn toggle_loop(model: &mut Model) -> Result<()> {
    model.playback.looping = !model.playback.looping;

    Ok(())
}

fn volume_down(model: &mut Model) -> Result<()> {
    if model.session.state == RunningState::Busy {
        return Ok(());
    }

    player::decrease_volume(&mut model.playback.sink, 0.05);

    Ok(())
}

fn volume_up(model: &mut Model) -> Result<()> {
    if model.session.state == RunningState::Busy {
        return Ok(());
    }

    player::increase_volume(&mut model.playback.sink, 0.05);

    Ok(())
}

fn busy(model: &mut Model) -> Result<()> {
    model.session.state = RunningState::Busy;

    Ok(())
}

fn conversion_started(handle: Arc<Mutex<FFmpegProcess>>, model: &mut Model) -> Result<()> {
    model.ffmpeg_handle = Some(handle);

    Ok(())
}

fn conversion_ended(model: &mut Model) -> Result<()> {
    model.session.state = RunningState::Running;
    if let Some(path) = model.playback.current.path.clone()
        && let Err(e) = player::play_next_track(&path, &mut model.playback)
    {
        log::error!("{}", e);
    };

    Ok(())
}

fn quit(model: &mut Model) -> Result<()> {
    model.session.state = RunningState::Done;

    Ok(())
}
