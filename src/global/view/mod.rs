pub mod confirmation_box;
pub mod help;
pub mod home;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{
    global::view_logic::focused_area::FocusedArea,
    model::Model,
    player::view::{control_bar, cover_art, track_details},
    playlist, queue,
    user_input::logic::InputMode,
};

pub fn draw(model: &mut Model, frame: &mut Frame) {
    if terminal_is_small(frame.area()) {
        draw_mini(model, frame);
    } else {
        if matches!(model.focused_view_area, FocusedArea::Queue) {
            model.focused_view_area.cycle_right();
        }
        draw_normal_size(model, frame);
    }
}

pub fn terminal_is_small(area: Rect) -> bool {
    area.width <= 60 && area.height < 15
}

fn draw_normal_size(model: &mut Model, frame: &mut Frame) {
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
        Line::from("Hide Help <H>").bold()
    } else {
        Line::from("Help <H>")
    };

    Block::new()
        .title(top_left_text)
        .title_alignment(Alignment::Left)
        .padding(Padding::horizontal(1))
        .render(outer_layout[0], frame.buffer_mut());

    home::draw(model, frame, outer_layout[1]);

    if matches!(model.focused_view_area, FocusedArea::ControlBar) {
        Block::bordered().render(outer_layout[2], frame.buffer_mut());
    }

    let main_view_chunk = Layout::default()
        .constraints(vec![Constraint::Percentage(100)])
        .margin(1)
        .split(outer_layout[2]);

    control_bar::draw(main_view_chunk[0], frame, model);

    if model.show_help {
        help::draw(frame, frame.area());
    }

    match model.input_mode.clone() {
        InputMode::Insert(prompt, _) => {
            model
                .user_input
                .draw(prompt.as_str(), 40, 3, frame, frame.area())
        }
        InputMode::Commands => {}
        InputMode::Confirmation => confirmation_box::draw(model, frame, frame.area()),
    }
}

fn draw_mini(model: &mut Model, frame: &mut Frame) {
    match model.focused_view_area {
        FocusedArea::ControlBar => {
            if model.player.current.is_some() {
                let layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Length(frame.area().height * 2),
                        Constraint::Fill(1),
                    ])
                    .horizontal_margin(1)
                    .spacing(1)
                    .split(frame.area());

                cover_art::draw(layout[0], frame, model);
                track_details::draw(layout[1], frame, model);
            } else {
                Paragraph::new(vec![
                    Line::from("Mini Player"),
                    Line::from("Press Q to queue your tracks."),
                    Line::from("Enlarge for full visual."),
                ])
                .render(frame.area(), frame.buffer_mut());
            }
        }
        FocusedArea::Playlist => {
            playlist::view::draw(frame.area(), frame, model);
        }
        FocusedArea::Queue => {
            if model.queue.is_empty() {
                Paragraph::new("This is queue, but it's empty.")
                    .render(frame.area(), frame.buffer_mut());
            } else {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
                    .split(frame.area());
                queue::view::draw(chunks[0], frame, model);
                control_bar::draw_mini(chunks[1], frame, model);
            }
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
