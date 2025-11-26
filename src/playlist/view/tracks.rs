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
    app::App, global::view::focused_area::FocusedArea,
    playlist::logic::playlist_tab_focus::PlaylistTabFocus,
};

pub fn draw(app: &mut App, frame: &mut Frame, entries_area: Rect, scrollbar_area: Rect) {
    let tab_focus = app.playlist_ctl.tab_focus;

    let header = ["Title", "Artist"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>();

    let is_arrange = app.playlist_ctl.arrange_mode;

    if let Some(selected_playlist) = app.playlist_ctl.get_selected_playlist() {
        let track_entries: Vec<Row> = selected_playlist
            .mini_tracks
            .iter()
            .map(|t| {
                if let Some(m) = t.borrow().metadata.as_ref() {
                    Row::new(vec![
                        m.title
                            .clone()
                            .unwrap_or(m.file_stem.clone().unwrap_or_default()),
                        m.artist.clone().unwrap_or_default(),
                    ])
                } else {
                    Row::new(vec![
                        t.borrow()
                            .path
                            .as_path()
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .unwrap_or("[Invalid UTF-8 name]")
                            .to_string(),
                        String::new(),
                    ])
                }
            })
            .collect();

        let widths = [Constraint::Fill(1), Constraint::Percentage(30)];

        let highlight_style = if !matches!(app.focused_view_area, FocusedArea::Playlist) {
            Style::default()
        } else if matches!(tab_focus, PlaylistTabFocus::Tracks) && is_arrange {
            Style::default().reversed().bold()
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

        if matches!(app.focused_view_area, FocusedArea::Playlist) {
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
