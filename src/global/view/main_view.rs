use ratatui::layout::{Layout, Rect};
use ratatui::{
    Frame,
    layout::{Constraint, Direction},
};

use crate::model::Model;
use crate::player::view::{cover_art, queue, track_details};
use crate::playlist;

pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    let left_panel = panels[0];
    let right_panel = panels[1];

    let left_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(left_panel.width / 2),
            Constraint::Min(0),
        ])
        .split(left_panel);

    let right_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(40), Constraint::Fill(1)])
        .split(right_panel);

    let track_details_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(1)
        .split(right_panel_chunks[0]);

    let cover_art_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(1)
        .split(left_panel_chunks[0]);

    cover_art::draw(cover_art_area[0], frame, model);
    queue::draw(left_panel_chunks[1], frame, model);
    track_details::draw(track_details_area[0], frame, model);
    playlist::view::draw(right_panel_chunks[1], frame, model);
}
