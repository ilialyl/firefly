use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{List, ListItem, ListState, StatefulWidget},
};

use crate::{
    global::view::focused_area::FocusedArea, model::Model,
    playlist::logic::playlist_tab_focus::PlaylistTabFocus,
};

pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
    let playlists = model.playlist_ctl.playlist_coll.get_playlists();

    let playlist_entries: Vec<ListItem> = playlists
        .iter()
        .map(|p| p.get_name().unwrap_or("New Playlist"))
        .map(ListItem::from)
        .collect();

    let highlight = if !matches!(model.focused_view_area, FocusedArea::Playlist) {
        Style::default()
    } else if matches!(model.playlist_ctl.tab_focus, PlaylistTabFocus::Playlists) {
        Style::default().reversed()
    } else {
        Style::default().reversed().italic()
    };

    let list = List::new(playlist_entries).highlight_style(highlight);

    let mut list_state = ListState::default();
    list_state.select(model.playlist_ctl.selected_playlist);

    StatefulWidget::render(list, area, frame.buffer_mut(), &mut list_state);
}
