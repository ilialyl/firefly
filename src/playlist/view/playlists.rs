use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{List, ListItem, ListState, StatefulWidget},
};

use crate::{
    app::App, global::view::focused_area::FocusedArea,
    playlist::logic::playlist_tab_focus::PlaylistTabFocus,
};

pub fn draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let playlists = app.playlist_ctl.playlist_coll.get_playlists();

    let playlist_entries: Vec<ListItem> = playlists
        .iter()
        .map(|p| p.get_name().unwrap_or("New Playlist"))
        .map(ListItem::from)
        .collect();

    let highlight = if !matches!(app.focused_view_area, FocusedArea::Playlist) {
        Style::default()
    } else if matches!(app.playlist_ctl.tab_focus, PlaylistTabFocus::Playlists) {
        Style::default().reversed()
    } else {
        Style::default().reversed().italic()
    };

    let list = List::new(playlist_entries).highlight_style(highlight);

    let mut list_state = ListState::default();
    list_state.select(app.playlist_ctl.selected_playlist);

    StatefulWidget::render(list, area, frame.buffer_mut(), &mut list_state);
}
