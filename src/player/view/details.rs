use std::{path::Path, time::Duration};

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

use crate::{
    global::view::center_vertical, model::Model, player::logic::playback_status::PlaybackStatus,
};

pub fn draw(area: Rect, frame: &mut Frame, model: &mut Model) {
    if let Some(ref mut current_track) = model.player.current
        && let Some(tagged_file) = current_track.tagged_file.as_mut()
    {
        let metadata = get_metadata_text_vec(&tagged_file).join("\n");
        frame.render_widget(Paragraph::new(metadata), area);
    }
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
            name.to_string()
        } else {
            "[Invalid UTF-8 name]".to_string()
        }
    } else {
        "[No file name]".to_string()
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
