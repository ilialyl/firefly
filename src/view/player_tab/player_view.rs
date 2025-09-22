use std::{path::Path, rc::Rc, time::Duration};

use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    tag::Accessor,
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::{logic::player::playback_status::PlaybackStatus, model::Model, view::center_vertical};

pub fn draw(model: &mut Model, frame: &mut Frame, chunk: Rc<[Rect]>) {
    let player_text: Vec<String>;

    if let Some(ref mut current_track) = model.player.current {
        // let path = current_track.real_path.clone();
        // let pos = current_track.pos.clone();
        // let dur = current_track.duration.clone();
        // let status = model.player.status.clone();
        // let looping = model.player.looping;

        player_text = vec![
            get_track_name_str(&current_track.real_path),
            String::new(),
            format!(
                "{} / {}",
                duration_as_str(&current_track.pos),
                duration_as_str(&current_track.duration)
            ),
            String::new(),
            get_status_str(&model.player.status),
            get_loop_status_str(&model.player.looping),
            model.info_display.clone(),
            get_volume_str(model.player.sink.volume()),
        ];

        if current_track.has_title {
            let meta_text = get_metadata_text_vec(&current_track.tagged_file);

            let metadata_margin = 2;

            let metadata_border_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(
                    (metadata_margin * 2) + meta_text.len() as u16,
                )])
                .margin(1)
                .split(chunk[1]);

            Block::bordered()
                .title(Line::style(Line::from("Metadata"), Style::new()))
                .border_style(Style::default())
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
    } else {
        player_text = vec![
            "[Empty]".to_string(),
            String::new(),
            "0:00 / 0:00".to_string(),
            String::new(),
            get_status_str(&model.player.status),
            get_loop_status_str(&model.player.looping),
            model.info_display.clone(),
            get_volume_str(model.player.sink.volume()),
        ];
    }

    let centered_area = center_vertical(chunk[0], player_text.len() as u16);

    let player_para = Paragraph::new(player_text.join("\n"))
        .centered()
        .alignment(Alignment::Center)
        .style(Style::new());

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

fn get_track_name_str(path: &Path) -> String {
    if let Some(os_name) = path.file_name() {
        if let Some(name) = os_name.to_str() {
            return name.to_string();
        } else {
            return "[Invalid UTF-8 name]".to_string();
        }
    } else {
        return "[No file name]".to_string();
    }
}

fn get_status_str(player_status: &PlaybackStatus) -> String {
    match player_status {
        PlaybackStatus::Playing => "Playing".into(),
        PlaybackStatus::Paused => ("Paused").into(),
        PlaybackStatus::Idle => ("Idle").into(),
    }
}

fn get_loop_status_str(loop_status: &bool) -> String {
    match loop_status {
        true => "[Looped]".into(),
        false => "".into(),
    }
}

fn get_volume_str(volume: f32) -> String {
    format!("Volume: {}%", (volume * 100.00).ceil() as i32)
}

fn duration_as_str(dur: &Duration) -> String {
    let sec = dur.as_secs() % 60;
    let min = (dur.as_secs() / 60) % 60;
    let hour = dur.as_secs() / 3600;

    if hour > 0 {
        format!("{:02}:{:02}:{:02}", hour, min, sec)
    } else {
        format!("{:02}:{:02}", min, sec)
    }
}
