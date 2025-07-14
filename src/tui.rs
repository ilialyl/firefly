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
};

use crate::player::{self};

pub struct App {
    _stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
    file_path: Option<PathBuf>,
    playing: bool,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let (stream, sink) = player::get_sink().expect("Error creating sink");
        Self {
            _stream: stream,
            sink: Arc::new(Mutex::new(sink)),
            file_path: None,
            playing: false,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Enter => {
                self.file_path = player::load_track(&self.sink);

                self.playing = true;
            }
            KeyCode::Char(' ') => {
                let sink = self.sink.lock().unwrap();
                if self.playing {
                    sink.pause();
                    self.playing = false;
                } else {
                    sink.play();
                    self.playing = true;
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
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .margin(1)
            .split(area);

        let title = Line::from(" Firefly ".bold());
        let instructions = Line::from(vec![
            " Browse ".into(),
            "<Enter>".blue().bold(),
            " Play/Pause ".into(),
            "<Spacebar>".blue().bold(),
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

        let status: Text = match self.playing {
            true => Text::from("Playing"),
            false => Text::from("Paused"),
        };

        Paragraph::new(track_name)
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[0], buf);

        Paragraph::new(status)
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[1], buf);
    }
}
