use std::time::Duration;

use lofty::file::{AudioFile, TaggedFile};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Paragraph,
};

use crate::{model::Model, player::logic::playback_status::PlaybackStatus};

pub fn draw(area: Rect, frame: &mut Frame, model: &mut Model) {
    let play_str = match model.player.status {
        PlaybackStatus::Playing => "Playing",
        PlaybackStatus::Idle => "Idle",
        PlaybackStatus::Paused => "Paused",
    };

    let quality_str = if let Some(current_track) = model.player.current.as_mut()
        && let Some(tagged_file) = current_track.tagged_file.as_mut()
    {
        get_quality_str(tagged_file)
    } else {
        String::from("              ")
    };

    let duration_str = if let Some(current_track) = model.player.current.as_mut() {
        format!(
            "{} / {}",
            duration_as_str(&current_track.pos),
            duration_as_str(&current_track.duration.unwrap_or(Duration::from_secs(0)))
        )
    } else {
        String::from("00:00 / 00:00")
    };

    let misc_str = format!(
        "{} {} {} {}",
        duration_str,
        if model.player.looping { "↻" } else { " " },
        get_volume_str(model.player.sink.volume()),
        quality_str,
    );

    let area = Layout::horizontal(vec![
        Constraint::Length(10),
        Constraint::Fill(1),
        Constraint::Length((misc_str.len() + 2) as u16),
    ])
    .flex(Flex::SpaceAround)
    .spacing(2)
    .split(area);

    let mut progress_str = "─".repeat(area[1].width as usize);
    if let Some(current_track) = model.player.current.as_mut()
        && let Some(dur) = current_track.duration
    {
        let bar_pos = get_progress_position(progress_str.chars().count(), &current_track.pos, &dur)
            .saturating_sub(1);
        if let Some((byte_pos, ch)) = progress_str.char_indices().nth(bar_pos) {
            let byte_end = byte_pos + ch.len_utf8();
            progress_str.replace_range(byte_pos..byte_end, "⚬");
        }
    }

    let play = Paragraph::new(play_str).centered();
    let progress = Paragraph::new(progress_str);
    let misc = Paragraph::new(misc_str).centered();

    frame.render_widget(play, area[0]);
    frame.render_widget(progress, area[1]);
    frame.render_widget(misc, area[2]);
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

fn get_quality_str(tag: &TaggedFile) -> String {
    let mut bit_depth = tag
        .properties()
        .bit_depth()
        .map(|s| format!("{}-bit", s))
        .unwrap_or("".to_string());

    if bit_depth.is_empty() {
        bit_depth = "16-bit".to_string();
    }

    let sample_rate = tag
        .properties()
        .sample_rate()
        .map(|n| format!("{}kHz", n / 1000))
        .unwrap_or("".to_string());

    format!("{}/{}", bit_depth, sample_rate)
}

fn get_progress_position(bar_size: usize, track_pos: &Duration, track_dur: &Duration) -> usize {
    (track_pos.div_duration_f32(*track_dur) * (bar_size as f32)).ceil() as usize
}
