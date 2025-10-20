use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Padding, StatefulWidget},
};

use crate::{global::view_logic::focused_area::FocusedArea, model::Model};

pub fn draw(area: Rect, frame: &mut Frame, model: &mut Model) {
    let queue_entries: Vec<ListItem> = model
        .queue
        .get_ref()
        .iter()
        .map(|t| {
            t.metadata.title.as_deref().unwrap_or(
                t.path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("[Invalid UTF-8 name]"),
            )
        })
        .map(ListItem::from)
        .collect();

    let highlight = if matches!(model.focused_view_area, FocusedArea::Playlist) {
        Style::default() // Because Queue and Player are counted as the same Area, which leaves Playlist.
    } else if model.queue.is_arrange() {
        Style::default().reversed().italic()
    } else {
        Style::default().reversed()
    };

    let list = List::new(queue_entries).highlight_style(highlight);
    let mut list_state = ListState::default();
    list_state.select(model.queue.get_selected());

    let top_title = if model.queue.is_arrange() {
        " Queue [Arrange on] "
    } else {
        " Queue "
    };

    let bottom_title = if matches!(model.focused_view_area, FocusedArea::ControlBar) {
        format!(
            " {} of {} ",
            model.queue.get_selected().unwrap_or(0) + 1,
            model.queue.len()
        )
    } else {
        String::new()
    };

    let block = if matches!(model.focused_view_area, FocusedArea::ControlBar)
        || matches!(model.focused_view_area, FocusedArea::Queue)
    {
        if model.queue.is_empty() {
            Block::default()
        } else {
            Block::bordered()
                .title(top_title)
                .title_bottom(bottom_title)
        }
    } else if model.queue.is_empty() {
        Block::default()
    } else {
        Block::default()
            .title(top_title)
            .title_bottom(bottom_title)
            .padding(Padding::horizontal(1))
    };

    StatefulWidget::render(list.block(block), area, frame.buffer_mut(), &mut list_state);
}
