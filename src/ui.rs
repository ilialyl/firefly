use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::{app, player::Status};

impl Widget for &app::App {
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

        let status: Text = match self.status {
            Status::Playing => Text::from("Playing"),
            Status::Paused => Text::from("Paused"),
            Status::Idle => Text::from("Idle"),
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
