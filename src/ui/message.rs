use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    player::{self, Status},
    ui::{
        model::{Model, RunningState},
        refresh_frame, view,
    },
};

#[derive(PartialEq)]
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
}

pub fn update(model: &mut Model, msg: Message, terminal: &mut DefaultTerminal) -> Option<Message> {
    match msg {
        Message::LoadNow => {
            if let Some(path) = player::choose_file() {
                match player::is_rodio_supported(&path) {
                    Ok(condition) => {
                        if !condition {
                            view::display_info(
                                model,
                                "Converting format and normalizing volume...",
                            );

                            refresh_frame(model, terminal).expect("Error refreshing frame");
                            player::convert_format(&path);
                        }
                    }
                    Err(e) => view::display_info(model, e.to_string().as_str()),
                }

                if let Err(e) = player::load_track(&model.sink, &path) {
                    view::display_info(model, e.to_string().as_str())
                }
                model.current_track.path = Some(path);
                model.current_track.duration =
                    player::get_track_duration(model.current_track.path.as_ref().unwrap()).ok();

                view::stop_info_display(model);
            }

            None
        }
        Message::PlayPause => {
            let sink = model.sink.lock().unwrap();
            if model.status == Status::Playing {
                sink.pause();
            } else {
                sink.play();
            }

            None
        }
        Message::QueueDir => {
            if let Some(dir) = player::choose_dir() {
                player::enqueue_dir(dir, &mut model.track_queue);
            }

            None
        }
        Message::QueueFile => {
            if let Some(path_vec) = player::choose_multiple_files() {
                player::enqueue_track(path_vec, &mut model.track_queue);
            }

            None
        }
        Message::QueueUp => {
            if model.selected_track == 0 {
                return None;
            }

            model.selected_track = model.selected_track.checked_sub(1).unwrap_or(0);

            if model.arrange_queue && model.track_queue.len() > model.selected_track {
                model
                    .track_queue
                    .swap(model.selected_track, model.selected_track + 1);
            }

            None
        }

        Message::QueueDown => {
            if model.selected_track == model.track_queue.len() - 1 {
                return None;
            }

            model.selected_track = (model.selected_track + 1).min(model.track_queue.len() - 1);

            if model.arrange_queue {
                model
                    .track_queue
                    .swap(model.selected_track, model.selected_track - 1);
            }

            None
        }

        Message::Quit => {
            model.running_state = RunningState::Done;

            None
        }

        Message::Rewind => {
            if let Some(track) = model.current_track.path.clone() {
                if model.current_track.path.is_some() {
                    if let Err(e) = player::rewind(&model.sink, &track, Duration::from_secs(5)) {
                        view::display_info(model, e.to_string().as_str())
                    };
                    model.current_track.duration = player::get_track_duration(&track).ok();
                }
            }

            None
        }

        Message::Seek => {
            if let Some(track_dur) = &model.current_track.duration {
                if model.current_track.path.is_some() {
                    player::forward(&model.sink, track_dur, Duration::from_secs(5));
                }
            }

            None
        }

        Message::Skip => {
            player::play_next_track(model, terminal);

            None
        }

        Message::Tick => {
            let mut err: Option<color_eyre::eyre::ErrReport> = None;
            {
                // Get sink
                let sink = model.sink.lock().unwrap();

                // If sink paused, set status to paused
                // If track_path exists in App and sink isn't empty, set status to playing
                if sink.is_paused() {
                    model.status = Status::Paused;
                } else if model.current_track.path.is_some() && !sink.empty() {
                    model.status = Status::Playing;
                }

                if sink.empty() {
                    model.status = Status::Idle;
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
                                model.status = Status::Idle;
                            }
                        }
                    }
                }
            }
            if let Some(e) = err {
                view::display_info(model, e.to_string().as_str())
            }

            if model.status == Status::Idle && !model.track_queue.is_empty() {
                player::play_next_track(model, terminal);
            }

            None
        }

        Message::ToggleArrange => {
            if model.arrange_queue {
                model.arrange_queue = false;
            } else {
                model.arrange_queue = true;
            }

            None
        }

        Message::ToggleLoop => {
            if model.looping {
                model.looping = false;
            } else {
                model.looping = true;
            }

            None
        }

        Message::VolumeDown => {
            player::decrease_volume(&model.sink, 0.05);
            model.volume = model.sink.lock().unwrap().volume();

            None
        }

        Message::VolumeUp => {
            player::increase_volume(&model.sink, 0.05);
            model.volume = model.sink.lock().unwrap().volume();

            None
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
