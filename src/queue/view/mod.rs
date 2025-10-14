use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Padding, StatefulWidget},
};

use crate::{global::view_logic::focused_area::FocusedArea, model::Model, player::logic::Player};

pub fn draw(area: Rect, frame: &mut Frame, model: &mut Model) {
    let queue_entries: Vec<ListItem> = model
        .player
        .queue
        .get()
        .iter()
        .map(|t| {
            t.metadata.title.clone().unwrap_or(
                t.path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("[Invalid UTF-8 name]")
                    .to_string(),
            )
        })
        .map(ListItem::from)
        .collect();

    let highlight = if model.player.queue.is_arrange() {
        Style::default().reversed().italic()
    } else {
        Style::default().reversed()
    };

    let list = List::new(queue_entries).highlight_style(highlight);
    let mut list_state = ListState::default();
    list_state.select(Some(model.player.queue.get_selected()));

    let top_title = if model.player.queue.is_arrange() {
        " Queue [Arrange on] "
    } else {
        " Queue "
    };

    let bottom_title = if matches!(model.focused_view_area, FocusedArea::ControlBarAndQueue) {
        format!(
            " {} of {} ",
            model.player.queue.get_selected() + 1,
            model.player.queue.len()
        )
    } else {
        String::new()
    };

    let block = if matches!(model.focused_view_area, FocusedArea::ControlBarAndQueue) {
        if model.player.queue.is_empty() {
            Block::default()
        } else {
            Block::bordered()
                .title(top_title)
                .title_bottom(bottom_title)
        }
    } else if model.player.queue.is_empty() {
        Block::default()
    } else {
        Block::default()
            .title(top_title)
            .title_bottom(bottom_title)
            .padding(Padding::horizontal(1))
    };

    StatefulWidget::render(list.block(block), area, frame.buffer_mut(), &mut list_state);
}

fn _get_previous_tracks(player: &Player) -> Vec<String> {
    let mut tracks: Vec<String> = Vec::new();
    player.previous.iter().for_each(|track| {
        if let Some(track_name) = track.file_name().and_then(|os| os.to_str()) {
            tracks.push(track_name.to_string());
        } else {
            tracks.push("[Invalid UTF-8 name]".into());
        }
    });

    tracks
}
