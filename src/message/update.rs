use std::{sync::mpsc::Sender, time::Duration};

use crate::{
    logic::player::{self, load_now, play_next_track},
    message::Message,
    model::{Model, RunningState},
    view::{self},
};

use tokio::runtime::Runtime;

use color_eyre::eyre::Result;

pub fn update(
    model: &mut Model,
    msg: Message,
    tx: &Sender<Message>,
) -> (Option<Message>, Result<()>) {
    match msg {
        Message::LoadNow => {
            if let Some(path) = player::choose_file() {
                if let Some(handle) = model.ffmpeg_handle.take() {
                    let runtime = Runtime::new().unwrap();
                    runtime.block_on(handle.lock().unwrap().kill()).unwrap();
                }
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

            if model.status == player::Status::Playing {
                model.sink.pause();
            } else {
                model.sink.play();
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
                    if let Err(e) = player::rewind(&mut model.sink, &track, Duration::from_secs(5))
                    {
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
                    player::forward(&mut model.sink, track_dur, Duration::from_secs(5));
                }
            }

            (None, Ok(()))
        }

        Message::Skip => {
            if model.track_queue.is_empty() {
                return (None, Ok(()));
            }
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

            player::decrease_volume(&mut model.sink, 0.05);

            model.volume = model.sink.volume();

            (None, Ok(()))
        }

        Message::VolumeUp => {
            if model.busy {
                return (None, Ok(()));
            }

            player::increase_volume(&mut model.sink, 0.05);

            model.volume = model.sink.volume();

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
            if let Some(path) = model.current_track.path.clone() {
                if let Err(e) = play_next_track(model, &path) {
                    log::error!("{}", e);
                };
            }

            (Some(Message::Log(String::new())), Ok(()))
        }

        Message::Log(str) => {
            model.info.push(str);

            (None, Ok(()))
        }
    }
}
