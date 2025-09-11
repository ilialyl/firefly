pub mod main_tab;
pub mod tabs;
pub mod terminal;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::ToSpan,
    widgets::{Block, Widget},
};

use crate::model::Model;

pub fn view(model: &mut Model, frame: &mut Frame) {
    render(model, frame);
}

pub fn render(model: &mut Model, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .split(frame.area());

    Block::new()
        .fg(Color::White)
        .title("Firefly Player".to_span().into_centered_line())
        .render(outer_layout[0], frame.buffer_mut());

    tabs::draw(model, outer_layout[0], frame.buffer_mut());

    Block::new()
        .fg(Color::White)
        .title(
            format!("v{}", env!("CARGO_PKG_VERSION"))
                .to_span()
                .into_right_aligned_line(),
        )
        .render(outer_layout[0], frame.buffer_mut());

    main_tab::draw(model, frame, outer_layout[1]);
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}
