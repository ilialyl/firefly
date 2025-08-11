use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Text, ToSpan},
    widgets::{Block, Widget},
};

use crate::{app::App, player::Status};

// impl Widget for &app::App {
//     fn render(self, area: Rect, buf: &mut Buffer) {}
// }

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
        .constraints(vec![Constraint::Percentage(10); 10])
        .margin(2)
        .split(inner_layout[0]);

    // let todo_text = Text::from("Work in progress.").centered();
    // frame.render_widget(todo_text, left_panel_chunks[0]);

    draw_queue(frame, left_panel_chunks, get_queue_text(app));

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
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .margin(2)
        .split(main_chunks[0]);

    draw_player(app, frame, player_chunks);

    let control_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); 6])
        .margin(2)
        .split(main_chunks[1]);

    draw_control(frame, control_chunks);
}

fn draw_player(app: &App, frame: &mut Frame, chunks: Rc<[Rect]>) {
    let track_name = get_track_name_text(&app).centered();
    let track_pos = get_track_pos_text(&app).centered();
    let status = get_status_text(&app).centered();
    let loop_status = get_loop_status_text(&app).centered();
    let volume = get_volume_text(&app).centered();

    frame.render_widget(track_name, chunks[0]);
    frame.render_widget(track_pos, chunks[1]);
    frame.render_widget(status, chunks[2]);
    frame.render_widget(loop_status, chunks[3]);
    frame.render_widget(volume, chunks[4]);
}

fn draw_control(frame: &mut Frame, chunks: Rc<[Rect]>) {
    let controls = vec![
        Line::from("Load <Enter>"),
        Line::from("Play/Pause <Space>"),
        Line::from("Rewind/Forward <Left/Right>"),
        Line::from("Volume <Up/Down>"),
        Line::from("Loop <L>"),
        Line::from("Quit <Esc>"),
    ];

    for (idx, line) in controls.iter().enumerate() {
        frame.render_widget(line, chunks[idx]);
    }
}

fn draw_queue(frame: &mut Frame, chunks: Rc<[Rect]>, text_vec: Vec<Text<'static>>) {
    for (idx, text) in text_vec.iter().enumerate() {
        frame.render_widget(text, chunks[idx]);
    }
}

fn get_track_name_text(app: &App) -> Text<'static> {
    match app.track_path.clone() {
        Some(path) => {
            if let Some(os_name) = path.file_name() {
                if let Some(name) = os_name.to_str() {
                    Text::from(name.to_string())
                } else {
                    Text::from("[Invalid UTF-8 name]")
                }
            } else {
                Text::from("[No file name]")
            }
        }
        None => Text::from("[Track Empty]"),
    }
}

fn get_track_pos_text(app: &App) -> Text<'static> {
    Text::from(app.track_pos_as_str())
}

fn get_status_text(app: &App) -> Text<'static> {
    match app.status {
        Status::Playing => Text::from("Playing"),
        Status::Paused => Text::from("Paused"),
        Status::Idle => Text::from("Idle"),
    }
}

fn get_loop_status_text(app: &App) -> Text<'static> {
    match app.looping {
        true => Text::from("[Looped]"),
        false => Text::from(""),
    }
}

fn get_volume_text(app: &App) -> Text<'static> {
    Text::from(format!("Volume: {}%", (app.volume * 100.00).ceil() as i32))
}

fn get_queue_text(app: &App) -> Vec<Text<'static>> {
    let mut text_vec: Vec<Text<'static>> = Vec::new();
    for track in app.track_queue.clone() {
        let text: Text;
        if let Some(track_name) = track.file_name().unwrap().to_str() {
            text = Text::from(track_name.to_string());
        } else {
            text = Text::from("[Invalid UTF-8 name]");
        }
        text_vec.push(text);
    }
    text_vec
}
