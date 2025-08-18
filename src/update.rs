use std::time::Duration;

use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;

use crate::{
    player,
    ui::{
        message::{Message, update},
        model::Model,
        view::view,
    },
};

fn stop_info_display(model: &mut Model) {
    model.info.push(String::new());
}

fn display_info(model: &mut Model, info: &str) {
    model.info.push(info.to_string());
}

fn play_next_track(model: &mut Model, terminal: &mut DefaultTerminal) {
    let next_track = match model.track_queue.pop_front() {
        Some(path) => path,
        None => {
            return;
        }
    };

    match player::is_rodio_supported(&next_track) {
        Ok(condition) => {
            if !condition {
                display_info(model, "Converting format and normalizing volume...");

                refresh_frame(model, terminal).expect("Error refreshing frame");
                player::convert_format(&next_track);
            }
        }
        Err(e) => display_info(model, e.to_string().as_str()),
    }

    if let Err(e) = player::load_track(&model.sink, &next_track) {
        display_info(model, e.to_string().as_str())
    };

    model.track_path = Some(next_track);
    model.track_duration = player::get_track_duration(model.track_path.as_ref().unwrap()).ok();

    stop_info_display(model);
}

pub fn track_pos_as_str(model: &Model) -> String {
    let track_pos = model.track_pos.clone().unwrap_or(Duration::from_secs(0));
    let sec = track_pos.as_secs() % 60;
    let min = track_pos.as_secs() / 60;

    format!("{:02}:{:02}", min, sec)
}

pub fn refresh_frame(model: &mut Model, terminal: &mut DefaultTerminal) -> Result<()> {
    update(model, Message::Tick);
    terminal.draw(|f| view(model, f))?;

    Ok(())
}
