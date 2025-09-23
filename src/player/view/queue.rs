use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::Paragraph,
};

use crate::{model::Model, player::logic::Player};

pub struct QueueViewState {
    pub scroll_offset: usize,
    pub area_height: Option<usize>,
}

impl Default for QueueViewState {
    fn default() -> Self {
        QueueViewState {
            scroll_offset: 0,
            area_height: None,
        }
    }
}

pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
    let queued_tracks = get_queued_tracks(&model.player);

    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .split(area);

    let mut lines: Vec<Line> = Vec::new();

    // Code for displaying previous tracks
    // let prev_tracks = get_previous_tracks(&model.player);
    // for track in prev_tracks {
    //     lines.push(Line::from(Line::styled(
    //         format!(" {}", track),
    //         Style::default().add_modifier(Modifier::CROSSED_OUT),
    //     )));
    // }

    let mut on_select = Style::default().fg(Color::Rgb(255, 192, 15));
    if model.player.queue.is_arrange() {
        on_select = on_select.add_modifier(Modifier::BOLD);
    }

    for (idx, track) in queued_tracks.into_iter().enumerate() {
        let text = format!(" {}", track);
        if model.player.queue.get_selected() == idx {
            lines.push(Line::from(text.clone()).style(on_select));
        } else {
            lines.push(Line::from(text).style(Style::new()));
        }
    }

    let paragraph = Paragraph::new(lines).scroll((model.queue_view.scroll_offset as u16, 0));

    frame.render_widget(paragraph, chunk[0]);
    model.queue_view.area_height = Some(chunk[0].height as usize);
}

fn _get_previous_tracks(player: &Player) -> Vec<String> {
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
