pub mod terminal;

use std::{rc::Rc, time::Duration};

use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    tag::Accessor,
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Span, ToSpan},
    widgets::{Block, Paragraph, Widget, Wrap},
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

    let (term_width, term_height) = crossterm::terminal::size().unwrap();
    let term_too_small = term_width < 112 || term_height < 28;
    let queue_panel_constant = if term_too_small {
        vec![Constraint::Percentage(70), Constraint::Percentage(30)]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let left_panel_border = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .split(inner_layout[0]);

    let left_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(queue_panel_constant)
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
        .render(left_panel_border[0], frame.buffer_mut());

    if term_too_small {
        Block::bordered()
            .fg(Color::White)
            .title("Warning")
            .title_alignment(Alignment::Left)
            .render(left_panel_chunks[1], frame.buffer_mut());

        let warning_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(100)])
            .margin(1)
            .split(left_panel_chunks[1]);

        frame.render_widget(
            Paragraph::new("Your terminal size is too small to display UI properly.")
                .wrap(Wrap { trim: true }),
            warning_chunk[0],
        )
    }

    let player_chunks_const = if model.current_track.has_metadata {
        vec![Constraint::Percentage(60), Constraint::Percentage(40)]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let player_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(player_chunks_const)
        .margin(2)
        .split(main_chunks[0]);

    draw_player(model, frame, player_chunks);

    let control_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .margin(2)
        .horizontal_margin(3)
        .split(main_chunks[1]);

    draw_controls(frame, control_chunks[0]);
}

fn draw_player(model: &Model, frame: &mut Frame, chunk: Rc<[Rect]>) {
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

    if model.current_track.has_metadata {
        if let Some(tag) = model.current_track.tagged_file.as_ref() {
            let meta_text = get_metadata_text_vec(tag);

            let metadata_margin = 2;

            let metadata_border_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(
                    (metadata_margin * 2) + meta_text.len() as u16,
                )])
                .margin(1)
                .split(chunk[1]);

            Block::bordered()
                .fg(Color::White)
                .title("Metadata")
                .title_alignment(Alignment::Right)
                .render(metadata_border_area[0], frame.buffer_mut());

            let metadata_chunk = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(100)])
                .margin(metadata_margin)
                .split(metadata_border_area[0]);

            let centered_area = center_vertical(metadata_chunk[0], meta_text.len() as u16);

            let meta_para = Paragraph::new(meta_text.join("\n"));

            frame.render_widget(meta_para, centered_area);
        }
    }

    let centered_area = center_vertical(chunk[0], player_text.len() as u16);

    let player_para = Paragraph::new(player_text.join("\n"))
        .centered()
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    frame.render_widget(player_para, centered_area);
}

fn get_metadata_text_vec(tag: &TaggedFile) -> Vec<String> {
    let mut meta_text: Vec<String> = Vec::new();

    let track_num = tag
        .primary_tag()
        .unwrap()
        .track()
        .map(|n| format!("#{} ", n))
        .unwrap_or("".to_string());
    let title = tag
        .primary_tag()
        .unwrap()
        .title()
        .map(|s| format!("{} ", s))
        .unwrap_or("".to_string());
    let artist = tag
        .primary_tag()
        .unwrap()
        .artist()
        .map(|s| format!("{} ", s))
        .unwrap_or("".to_string());
    let album = tag
        .primary_tag()
        .unwrap()
        .album()
        .map(|s| format!("{} ", s))
        .unwrap_or("".to_string());
    let year = tag
        .primary_tag()
        .unwrap()
        .year()
        .map(|n| format!("{} ", n))
        .unwrap_or("".to_string());
    let disc_num = tag
        .primary_tag()
        .unwrap()
        .disk()
        .map(|n| format!("{} ", n))
        .unwrap_or("".to_string());
    let mut bit_depth = tag
        .properties()
        .bit_depth()
        .map(|s| format!("{}-bit/", s))
        .unwrap_or("".to_string());

    if bit_depth.is_empty() {
        bit_depth = "16-bit/".to_string();
    }

    let sample_rate = tag
        .properties()
        .sample_rate()
        .map(|n| format!("{}kHz ", n / 1000))
        .unwrap_or("".to_string());

    let bitrate = tag
        .properties()
        .audio_bitrate()
        .map(|n| format!("{}kbps ", n))
        .unwrap_or("".to_string());

    let lines = vec![
        format!("{}{}", track_num, title),
        format!("{}", artist),
        format!("{}{}{}", album, disc_num, year),
        format!("{}{}{}", bit_depth, sample_rate, bitrate),
    ];

    if let Some((last, rest)) = lines.split_last() {
        for line in rest {
            if !line.is_empty() {
                meta_text.push(line.clone());
                meta_text.push(String::new());
            }
        }
        meta_text.push(last.clone());
    }

    meta_text
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
