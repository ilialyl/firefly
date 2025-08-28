use ratatui::layout::{Layout, Rect};

mod controls_view;
mod player_view;
mod queue_view;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction},
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::model::Model;

pub fn draw(model: &Model, frame: &mut Frame, area: Rect) {
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(inner_layout[1]);

    let (term_width, term_height) = crossterm::terminal::size().unwrap();
    let term_too_small = term_width < 122 || term_height < 28;
    let queue_panel_constant = if term_too_small {
        vec![Constraint::Percentage(70), Constraint::Percentage(30)]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let left_panel_border = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .split(inner_layout[0]);

    let left_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(queue_panel_constant)
        .margin(2)
        .split(inner_layout[0]);

    queue_view::draw(model, frame, left_panel_chunks[0]);

    Block::bordered()
        .fg(Color::White)
        .title("Player")
        .title_alignment(Alignment::Right)
        .render(main_chunks[0], frame.buffer_mut());

    Block::bordered()
        .fg(Color::White)
        .title("Control")
        .title_alignment(Alignment::Right)
        .render(main_chunks[1], frame.buffer_mut());

    Block::bordered()
        .fg(Color::White)
        .title("Queue")
        .title_alignment(Alignment::Left)
        .render(left_panel_border[0], frame.buffer_mut());

    if term_too_small {
        Block::bordered()
            .fg(Color::White)
            .title("Warning")
            .title_alignment(Alignment::Left)
            .render(left_panel_chunks[1], frame.buffer_mut());

        let warning_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(100)])
            .margin(1)
            .split(left_panel_chunks[1]);

        frame.render_widget(
            Paragraph::new("Your terminal size is too small to display UI properly.")
                .wrap(Wrap { trim: true }),
            warning_chunk[0],
        )
    }

    let player_chunks_const = if model.current_track.has_metadata {
        vec![Constraint::Percentage(60), Constraint::Percentage(40)]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let player_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(player_chunks_const)
        .margin(2)
        .split(main_chunks[0]);

    player_view::draw(model, frame, player_chunks);

    let control_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(2)
        .horizontal_margin(3)
        .split(main_chunks[1]);

    controls_view::draw(frame, control_chunks[0]);
}
