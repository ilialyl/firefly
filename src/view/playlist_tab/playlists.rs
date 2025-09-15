use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

use crate::model::Model;

pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
    let playlists = model
        .playlist_controller
        .playlist_collection
        .get_playlists();

    let name_lines: Vec<Line> = playlists
        .iter()
        .map(|p| p.get_name().unwrap_or("New Playlist".to_string()))
        .map(|n| Line::from(n))
        .collect();

    let paragraph = Paragraph::new(name_lines);

    frame.render_widget(paragraph, area);
}
