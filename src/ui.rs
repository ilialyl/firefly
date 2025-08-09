use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Text, ToSpan},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{app, player::Status};

// impl Widget for &app::App {
//     fn render(self, area: Rect, buf: &mut Buffer) {}
// }

pub fn render(frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .split(frame.area());

    Block::new()
        .fg(Color::White)
        .title("Firefly Player".to_span().into_centered_line())
        .render(outer_layout[0], frame.buffer_mut());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(outer_layout[1]);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(inner_layout[1]);

    frame.render_widget(
        Paragraph::new("Queue").block(Block::new().fg(Color::White).borders(Borders::ALL)),
        inner_layout[0],
    );

    frame.render_widget(
        Paragraph::new("Player")
            .block(Block::new().fg(Color::White).borders(Borders::ALL))
            .alignment(Alignment::Right),
        main_layout[0],
    );

    frame.render_widget(
        Paragraph::new("Control")
            .block(Block::new().fg(Color::White).borders(Borders::ALL))
            .alignment(Alignment::Right),
        main_layout[1],
    );
}
