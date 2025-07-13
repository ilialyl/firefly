use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use rodio::{OutputStream, Sink};
use std::{sync::{Arc, Mutex}, thread, time::Duration};

use crate::player::{self, get_source};

pub struct App {
    stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
    playing: bool,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let (stream, sink) = player::get_sink().expect("Error creating sink");
        Self {
            stream,
            sink: Arc::new(Mutex::new(sink)),
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
                self.playing = true;

                let sink = Arc::clone(&self.sink); // Clone Arc to move into thread

                thread::spawn(move || {
                    let source = get_source("audios/secretly_love_you.mp3").expect("Error obtaining source");
                    let sink = sink.lock().unwrap();
                    sink.append(source);
                    sink.play();

                    // Optional: wait until it's done playing
                    // This sleep won't block UI thread
                    sink.sleep_until_end();
                });
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
        let title = Line::from(" Firefly ".bold());
        let instructions = Line::from(vec![
            " Play ".into(),
            "<Enter>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let status: Text = match self.playing {
            true => Text::from("Playing"),
            false => Text::from("Paused"),
        };

        Paragraph::new(status)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
