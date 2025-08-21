pub mod terminal;

use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Span, ToSpan},
    widgets::{Block, Paragraph, Widget},
};

use crate::model::{Model, player};

pub fn view(model: &Model, frame: &mut Frame) {
    render(model, frame);
}

pub fn render(model: &Model, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .split(frame.area());

    Block::new()
        .fg(Color::White)
        .title("Firefly Player".to_span().into_centered_line())
        .render(outer_layout[0], frame.buffer_mut());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(outer_layout[1]);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(inner_layout[1]);

    let left_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(2)
        .split(inner_layout[0]);

    draw_queue(model, get_queued_tracks(model), frame, left_panel_chunks[0]);

    Block::bordered()
        .fg(Color::White)
        .title("Player")
        .title_alignment(Alignment::Right)
        .render(main_chunks[0], frame.buffer_mut());

    Block::bordered()
        .fg(Color::White)
        .title("Control")
        .title_alignment(Alignment::Right)
        .render(main_chunks[1], frame.buffer_mut());

    Block::bordered()
        .fg(Color::White)
        .title("Queue")
        .title_alignment(Alignment::Left)
        .render(inner_layout[0], frame.buffer_mut());

    let player_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(2)
        .split(main_chunks[0]);

    draw_player(model, frame, player_chunks[0]);

    let control_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(2)
        .horizontal_margin(3)
        .split(main_chunks[1]);

    draw_controls(frame, control_chunks[0]);
}

fn draw_player(model: &Model, frame: &mut Frame, chunk: Rect) {
    let player_text = vec![
        get_track_name_str(model),
        "".into(),
        get_track_pos_str(model),
        "".into(),
        get_status_str(model),
        get_loop_status_str(model),
        get_info_str(model),
        get_volume_str(model),
    ];

    let area = center_vertical(chunk, player_text.len() as u16);

    let player_para = Paragraph::new(player_text.join("\n"))
        .centered()
        .alignment(Alignment::Center);

    frame.render_widget(player_para, area);
}

fn get_queued_tracks(model: &Model) -> Vec<String> {
    let mut tracks: Vec<String> = Vec::new();
    for track in model.track_queue.clone() {
        if let Some(track_name) = track.file_name().unwrap().to_str() {
            tracks.push(track_name.to_string());
        } else {
            tracks.push("[Invalid UTF-8 name]".into());
        }
    }

    tracks
}

fn draw_queue(model: &Model, queued_tracks: Vec<String>, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); queued_tracks.len()])
        .split(area);

    let mut on_select = Style::default().add_modifier(Modifier::ITALIC);

    if model.arrange_mode {
        on_select = on_select.add_modifier(Modifier::UNDERLINED);
    }

    for (idx, track) in queued_tracks.iter().enumerate() {
        if model.selected_track == idx {
            frame.render_widget(Span::styled(track.clone(), on_select), chunks[idx]);
        }
        frame.render_widget(Paragraph::new(track.clone()), chunks[idx]);
    }
}

fn draw_controls(frame: &mut Frame, chunk: Rect) {
    let controls = vec![
        " Play/Pause <Space>",
        " Load Now <N>",
        " Queue <Q>",
        " Queue Dir <ShiftQ>",
        " Skip <S>",
        " Rewind/Seek <←/→>",
        " Volume <=/->",
        " Loop <L>",
        " Quit <Esc>",
    ];

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); 4])
        .spacing(1)
        .split(chunk);

    let grid: Vec<Rect> = rows
        .iter()
        .flat_map(|col| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(25); 4])
                .spacing(1)
                .split(*col)
                .to_vec()
        })
        .collect();

    for (idx, control) in controls.iter().enumerate() {
        frame.render_widget(Paragraph::new(*control), grid[idx])
    }
}

fn get_track_name_str(model: &Model) -> String {
    match model.current_track.path.clone() {
        Some(path) => {
            if let Some(os_name) = path.file_name() {
                if let Some(name) = os_name.to_str() {
                    name.to_string()
                } else {
                    "[Invalid UTF-8 name]".into()
                }
            } else {
                "[No file name]".into()
            }
        }
        None => "[Track Empty]".into(),
    }
}

fn get_track_pos_str(model: &Model) -> String {
    track_pos_as_str(model)
}

fn get_status_str(model: &Model) -> String {
    match model.status {
        player::Status::Playing => "Playing".into(),
        player::Status::Paused => ("Paused").into(),
        player::Status::Idle => ("Idle").into(),
    }
}

fn get_loop_status_str(model: &Model) -> String {
    match model.looping {
        true => "[Looped]".into(),
        false => "".into(),
    }
}

fn get_volume_str(model: &Model) -> String {
    format!("Volume: {}%", (model.volume * 100.00).ceil() as i32)
}

fn get_info_str(model: &Model) -> String {
    match model.info.last() {
        Some(str) => str.clone(),
        None => "".into(),
    }
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn track_pos_as_str(model: &Model) -> String {
    let track_pos = model
        .current_track
        .pos
        .clone()
        .unwrap_or(Duration::from_secs(0));
    let sec = track_pos.as_secs() % 60;
    let min = track_pos.as_secs() / 60;

    format!("{:02}:{:02}", min, sec)
}

pub fn stop_info_display(model: &mut Model) {
    model.info.push(String::new());
}

pub fn display_info(model: &mut Model, info: &str) {
    model.info.push(info.to_string());
}
