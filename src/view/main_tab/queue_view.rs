use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::Paragraph,
};

use crate::{logic::player::Player, model::Model};

pub fn draw(model: &Model, frame: &mut Frame, area: Rect) {
    let queued_tracks = get_queued_tracks(&model.player);
    let prev_tracks = get_previous_tracks(&model.player);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1);
            prev_tracks.len() + queued_tracks.len()
        ])
        .split(area);

    let mut on_select = Style::default().add_modifier(Modifier::ITALIC);

    if model.player.queue.is_arrange() {
        on_select = on_select.add_modifier(Modifier::UNDERLINED);
    }

    for (idx, track) in prev_tracks.iter().enumerate() {
        frame.render_widget(
            Span::styled(
                track.clone(),
                Style::default().add_modifier(Modifier::CROSSED_OUT),
            ),
            chunks[idx],
        );
    }

    for (idx, track) in queued_tracks.iter().enumerate() {
        if model.player.queue.get_selected() == idx {
            frame.render_widget(
                Span::styled(track.clone(), on_select),
                chunks[prev_tracks.len() + idx],
            );
        }
        frame.render_widget(
            Paragraph::new(track.clone()),
            chunks[prev_tracks.len() + idx],
        );
    }
}

fn get_previous_tracks(player: &Player) -> Vec<String> {
    let mut tracks: Vec<String> = Vec::new();
    for track in &player.previous {
        if let Some(track_name) = track.file_name().unwrap().to_str() {
            tracks.push(track_name.to_string());
        } else {
            tracks.push("[Invalid UTF-8 name]".into());
        }
    }

    tracks
}

fn get_queued_tracks(player: &Player) -> Vec<String> {
    let mut tracks: Vec<String> = Vec::new();
    for track in player.queue.get() {
        if let Some(track_name) = track.file_name().unwrap().to_str() {
            tracks.push(track_name.to_string());
        } else {
            tracks.push("[Invalid UTF-8 name]".into());
        }
    }

    tracks
}
