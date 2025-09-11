pub mod main_tab;
pub mod tabs;
pub mod terminal;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Widget},
};

use crate::{model::Model, view::tabs::SelectedTab};

pub fn view(model: &mut Model, frame: &mut Frame) {
    render(model, frame);
}

pub fn render(model: &mut Model, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .split(frame.area());

    Block::new()
        .title(Line::style(Line::from("Player"), Style::new().fg(Color::White)).centered())
        .render(outer_layout[0], frame.buffer_mut());

    tabs::draw(model, outer_layout[0], frame.buffer_mut());

    Block::new()
        .title(
            Line::style(
                Line::from(format!("v{}", env!("CARGO_PKG_VERSION"))),
                Style::new().fg(Color::White),
            )
            .right_aligned(),
        )
        .render(outer_layout[0], frame.buffer_mut());

    match model.selected_tab {
        SelectedTab::Main => main_tab::draw(model, frame, outer_layout[1]),
        SelectedTab::Playlist => {}
    }
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}
