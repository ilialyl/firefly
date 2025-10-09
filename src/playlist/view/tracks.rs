use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{
        List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
    },
};

use crate::{
    global::view_logic::focused_area::FocusedArea, model::Model,
    playlist::logic::playlist_tab_focus::PlaylistTabFocus,
};

pub fn draw(model: &mut Model, frame: &mut Frame, entries_area: Rect, scrollbar_area: Rect) {
    let tab_focus = model.playlist_ctl.tab_focus;

    if let Some(selected_playlist) = model.playlist_ctl.get_selected_playlist() {
        let track_entries: Vec<ListItem> = selected_playlist
            .tracks
            .iter()
            .map(|e| {
                e.file_stem()
                    .and_then(|os| os.to_str())
                    .unwrap_or("[Invalid UTF-8 name]")
            })
            .map(ListItem::from)
            .collect();

        let highlight = if matches!(tab_focus, PlaylistTabFocus::Tracks) {
            Style::default().reversed()
        } else {
            Style::default()
        };

        let list = List::new(track_entries).highlight_style(highlight);

        let mut list_state = ListState::default();
        list_state.select(selected_playlist.selected_track);

        StatefulWidget::render(list, entries_area, frame.buffer_mut(), &mut list_state);

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
