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
    file_path: Option<PathBuf>,
    playing: bool,
    track_pos: String,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let (stream, sink) = player::get_sink().expect("Error creating sink");
        Self {
            _stream: stream,
            sink: Arc::new(Mutex::new(sink)),
            volume: 1.0,
            file_path: None,
            playing: false,
            track_pos: String::from("00:00"),
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
        }

        self.track_pos = player::get_track_pos(&self.sink);
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
                let track = player::load_track(&self.sink);
                if track.is_some() {
                    self.file_path = track;
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
                if self.file_path.is_some() {
                    player::forward(&self.sink);
                }
            }
            KeyCode::Left => {
                if self.file_path.is_some() {
                    let file = self.file_path.as_ref().unwrap().clone();
                    player::rewind(&self.sink, file);
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
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
            ])
            .margin(1)
            .split(area);

        let title = Line::from(" Firefly ".bold());
        let instructions = Line::from(vec![
            " Load ".into(),
            "<Enter>".blue().bold(),
            " Play/Pause ".into(),
            "<Spacebar>".blue().bold(),
            " Rewind/Forward ".into(),
            "<Left/Right>".blue().bold(),
            " Volume ".into(),
            "<Up/Down>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered());

        block.render(area, buf);

        let track_name: Text = match self.file_path.clone() {
            Some(f) => {
                let name = f.file_name().unwrap().to_str().unwrap();
                Text::from(format!("{}", name))
            }
            None => Text::from("Empty"),
        };

        let track_pos: Text = Text::from(self.track_pos.clone());

        let status: Text = match self.playing {
            true => Text::from("Playing"),
            false => Text::from("Paused"),
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

        Paragraph::new(volume)
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[3], buf);
    }
}
