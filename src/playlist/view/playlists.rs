use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Styled, Stylize},
    text::Line,
    widgets::Paragraph,
};

use crate::{model::Model, playlist::logic::playlist_tab_focus::PlaylistTabFocus};

pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
    let playlists = model.playlist_ctl.playlist_coll.get_playlists();

    let mut name_lines: Vec<Line> = playlists
        .iter()
        .map(|p| p.get_name().unwrap_or("New Playlist".to_string()))
        .map(|n| Line::from(n))
        .collect();

    let mut selected_playlist_style = Style::default().fg(Color::Rgb(255, 192, 15));

    if !matches!(model.playlist_ctl.tab_focus, PlaylistTabFocus::Playlists) {
        selected_playlist_style = selected_playlist_style.italic();
    };

    if let Some(selected_playlist) = model.playlist_ctl.selected_playlist {
        name_lines[selected_playlist] = name_lines[selected_playlist]
            .clone()
            .set_style(selected_playlist_style)
    }

    let paragraph =
        Paragraph::new(name_lines).scroll((model.playlist_view.playlists_scroll_offset as u16, 0));

    frame.render_widget(paragraph, area);
}
