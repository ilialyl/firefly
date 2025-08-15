use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::ToSpan,
    widgets::{Block, Paragraph, Widget},
};

use crate::{app::App, player::Status};

pub fn render(app: &App, frame: &mut Frame) {
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

    frame.render_widget(get_queue_para(app), left_panel_chunks[0]);

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

    draw_player(app, frame, player_chunks[0]);

    let control_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(2)
        .split(main_chunks[1]);

    draw_control(frame, control_chunks[0]);
}

fn draw_player(app: &App, frame: &mut Frame, chunk: Rect) {
    let player_text = vec![
        get_track_name_str(app),
        "".into(),
        get_track_pos_str(app),
        "".into(),
        get_status_str(app),
        get_loop_status_str(app),
        get_info_str(app),
        get_volume_str(app),
    ];

    let area = center_vertical(chunk, player_text.len() as u16);

    let player_para = Paragraph::new(player_text.join("\n"))
        .centered()
        .alignment(Alignment::Center);

    frame.render_widget(player_para, area);
}

fn draw_control(frame: &mut Frame, chunk: Rect) {
    let controls = vec![
        "Play/Pause <Space>",
        "Load Now <N>",
        "Queue <Q>",
        "Queue Folder <D>",
        "Skip <S>",
        "Rewind/Forward <Left/Right>",
        "Volume <Up/Down>",
        "Loop <L>",
        "Quit <Esc>",
    ];

    let area = center_vertical(chunk, controls.len() as u16);

    let control_para = Paragraph::new(controls.join("\n"));

    frame.render_widget(control_para, area);
}

fn get_track_name_str(app: &App) -> String {
    match app.track_path.clone() {
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

fn get_track_pos_str(app: &App) -> String {
    app.track_pos_as_str()
}

fn get_status_str(app: &App) -> String {
    match app.status {
        Status::Playing => "Playing".into(),
        Status::Paused => ("Paused").into(),
        Status::Idle => ("Idle").into(),
    }
}

fn get_loop_status_str(app: &App) -> String {
    match app.looping {
        true => "[Looped]".into(),
        false => "".into(),
    }
}

fn get_volume_str(app: &App) -> String {
    format!("Volume: {}%", (app.volume * 100.00).ceil() as i32)
}

fn get_info_str(app: &App) -> String {
    match app.info.last() {
        Some(str) => str.clone(),
        None => "".into(),
    }
}

fn get_queue_para(app: &App) -> Paragraph<'static> {
    let mut track_vec: Vec<String> = Vec::new();
    for track in app.track_queue.clone() {
        if let Some(track_name) = track.file_name().unwrap().to_str() {
            track_vec.push(track_name.to_string());
        } else {
            track_vec.push("[Invalid UTF-8 name]".into());
        }
    }

    let tracks_str = track_vec.join("\n");

    Paragraph::new(tracks_str)
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}
