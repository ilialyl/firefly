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
        .map(|t| t.title.clone())
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

    let title = if model.player.queue.is_arrange() {
        " Queue [Arrange on] "
    } else {
        " Queue "
    };

    let block = if matches!(model.focused_view_area, FocusedArea::ControlBarAndQueue) {
        if model.player.queue.is_empty() {
            Block::default()
        } else {
            Block::bordered().title(title)
        }
    } else {
        Block::default().title(title)
    }
    .padding(Padding::uniform(1));

    StatefulWidget::render(list.block(block), area, frame.buffer_mut(), &mut list_state);
}

fn _get_previous_tracks(player: &Player) -> Vec<String> {
    let mut tracks: Vec<String> = Vec::new();
    player.previous.iter().for_each(|track| {
        if let Some(track_name) = track.file_name().unwrap().to_str() {
            tracks.push(track_name.to_string());
        } else {
            tracks.push("[Invalid UTF-8 name]".into());
        }
    });

    tracks
}
