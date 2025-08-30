use std::{
    sync::{Arc, Mutex, mpsc::Sender},
    time::Duration,
};

use color_eyre::eyre::{Result, eyre};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rust_ffmpeg::FFmpegProcess;
use tokio::runtime::Runtime;

use crate::{
    model::{
        Model, RunningState,
        player::{self, load_now, play_next_track},
    },
    view::{self},
};

pub enum Message {
    Tick,
    LoadNow,
    PlayPause,
    Skip,
    VolumeUp,
    VolumeDown,
    Seek,
    Rewind,
    ToggleArrange,
    ToggleLoop,
    QueueUp,
    QueueDown,
    QueueFile,
    QueueDir,
    Quit,
    ConversionStarted(Arc<Mutex<FFmpegProcess>>),
    ConversionEnded,
    Busy(String),
    Log(String),
}

pub fn update(
    model: &mut Model,
    msg: Message,
    tx: &Sender<Message>,
) -> (Option<Message>, Result<()>) {
    match msg {
        Message::LoadNow => {
            if model.busy {
                return (None, Ok(()));
            }

            if let Some(path) = player::choose_file() {
                if let Err(e) = load_now(model, path, tx) {
                    log::error!("{}", e);
                }
            }

            (None, Ok(()))
        }
        Message::PlayPause => {
            if model.busy {
                return (None, Ok(()));
            }

            let sink = match model.sink.lock() {
                Ok(s) => s,
                Err(e) => {
                    return (None, Err(eyre!(e.to_string())));
                }
            };

            if model.status == player::Status::Playing {
                sink.pause();
            } else {
                sink.play();
            }

            (None, Ok(()))
        }
        Message::QueueDir => {
            if let Some(dir) = player::choose_dir() {
                player::enqueue_dir(dir, &mut model.track_queue);
            }

            (None, Ok(()))
        }
        Message::QueueFile => {
            if let Some(path_vec) = player::choose_multiple_files() {
                player::enqueue_track(path_vec, &mut model.track_queue);
            }

            (None, Ok(()))
        }
        Message::QueueUp => {
            if model.selected_track == 0 || model.track_queue.is_empty() {
                return (None, Ok(()));
            }

            model.selected_track = model.selected_track.saturating_sub(1);

            if model.arrange_mode && model.track_queue.len() > model.selected_track {
                model
                    .track_queue
                    .swap(model.selected_track, model.selected_track + 1);
            }

            (None, Ok(()))
        }

        Message::QueueDown => {
            if model.track_queue.is_empty() || model.selected_track == model.track_queue.len() - 1 {
                return (None, Ok(()));
            }

            model.selected_track = (model.selected_track + 1).min(model.track_queue.len() - 1);

            if model.arrange_mode {
                model
                    .track_queue
                    .swap(model.selected_track, model.selected_track - 1);
            }

            (None, Ok(()))
        }

        Message::Quit => {
            model.running_state = RunningState::Done;

            (None, Ok(()))
        }

        Message::Rewind => {
            if model.busy {
                return (None, Ok(()));
            }

            if let Some(track) = model.current_track.path.clone() {
                if model.current_track.path.is_some() {
                    if let Err(e) = player::rewind(&model.sink, &track, Duration::from_secs(5)) {
                        view::display_info(model, e.to_string().as_str())
                    };
                    model.current_track.duration = player::get_track_duration(&track).ok();
                }
            }

            (None, Ok(()))
        }

        Message::Seek => {
            if model.busy {
                return (None, Ok(()));
            }

            if let Some(track_dur) = &model.current_track.duration {
                if model.current_track.path.is_some() {
                    player::forward(&model.sink, track_dur, Duration::from_secs(5));
                }
            }

            (None, Ok(()))
        }

        Message::Skip => {
            if let Some(handle) = model.ffmpeg_handle.take() {
                let runtime = Runtime::new().unwrap();
                runtime.block_on(handle.lock().unwrap().kill()).unwrap();
            }
            if let Err(e) = player::try_next_track(model, tx) {
                log::error!("{}", e);
            };

            (None, Ok(()))
        }

        Message::Tick => {
            if model.busy {
                return (None, Ok(()));
            }
            let mut err: Option<color_eyre::eyre::ErrReport> = None;
            {
                // Get sink
                let sink = match model.sink.lock() {
                    Ok(s) => s,
                    Err(e) => {
                        return (None, Err(eyre!(e.to_string())));
                    }
                };

                // If sink paused, set status to paused
                // If track_path exists in App and sink isn't empty, set status to playing
                if sink.is_paused() {
                    model.status = player::Status::Paused;
                } else if model.current_track.path.is_some() && !sink.empty() {
                    model.status = player::Status::Playing;
                }

                if sink.empty() {
                    model.status = player::Status::Idle;
                }

                // Get track position
                model.current_track.pos = Some(sink.get_pos());

                // If path, duration, and position are not None,
                // If sink is empty or the track is within 3 seconds away from ending
                // If looping is on, load the same track
                // Else, load next track in queue.

                {
                    if let (Some(path), Some(dur), Some(pos)) = (
                        &model.current_track.path,
                        model.current_track.duration,
                        model.current_track.pos,
                    ) {
                        if sink.empty() && dur.saturating_sub(pos) < Duration::from_secs(3) {
                            if model.looping {
                                if let Err(e) = player::load_track(&model.sink, path) {
                                    err = Some(e);
                                }
                            } else {
                                model.current_track.pos = None;
                                model.current_track.duration = None;
                                model.status = player::Status::Idle;
                            }
                        }
                    }
                }
            }
            if let Some(e) = err {
                view::display_info(model, &e.to_string())
            }

            if model.status == player::Status::Idle && !model.track_queue.is_empty() {
                if let Err(e) = player::try_next_track(model, tx) {
                    log::error!("{}", e);
                };
            }

            (None, Ok(()))
        }

        Message::ToggleArrange => {
            model.arrange_mode = !model.arrange_mode;

            (None, Ok(()))
        }

        Message::ToggleLoop => {
            model.looping = !model.looping;

            (None, Ok(()))
        }

        Message::VolumeDown => {
            if model.busy {
                return (None, Ok(()));
            }

            player::decrease_volume(&model.sink, 0.05);
            let sink = match model.sink.lock() {
                Ok(s) => s,
                Err(e) => {
                    return (None, Err(eyre!(e.to_string())));
                }
            };

            model.volume = sink.volume();

            (None, Ok(()))
        }

        Message::VolumeUp => {
            if model.busy {
                return (None, Ok(()));
            }

            player::increase_volume(&model.sink, 0.05);
            let sink = match model.sink.lock() {
                Ok(s) => s,
                Err(e) => {
                    return (None, Err(eyre!(e.to_string())));
                }
            };

            model.volume = sink.volume();

            (None, Ok(()))
        }

        Message::ConversionStarted(handle) => {
            model.ffmpeg_handle = Some(handle);

            (
                Some(Message::Busy(
                    "Converting format and normalizing volume".to_string(),
                )),
                Ok(()),
            )
        }

        Message::Busy(log) => {
            model.busy = true;

            (Some(Message::Log(log)), Ok(()))
        }

        Message::ConversionEnded => {
            model.busy = false;
            if let Err(e) = play_next_track(model, &model.current_track.path.clone().unwrap()) {
                log::error!("{}", e);
            };

            (Some(Message::Log(String::new())), Ok(()))
        }

        Message::Log(str) => {
            model.info.push(str);

            (None, Ok(()))
        }
    }
}

pub fn handle_events() -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(16))? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                return Ok(handle_key(key_event));
            }
            _ => {}
        };
    }
    Ok(None)
}

fn handle_key(key_event: KeyEvent) -> Option<Message> {
    match key_event.code {
        KeyCode::Esc => Some(Message::Quit),
        KeyCode::Char('n') => Some(Message::LoadNow),
        KeyCode::Char(' ') => Some(Message::PlayPause),
        KeyCode::Char('s') => Some(Message::Skip),
        KeyCode::Char('=') => Some(Message::VolumeUp),
        KeyCode::Char('-') => Some(Message::VolumeDown),
        KeyCode::Right => Some(Message::Seek),
        KeyCode::Left => Some(Message::Rewind),
        KeyCode::Char('l') => Some(Message::ToggleLoop),
        KeyCode::Char('q') => Some(Message::QueueFile),
        KeyCode::Char('Q') => Some(Message::QueueDir),
        KeyCode::Up => Some(Message::QueueUp),
        KeyCode::Down => Some(Message::QueueDown),
        KeyCode::Char('a') => Some(Message::ToggleArrange),
        _ => None,
    }
}
