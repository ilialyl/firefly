pub mod playlists;
pub mod tracks;

use ratatui::layout::{Layout, Rect};

use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction},
    widgets::{Block, Widget},
};

use crate::global::view_logic::focused_area::FocusedArea;
use crate::model::Model;

pub fn draw(area: Rect, frame: &mut Frame, model: &mut Model) {
    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area);

    let (playlists_block, tracks_block) =
        if matches!(model.focused_view_area, FocusedArea::Playlist) {
            (Block::bordered(), Block::bordered())
        } else {
            (Block::new(), Block::new())
        };

    playlists_block
        .title(Line::style(Line::from("Playlists"), Style::new()))
        .border_style(Style::default())
        .title_alignment(Alignment::Left)
        .render(inner_chunks[0], frame.buffer_mut());

    tracks_block
        .title(Line::style(Line::from("Tracks"), Style::new()))
        .title_alignment(Alignment::Right)
        .border_style(Style::default())
        .render(inner_chunks[1], frame.buffer_mut());

    let left_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(2)
        .split(inner_chunks[0]);

    let right_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(2)
        .split(inner_chunks[1]);

    // draw_mini_controls(frame, outer_chunks[1]);
    playlists::draw(model, frame, left_panel_chunks[0]);
    tracks::draw(model, frame, right_panel_chunks[0], inner_chunks[2]);
}
