use ratatui::layout::{Layout, Rect};

use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction},
    style::Color,
    widgets::{Block, Widget},
};

use crate::model::Model;

pub fn draw(_model: &mut Model, frame: &mut Frame, area: Rect) {
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
        .split(area);

    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(outer_chunks[0]);

    Block::bordered()
        .title(Line::style(
            Line::from("Entries"),
            Style::new().fg(Color::White),
        ))
        .border_style(Style::default().fg(Color::White))
        .title_alignment(Alignment::Left)
        .render(inner_chunks[0], frame.buffer_mut());

    Block::bordered()
        .border_style(Style::default().fg(Color::White))
        .render(inner_chunks[1], frame.buffer_mut());

    draw_controls(frame, outer_chunks[1]);
}

fn draw_controls(frame: &mut Frame, area: Rect) {
    let controls = [
        "New <n>",
        "Rename <F2>",
        "Del Playlist <F5>",
        "Add Tracks <q/Q>",
        "Remove Track <Del>",
        "Arrange <a>",
        "Send to Player <F1>",
    ];

    let paragraph = Paragraph::new(controls.join("  ")).centered().white();

    frame.render_widget(paragraph, area);
}
