use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use rodio::{OutputStream, Sink};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::player::{self};

pub struct App {
    _stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
    volume: f32,
    track_path: Option<PathBuf>,
    playing: bool,
    track_pos: Option<Duration>,
    track_duration: Option<Duration>,
    looping: bool,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let (stream, sink) = player::get_sink().expect("Error creating sink");
        Self {
            _stream: stream,
            sink: Arc::new(Mutex::new(sink)),
            volume: 1.0,
            track_path: None,
            playing: false,
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
                self.playing = false;
            } else {
                self.playing = true;
            }
            self.track_pos = Some(sink.get_pos());

            if let (Some(path), Some(dur), Some(pos)) =
                (&self.track_path, self.track_duration, self.track_pos)
            {
                if sink.empty() && dur.saturating_sub(pos) < Duration::from_secs(3) {
                    if self.looping {
                        player::load_track(&self.sink, path.clone());
                    } else {
                        self.playing = false;
                        self.track_pos = None;
                        self.track_duration = None;
                    }
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            KeyCode::Char('q') => self.exit(),
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
                if self.playing {
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
                if self.track_path.is_some() {
                    player::forward(
                        &self.sink,
                        self.track_duration.as_ref().unwrap(),
                        Duration::from_secs(5),
                    );
                }
            }
            KeyCode::Left => {
                if self.track_path.is_some() {
                    let file = self.track_path.as_ref().unwrap().clone();
                    player::rewind(&self.sink, file, Duration::from_secs(5));
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .margin(1)
            .split(area);

        let title = Line::from(" Firefly ".bold());
        let instructions = Line::from(vec![
            " Load ".into(),
            "<Enter>".blue().bold(),
            " Play/Pause ".into(),
            "<Space>".blue().bold(),
            " Rewind/Forward ".into(),
            "<Left/Right>".blue().bold(),
            " Vol ".into(),
            "<Up/Down>".blue().bold(),
            " Loop ".into(),
            "<L>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered());

        block.render(area, buf);

        let track_name: Text = match self.track_path.clone() {
            Some(path) => {
                if let Some(os_name) = path.file_name() {
                    if let Some(name) = os_name.to_str() {
                        Text::from(name.to_string())
                    } else {
                        Text::from("[Invalid UTF-8 name]")
                    }
                } else {
                    Text::from("[No file name]")
                }
            }
            None => Text::from("[Track Empty]"),
        };

        let track_pos: Text = Text::from(self.track_pos_as_str());

        let status: Text = match self.playing {
            true => Text::from("Playing"),
            false => Text::from("Paused"),
        };

        let loop_status: Text = match self.looping {
            true => Text::from("[Looped]"),
            false => Text::from(""),
        };

        let volume: Text = Text::from(format!("Volume: {}%", (self.volume * 100.00).ceil() as i32));

        Paragraph::new(track_name)
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[0], buf);

        Paragraph::new(track_pos)
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[1], buf);

        Paragraph::new(status)
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[2], buf);

        Paragraph::new(loop_status)
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[3], buf);

        Paragraph::new(volume)
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[4], buf);
    }
}
