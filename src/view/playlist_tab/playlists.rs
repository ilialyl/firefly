use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Styled},
    text::Line,
    widgets::Paragraph,
};

use crate::{logic::playlist::playlist_tab_focus::PlaylistTabFocus, model::Model};

pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
    let playlists = model.playlist_ctl.playlist_coll.get_playlists();

    let mut name_lines: Vec<Line> = playlists
        .iter()
        .map(|p| p.get_name().unwrap_or("New Playlist".to_string()))
        .map(|n| Line::from(n))
        .collect();

    if matches!(model.playlist_ctl.tab_focus, PlaylistTabFocus::Playlists) {
        if let Some(selected_playlist) = model.playlist_ctl.selected_playlist {
            name_lines[selected_playlist] = name_lines[selected_playlist]
                .clone()
                .set_style(Style::default().fg(Color::Rgb(255, 192, 15)))
        }
    }

    let paragraph = Paragraph::new(name_lines);

    frame.render_widget(paragraph, area);
}
