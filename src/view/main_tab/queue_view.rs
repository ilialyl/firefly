use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::Paragraph,
};

use crate::model::Model;

fn get_queued_tracks(model: &Model) -> Vec<String> {
    let mut tracks: Vec<String> = Vec::new();
    for track in model.track_queue.queue() {
        if let Some(track_name) = track.file_name().unwrap().to_str() {
            tracks.push(track_name.to_string());
        } else {
            tracks.push("[Invalid UTF-8 name]".into());
        }
    }

    tracks
}

pub fn draw(model: &Model, frame: &mut Frame, area: Rect) {
    let queued_tracks = get_queued_tracks(model);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); queued_tracks.len()])
        .split(area);

    let mut on_select = Style::default().add_modifier(Modifier::ITALIC);

    if model.track_queue.is_arrange() {
        on_select = on_select.add_modifier(Modifier::UNDERLINED);
    }

    for (idx, track) in queued_tracks.iter().enumerate() {
        if model.track_queue.get_selected() == idx {
            frame.render_widget(Span::styled(track.clone(), on_select), chunks[idx]);
        }
        frame.render_widget(Paragraph::new(track.clone()), chunks[idx]);
    }
}
