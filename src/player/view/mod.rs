pub mod controls;
pub mod cover_art;
pub mod now_playing;
pub mod queue;

use ratatui::layout::{Layout, Rect};
use ratatui::{
    Frame,
    layout::{Constraint, Direction},
};

use crate::model::Model;

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
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .margin(2)
        .split(right_panel);

    let cover_art_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(1)
        .split(left_panel_chunks[0]);

    cover_art::draw(cover_art_area[0], frame, model);
    queue::draw(left_panel_chunks[1], frame, model);
    now_playing::draw(right_panel_chunks[0], frame, model);
    controls::draw(right_panel_chunks[1], frame, model);
}
