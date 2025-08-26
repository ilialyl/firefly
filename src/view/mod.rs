pub mod main_tab;
pub mod terminal;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::ToSpan,
    widgets::{Block, Widget},
};

use crate::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    render(model, frame);
}

pub fn render(model: &Model, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .split(frame.area());

    Block::new()
        .fg(Color::White)
        .title("Firefly Player".to_span().into_centered_line())
        .render(outer_layout[0], frame.buffer_mut());

    main_tab::draw(model, frame, outer_layout[1]);
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn stop_info_display(model: &mut Model) {
    model.info.push(String::new());
}

pub fn display_info(model: &mut Model, info: &str) {
    model.info.push(info.to_string());
}
