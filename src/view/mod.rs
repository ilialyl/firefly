pub mod main_tab;
pub mod playlist_tab;
pub mod tabs;
pub mod terminal;
pub mod user_input_view;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Widget},
};

use crate::{logic::user_input::InputMode, model::Model, view::tabs::SelectedTab};

pub fn view(model: &mut Model, frame: &mut Frame) {
    render(model, frame);
}

pub fn render(model: &mut Model, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .split(frame.area());

    Block::new()
        .title(Line::style(Line::from("Player"), Style::new()).centered())
        .render(outer_layout[0], frame.buffer_mut());

    tabs::draw(model, outer_layout[0], frame.buffer_mut());

    Block::new()
        .title(
            Line::style(
                Line::from(format!("v{}", env!("CARGO_PKG_VERSION"))),
                Style::new(),
            )
            .right_aligned(),
        )
        .render(outer_layout[0], frame.buffer_mut());

    match model.selected_tab {
        SelectedTab::Main => main_tab::draw(model, frame, outer_layout[1]),
        SelectedTab::Playlist => playlist_tab::draw(model, frame, outer_layout[1]),
    }

    match model.input_mode.clone() {
        InputMode::Insert(prompt, _) => {
            model
                .user_input
                .render(prompt.as_str(), 40, 3, frame, frame.area())
        }
        InputMode::Commands => {}
    }
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}
