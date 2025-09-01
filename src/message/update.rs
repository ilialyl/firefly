use std::{sync::mpsc::Sender, thread, time::Duration};

use crate::{
    logic::{
        playback_state::PlaybackStatus,
        player::{self, load_now, play_next_track},
        session_state::RunningState,
    },
    message::Message,
    model::Model,
};

use tokio::runtime::Runtime;

use color_eyre::eyre::Result;

pub fn update(
    model: &mut Model,
    msg: Message,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> (Option<Message>, Result<()>) {
    match msg {
        Message::LoadNow => {
            if let Some(path) = player::choose_file() {
                if let Some(handle) = model.ffmpeg_handle.take() {
                    let runtime = Runtime::new().unwrap();
                    runtime.block_on(handle.lock().unwrap().kill()).unwrap();
                }
                if let Err(e) = load_now(path, &mut model.playback, msg_tx, info_tx) {
                    log::error!("{}", e);
                }
            }

            (None, Ok(()))
        }
        Message::PlayPause => {
            if model.session.state == RunningState::Busy {
                return (None, Ok(()));
            }

            if model.playback.status == PlaybackStatus::Playing {
                model.playback.sink.pause();
            } else {
                model.playback.sink.play();
            }

            (None, Ok(()))
        }
        Message::QueueDir => {
            if let Some(dir) = player::choose_dir() {
                model.playback.queue.enqueue_dir(dir);
            }

            (None, Ok(()))
        }
        Message::QueueFile => {
            if let Some(path_vec) = player::choose_multiple_files() {
                model.playback.queue.enqueue_tracks(path_vec);
            }

            (None, Ok(()))
        }
        Message::QueueUp => {
            if let Err(e) = model.playback.queue.move_selected_up() {
                log::info!("{}", e);
            };

            (None, Ok(()))
        }

        Message::QueueDown => {
            if let Err(e) = model.playback.queue.move_selected_down() {
                log::info!("{}", e);
            };

            (None, Ok(()))
        }

        Message::Quit => {
            model.session.state = RunningState::Done;

            (None, Ok(()))
        }

        Message::Rewind => {
            if model.session.state == RunningState::Busy {
                return (None, Ok(()));
            }

            if let Some(track) = model.playback.current.path.clone() {
                if model.playback.current.path.is_some() {
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
                        player::get_track_duration(&track, &mut model.playback).ok();
                    info_tx.send("".to_string()).unwrap();
                }
            }

            (None, Ok(()))
        }

        Message::Seek => {
            if model.session.state == RunningState::Busy {
                return (None, Ok(()));
            }

            if let Some(track_dur) = &model.playback.current.duration {
                if model.playback.current.path.is_some() {
                    player::forward(&mut model.playback.sink, track_dur, Duration::from_secs(5));
                }
            }

            (None, Ok(()))
        }

        Message::Skip => {
            model.session.state = RunningState::Running;

            if model.playback.queue.is_empty() {
                return (None, Ok(()));
            }
            log::info!("Skipping...");

            if let Some(handle) = model.ffmpeg_handle.take() {
                let runtime = Runtime::new().unwrap();
                runtime.block_on(handle.lock().unwrap().kill()).unwrap();
            }

            if let Err(e) = player::try_next_track(&mut model.playback, msg_tx, info_tx) {
                log::error!("{}", e);
            };

            (None, Ok(()))
        }

        Message::Tick => {
            if model.session.state == RunningState::Busy {
                return (None, Ok(()));
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
                    ) {
                        if model.playback.sink.empty()
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
            }
            if let Some(e) = err {
                log::error!("{}", e);
            }

            if model.playback.status == PlaybackStatus::Idle && !model.playback.queue.is_empty() {
                log::info!(
                    "[TICK] Trying to load {:?}.",
                    model.playback.queue.front().clone().take()
                );
                if let Err(e) = player::try_next_track(&mut model.playback, msg_tx, info_tx) {
                    log::error!("{}", e);
                };
            }

            (None, Ok(()))
        }

        Message::ToggleArrange => {
            model.playback.queue.toggle_arrange();

            (None, Ok(()))
        }

        Message::ToggleLoop => {
            model.playback.looping = !model.playback.looping;

            (None, Ok(()))
        }

        Message::VolumeDown => {
            if model.session.state == RunningState::Busy {
                return (None, Ok(()));
            }

            player::decrease_volume(&mut model.playback.sink, 0.05);

            (None, Ok(()))
        }

        Message::VolumeUp => {
            if model.session.state == RunningState::Busy {
                return (None, Ok(()));
            }

            player::increase_volume(&mut model.playback.sink, 0.05);

            (None, Ok(()))
        }

        Message::ConversionStarted(handle) => {
            model.ffmpeg_handle = Some(handle);

            (Some(Message::Busy), Ok(()))
        }

        Message::Busy => {
            model.session.state = RunningState::Busy;

            (None, Ok(()))
        }

        Message::ConversionEnded => {
            model.session.state = RunningState::Running;
            if let Some(path) = model.playback.current.path.clone() {
                if let Err(e) = play_next_track(&path, &mut model.playback) {
                    log::error!("{}", e);
                };
            }

            (None, Ok(()))
        }
    }
}
