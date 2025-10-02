use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Padding, StatefulWidget},
};

use crate::{model::Model, player::logic::Player};

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

    StatefulWidget::render(
        list.block(Block::default().title("Queue").padding(Padding::uniform(1))),
        area,
        frame.buffer_mut(),
        &mut list_state,
    );
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
