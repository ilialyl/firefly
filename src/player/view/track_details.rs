use std::path::Path;

use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    tag::Accessor,
};
use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::{global::view::center_vertical, model::Model};

pub fn draw(area: Rect, frame: &mut Frame, model: &mut Model) {
    if let Some(current_track) = model.player.current.as_mut()
        && let Some(tagged_file) = current_track.tagged_file.as_mut()
    {
        let metadata = get_metadata_lines(tagged_file, &current_track.real_path);
        frame.render_widget(
            Paragraph::new(metadata.join("\n")),
            center_vertical(area, (metadata.len() + 1) as u16),
        );
    } else {
        let text = "Press Q to open file dialog to queue files.\n\nAlternatively, run \"firefly with <path>\" or \"cargo run -r -- with <path>\".\n\nPress H to view all keybinds.";
        frame.render_widget(
            Paragraph::new(text),
            center_vertical(area, text.lines().count() as u16),
        );
    }
}

fn get_metadata_lines(tagged_file: &TaggedFile, path: &Path) -> Vec<String> {
    if let Some(tag) = tagged_file.primary_tag() {
        let properties = tagged_file.properties();

        let track_num = tag
            .track()
            .map(|n| format!("#{} ", n))
            .unwrap_or("".to_string());
        let title = tag.title().map(|s| format!("{} ", s)).unwrap_or(
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("No Title")
                .to_string(),
        );
        let artist = tag
            .artist()
            .map(|s| format!("{} ", s))
            .unwrap_or("".to_string());
        let album = tag
            .album()
            .map(|s| format!("{} ", s))
            .unwrap_or("".to_string());
        let year = tag
            .year()
            .map(|n| format!("({}) ", n))
            .unwrap_or("".to_string());
        let disc_num = tag
            .disk()
            .map(|n| format!("{} ", n))
            .unwrap_or("".to_string());
        let bit_depth = properties
            .bit_depth()
            .map(|s| format!("{}-bit/", s))
            .unwrap_or("16-bit/".to_string());

        let sample_rate = properties
            .sample_rate()
            .map(|n| format!("{}kHz ", n / 1000))
            .unwrap_or("".to_string());

        let bitrate = properties
            .audio_bitrate()
            .map(|n| format!("{}kbps ", n))
            .unwrap_or("".to_string());

        let channels = properties
            .channels()
            .map(|n| format!("{} Channels", n))
            .unwrap_or("".to_string());

        let file_ext = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string()
            .to_uppercase();

        let vec = [
            format!("{}{}", track_num, title),
            artist.to_string(),
            format!("{}{}{}", album, disc_num, year),
            format!(
                "{} {}{}{}{}",
                file_ext, bit_depth, sample_rate, bitrate, channels
            ),
        ];

        let mut padded_vec = Vec::<String>::new();
        if let Some((last, rest)) = vec.split_last() {
            rest.iter().for_each(|line| {
                if !line.is_empty() {
                    padded_vec.push(line.clone());
                    padded_vec.push(String::new());
                }
            });
            padded_vec.push(last.clone());
        }
        padded_vec
    } else {
        vec!["Metadata not found.".to_string()]
    }
}

// fn get_file_detail_lines(path: &Path) -> Vec<String> {
//     let mut vec = Vec::<String>::new();
//     let file_ext = path
//         .extension()
//         .and_then(|ext| ext.to_str())
//         .unwrap_or("")
//         .to_string()
//         .to_uppercase();

//     if !file_ext.is_empty() {
//         vec.push(file_ext);
//     }

//     let mut padded_vec = Vec::<String>::new();
//     if let Some((last, rest)) = vec.split_last() {
//         for line in rest {
//             if !line.is_empty() {
//                 padded_vec.push(line.clone());
//                 padded_vec.push(String::new());
//             }
//         }
//         padded_vec.push(last.clone());
//     }
//     padded_vec
// }
