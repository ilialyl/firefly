use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::Paragraph,
};

use crate::{logic::playback_state::PlaybackState, model::Model};

pub fn draw(model: &Model, frame: &mut Frame, area: Rect) {
    let queued_tracks = get_queued_tracks(&model.playback);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); queued_tracks.len()])
        .split(area);

    let mut on_select = Style::default().add_modifier(Modifier::ITALIC);

    if model.playback.queue.is_arrange() {
        on_select = on_select.add_modifier(Modifier::UNDERLINED);
    }

    for (idx, track) in queued_tracks.iter().enumerate() {
        if model.playback.queue.get_selected() == idx {
            frame.render_widget(Span::styled(track.clone(), on_select), chunks[idx]);
        }
        frame.render_widget(Paragraph::new(track.clone()), chunks[idx]);
    }
}

fn get_queued_tracks(playback_st: &PlaybackState) -> Vec<String> {
    let mut tracks: Vec<String> = Vec::new();
    for track in playback_st.queue.get() {
        if let Some(track_name) = track.file_name().unwrap().to_str() {
            tracks.push(track_name.to_string());
        } else {
            tracks.push("[Invalid UTF-8 name]".into());
        }
    }

    tracks
}
