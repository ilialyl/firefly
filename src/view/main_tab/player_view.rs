use std::{rc::Rc, time::Duration};

use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    tag::Accessor,
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Widget},
};

use crate::{logic::player, model::Model, view::center_vertical};

pub fn draw(model: &Model, frame: &mut Frame, chunk: Rc<[Rect]>) {
    let player_text = [
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
        .map(|n| format!("({}) ", n))
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

    let lines = [
        format!("{}{}", track_num, title),
        artist.to_string(),
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
    format!("Volume: {}%", (model.sink.volume() * 100.00).ceil() as i32)
}

fn get_info_str(model: &Model) -> String {
    match model.info.last() {
        Some(str) => str.clone(),
        None => "".into(),
    }
}

fn track_pos_as_str(model: &Model) -> String {
    let track_pos = model.current_track.pos.unwrap_or(Duration::from_secs(0));
    let sec = track_pos.as_secs() % 60;
    let min = track_pos.as_secs() / 60;

    format!("{:02}:{:02}", min, sec)
}
