use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{List, ListItem, ListState, StatefulWidget},
};

use crate::{model::Model, player::logic::Player};

pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
    let queue_entries: Vec<ListItem> = model
        .player
        .queue
        .get()
        .iter()
        .map(|t| t.title.clone())
        .into_iter()
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

    StatefulWidget::render(list, area, frame.buffer_mut(), &mut list_state);
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
