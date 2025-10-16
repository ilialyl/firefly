use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{
        Cell, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Table,
        TableState,
    },
};

use crate::{
    global::view_logic::focused_area::FocusedArea, model::Model,
    playlist::logic::playlist_tab_focus::PlaylistTabFocus,
};

pub fn draw(model: &mut Model, frame: &mut Frame, entries_area: Rect, scrollbar_area: Rect) {
    let tab_focus = model.playlist_ctl.tab_focus;

    let header = ["Title", "Artist"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>();

    if let Some(selected_playlist) = model.playlist_ctl.get_selected_playlist() {
        let track_entries: Vec<Row> = selected_playlist
            .metadata_caches
            .iter()
            .map(|m| {
                Row::new(vec![
                    m.title
                        .clone()
                        .unwrap_or(m.file_stem.clone().unwrap_or("Unknown".to_string())),
                    m.artist.clone().unwrap_or(String::new()),
                ])
            })
            .collect();

        let widths = [Constraint::Fill(1), Constraint::Percentage(30)];

        let highlight_style = if !matches!(model.focused_view_area, FocusedArea::Playlist) {
            Style::default()
        } else if matches!(tab_focus, PlaylistTabFocus::Tracks) {
            Style::default().reversed()
        } else {
            Style::default()
        };

        let mut table_state = TableState::default();
        table_state.select(selected_playlist.selected_track);
        Table::new(track_entries, widths)
            .header(header)
            .row_highlight_style(highlight_style)
            .render(entries_area, frame.buffer_mut(), &mut table_state);

        let mut scrollbar_state = ScrollbarState::new(selected_playlist.len())
            .position(selected_playlist.selected_track.unwrap_or(0));

        if matches!(model.focused_view_area, FocusedArea::Playlist) {
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                scrollbar_area,
                &mut scrollbar_state,
            );
        }
    }
}
