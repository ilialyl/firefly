use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use rodio::{OutputStream, Sink};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    player::{self, Status},
    ui,
};

pub struct App {
    pub _stream: OutputStream,
    pub sink: Arc<Mutex<Sink>>,
    pub volume: f32,
    pub track_path: Option<PathBuf>,
    pub status: player::Status,
    pub track_pos: Option<Duration>,
    pub track_duration: Option<Duration>,
    pub looping: bool,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        let (stream, sink) = player::get_sink().expect("Error creating sink");
        Self {
            _stream: stream,
            sink: Arc::new(Mutex::new(sink)),
            volume: 1.0,
            track_path: None,
            status: Status::Idle,
            track_pos: None,
            track_duration: None,
            looping: false,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            self.update_logic();
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn update_logic(&mut self) {
        {
            let sink = self.sink.lock().unwrap();
            if sink.is_paused() {
                self.status = Status::Paused;
            } else if self.track_path.is_some() && !sink.empty() {
                self.status = Status::Playing;
            }
            self.track_pos = Some(sink.get_pos());

            if let (Some(path), Some(dur), Some(pos)) =
                (&self.track_path, self.track_duration, self.track_pos)
            {
                if sink.empty() && dur.saturating_sub(pos) < Duration::from_secs(3) {
                    if self.looping {
                        player::load_track(&self.sink, path.clone());
                    } else {
                        self.status = Status::Idle;
                        self.track_pos = None;
                        self.track_duration = None;
                    }
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        // frame.render_widget(self, frame.area());
        ui::render(self, frame);
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Enter => {
                let track = player::load_track_manually(&self.sink);
                if track.is_some() {
                    self.track_path = track;
                    self.track_duration =
                        Some(player::get_track_duration(self.track_path.clone().unwrap()));
                }
            }
            KeyCode::Char(' ') => {
                let sink = self.sink.lock().unwrap();
                if self.status == Status::Playing {
                    sink.pause();
                } else {
                    sink.play();
                }
            }
            KeyCode::Up => {
                player::increase_volume(&self.sink, 0.05);
                self.volume = self.sink.lock().unwrap().volume();
            }
            KeyCode::Down => {
                player::decrease_volume(&self.sink, 0.05);
                self.volume = self.sink.lock().unwrap().volume();
            }
            KeyCode::Right => {
                if let Some(track_dur) = &self.track_duration {
                    if self.track_path.is_some() {
                        player::forward(&self.sink, track_dur, Duration::from_secs(5));
                    }
                }
            }
            KeyCode::Left => {
                if let Some(track) = &self.track_path {
                    if self.track_path.is_some() {
                        player::rewind(&self.sink, track.clone(), Duration::from_secs(5));
                        self.track_duration = Some(player::get_track_duration(track.clone()));
                    }
                }
            }
            KeyCode::Char('l') => {
                if self.looping {
                    self.looping = false;
                } else {
                    self.looping = true;
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub fn track_pos_as_str(&self) -> String {
        let track_pos = self.track_pos.unwrap_or(Duration::from_secs(0));
        let sec = track_pos.as_secs() % 60;
        let min = track_pos.as_secs() / 60;

        format!("{:02}:{:02}", min, sec)
    }
}
