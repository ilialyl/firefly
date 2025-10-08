pub mod confirmation_box;
pub mod help_view;
pub mod main_view;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    widgets::{Block, Padding, Widget},
};

use crate::{
    global::view_logic::focused_area::FocusedArea, model::Model, player::view::control_bar,
    user_input::logic::InputMode,
};

pub fn render_tui(model: &mut Model, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let top_right_text = if model.info_msg.is_empty() {
        format!("v{}", env!("CARGO_PKG_VERSION"))
    } else {
        model.info_msg.clone()
    };

    Block::new()
        .title("Firefly")
        .title_alignment(Alignment::Center)
        .render(outer_layout[0], frame.buffer_mut());

    Block::new()
        .title(top_right_text)
        .title_alignment(Alignment::Right)
        .padding(Padding::horizontal(1))
        .render(outer_layout[0], frame.buffer_mut());

    let top_left_text = if model.show_help {
        "Hide Help <H>"
    } else {
        "Help <H>"
    };

    Block::new()
        .title(top_left_text)
        .title_alignment(Alignment::Left)
        .padding(Padding::horizontal(1))
        .render(outer_layout[0], frame.buffer_mut());

    main_view::draw(model, frame, outer_layout[1]);

    if matches!(model.focused_view_area, FocusedArea::ControlBarAndQueue) {
        Block::bordered().render(outer_layout[2], frame.buffer_mut());
    }

    let main_view_chunk = Layout::default()
        .constraints(vec![Constraint::Percentage(100)])
        .margin(1)
        .split(outer_layout[2]);

    control_bar::draw(main_view_chunk[0], frame, model);

    if model.show_help {
        help_view::draw(frame, frame.area());
    }

    match model.input_mode.clone() {
        InputMode::Insert(prompt, _) => {
            model
                .user_input
                .render(prompt.as_str(), 40, 3, frame, frame.area())
        }
        InputMode::Commands => {}
        InputMode::Confirmation => confirmation_box::render(model, frame, frame.area()),
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
