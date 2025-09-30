pub mod confirmation_box;
pub mod info_box;
pub mod tabs;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Widget},
};

use crate::{
    global::view::tabs::SelectedTab, model::Model, player, playlist, user_input::logic::InputMode,
};

pub fn render_tui(model: &mut Model, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .split(frame.area());

    Block::new()
        .title(Line::style(Line::from("Firefly"), Style::new()).centered())
        .render(outer_layout[0], frame.buffer_mut());

    tabs::draw(model, outer_layout[0], frame.buffer_mut());

    let top_right_text = if model.status_msg.is_empty() {
        format!("v{}", env!("CARGO_PKG_VERSION"))
    } else {
        model.status_msg.clone()
    };

    Block::new()
        .title(Line::style(Line::from(top_right_text), Style::new()).right_aligned())
        .render(outer_layout[0], frame.buffer_mut());

    match model.selected_tab {
        SelectedTab::Main => player::view::draw(model, frame, outer_layout[1]),
        SelectedTab::Playlist => playlist::view::draw(model, frame, outer_layout[1]),
    }

    match model.input_mode.clone() {
        InputMode::Insert(prompt, _) => {
            model
                .user_input
                .render(prompt.as_str(), 40, 3, frame, frame.area())
        }
        InputMode::Commands => {}
        InputMode::Confirmation => confirmation_box::render(model, frame, frame.area()),
        InputMode::Info => {
            info_box::render(model, frame, frame.area());
        }
    }
}

pub fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn center_xy(area: Rect, percent_x: u16, length_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(length_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
