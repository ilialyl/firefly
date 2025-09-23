use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Styled},
    text::Line,
    widgets::Paragraph,
};

use crate::{model::Model, playlist::logic::playlist_tab_focus::PlaylistTabFocus};

pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
    let tab_focus = model.playlist_ctl.tab_focus;

    if let Some(selected_playlist) = model.playlist_ctl.get_selected_playlist() {
        let mut name_lines: Vec<Line> = selected_playlist
            .tracks
            .iter()
            .map(|e| {
                e.file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap_or("[Invalid UTF-8 name]")
            })
            .map(|n| Line::from(n))
            .collect();

        if matches!(tab_focus, PlaylistTabFocus::Tracks) {
            if let Some(selected_track_idx) = selected_playlist.selected_track {
                name_lines[selected_track_idx] = name_lines[selected_track_idx]
                    .clone()
                    .set_style(Style::default().fg(Color::Rgb(255, 192, 15)))
            }
        }

        let paragraph =
            Paragraph::new(name_lines).scroll((model.playlist_view.tracks_scroll_offset as u16, 0));

        frame.render_widget(paragraph, area);
    }
}
