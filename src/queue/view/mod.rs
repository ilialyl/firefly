use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Padding, StatefulWidget},
};

use crate::{app::App, global::view::focused_area::FocusedArea};

pub fn draw(area: Rect, frame: &mut Frame, app: &mut App) {
    let queue_entries: Vec<ListItem> = app
        .queue
        .get_ref()
        .iter()
        .map(|t| {
            if let Some(metadata) = t.borrow().metadata.as_ref() {
                metadata.title.clone().unwrap_or(
                    t.borrow()
                        .path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("[Invalid UTF-8 name]")
                        .to_string(),
                )
            } else {
                t.borrow()
                    .path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("[Invalid UTF-8 name]")
                    .to_string()
            }
        })
        .map(ListItem::from)
        .collect();

    let highlight = if matches!(app.focused_view_area, FocusedArea::Playlist) {
        Style::default() // Because Queue and Player are counted as the same Area, which leaves Playlist.
    } else if app.queue.is_arrange() {
        Style::default().reversed().italic()
    } else {
        Style::default().reversed()
    };

    let list = List::new(queue_entries).highlight_style(highlight);
    let mut list_state = ListState::default();
    list_state.select(app.queue.get_selected());

    let top_title = if app.queue.is_arrange() {
        " Queue [Arrange on] "
    } else {
        " Queue "
    };

    let bottom_title = if matches!(app.focused_view_area, FocusedArea::ControlBar) {
        format!(
            " {} of {} ",
            app.queue.get_selected().unwrap_or(0) + 1,
            app.queue.len()
        )
    } else {
        String::new()
    };

    let block = if matches!(app.focused_view_area, FocusedArea::ControlBar)
        || matches!(app.focused_view_area, FocusedArea::Queue)
    {
        if app.queue.is_empty() {
            Block::default()
        } else {
            Block::bordered()
                .title(top_title)
                .title_bottom(bottom_title)
        }
    } else if app.queue.is_empty() {
        Block::default()
    } else {
        Block::default()
            .title(top_title)
            .title_bottom(bottom_title)
            .padding(Padding::horizontal(1))
    };

    StatefulWidget::render(list.block(block), area, frame.buffer_mut(), &mut list_state);
}
