use std::{sync::mpsc::Sender, thread, time::Duration};

use crate::{
    logic::player::{self, load_now, play_next_track},
    message::Message,
    model::{Model, RunningState},
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
                if let Err(e) = load_now(model, path, msg_tx, info_tx) {
                    log::error!("{}", e);
                }
            }

            (None, Ok(()))
        }
        Message::PlayPause => {
            if model.running_state == RunningState::Busy {
                return (None, Ok(()));
            }

            if model.status == player::Status::Playing {
                model.sink.pause();
            } else {
                model.sink.play();
            }

            (None, Ok(()))
        }
        Message::QueueDir => {
            if let Some(dir) = player::choose_dir() {
                model.track_queue.enqueue_dir(dir);
            }

            (None, Ok(()))
        }
        Message::QueueFile => {
            if let Some(path_vec) = player::choose_multiple_files() {
                model.track_queue.enqueue_tracks(path_vec);
            }

            (None, Ok(()))
        }
        Message::QueueUp => {
            if let Err(e) = model.track_queue.move_selected_up() {
                log::info!("{}", e);
            };

            (None, Ok(()))
        }

        Message::QueueDown => {
            if let Err(e) = model.track_queue.move_selected_down() {
                log::info!("{}", e);
            };

            (None, Ok(()))
        }

        Message::Quit => {
            model.running_state = RunningState::Done;

            (None, Ok(()))
        }

        Message::Rewind => {
            if model.running_state == RunningState::Busy {
                return (None, Ok(()));
            }

            if let Some(track) = model.current_track.path.clone() {
                if model.current_track.path.is_some() {
                    info_tx.send("Rewinding...".to_string()).unwrap();
                    if let Err(e) = player::rewind(&mut model.sink, &track, Duration::from_secs(5))
                    {
                        let cloned_info_tx = info_tx.clone();

                        log::error!("{}", e);
                        thread::spawn(move || {
                            cloned_info_tx.send(e.to_string()).unwrap();
                            thread::sleep(Duration::from_secs(2));
                            cloned_info_tx.send("".to_string()).unwrap();
                        });
                    };
                    model.current_track.duration = player::get_track_duration(&track).ok();
                    info_tx.send("".to_string()).unwrap();
                }
            }

            (None, Ok(()))
        }

        Message::Seek => {
            if model.running_state == RunningState::Busy {
                return (None, Ok(()));
            }

            if let Some(track_dur) = &model.current_track.duration {
                if model.current_track.path.is_some() {
                    player::forward(&mut model.sink, track_dur, Duration::from_secs(5));
                }
            }

            (None, Ok(()))
        }

        Message::Skip => {
            if model.track_queue.is_empty() {
                return (None, Ok(()));
            }
            log::info!("Skipping...");

            if let Some(handle) = model.ffmpeg_handle.take() {
                let runtime = Runtime::new().unwrap();
                runtime.block_on(handle.lock().unwrap().kill()).unwrap();
            }

            if let Err(e) = player::try_next_track(model, msg_tx, info_tx) {
                log::error!("{}", e);
            };

            (None, Ok(()))
        }

        Message::Tick => {
            if model.running_state == RunningState::Busy {
                return (None, Ok(()));
            }
            let mut err: Option<color_eyre::eyre::ErrReport> = None;
            {
                // If sink paused, set status to paused
                // If track_path exists in App and sink isn't empty, set status to playing
                if model.sink.is_paused() {
                    model.status = player::Status::Paused;
                } else if model.current_track.path.is_some() && !model.sink.empty() {
                    model.status = player::Status::Playing;
                }

                if model.sink.empty() {
                    model.status = player::Status::Idle;
                }

                // Get track position
                model.current_track.pos = Some(model.sink.get_pos());

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
                        if model.sink.empty() && dur.saturating_sub(pos) < Duration::from_secs(3) {
                            if model.looping {
                                if let Err(e) = player::load_track(&mut model.sink, path) {
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
                log::error!("{}", e);
            }

            if model.status == player::Status::Idle && !model.track_queue.is_empty() {
                log::info!(
                    "Trying to load {:?}.",
                    model.track_queue.front().clone().take()
                );
                if let Err(e) = player::try_next_track(model, msg_tx, info_tx) {
                    log::error!("{}", e);
                };
            }

            (None, Ok(()))
        }

        Message::ToggleArrange => {
            model.track_queue.toggle_arrange();

            (None, Ok(()))
        }

        Message::ToggleLoop => {
            model.looping = !model.looping;

            (None, Ok(()))
        }

        Message::VolumeDown => {
            if model.running_state == RunningState::Busy {
                return (None, Ok(()));
            }

            player::decrease_volume(&mut model.sink, 0.05);

            (None, Ok(()))
        }

        Message::VolumeUp => {
            if model.running_state == RunningState::Busy {
                return (None, Ok(()));
            }

            player::increase_volume(&mut model.sink, 0.05);

            (None, Ok(()))
        }

        Message::ConversionStarted(handle) => {
            model.ffmpeg_handle = Some(handle);

            (None, Ok(()))
        }

        Message::Busy => {
            model.running_state = RunningState::Busy;

            (None, Ok(()))
        }

        Message::ConversionEnded => {
            model.running_state = RunningState::Running;
            if let Some(path) = model.current_track.path.clone() {
                if let Err(e) = play_next_track(model, &path) {
                    log::error!("{}", e);
                };
            }

            (None, Ok(()))
        }
    }
}
