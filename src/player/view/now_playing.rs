use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
};

use crate::{model::Model, player::view::details::duration_as_str};

pub fn draw(area: Rect, frame: &mut Frame, model: &mut Model) {
    let area = Layout::horizontal(vec![
        Constraint::Percentage(20),
        Constraint::Fill(1),
        Constraint::Percentage(20),
    ])
    .flex(ratatui::layout::Flex::SpaceAround)
    .spacing(2)
    .split(area);
    // ⚬──────
    let progress_str = "─".repeat(area[1].width as usize);
    let duration_str = if let Some(ref mut current_track) = model.player.current {
        format!(
            "{} / {}",
            duration_as_str(&current_track.pos),
            duration_as_str(&current_track.duration.unwrap_or(Duration::from_secs(0)))
        )
    } else {
        String::new()
    };

    let misc_str = format!("{} ↻", duration_str);

    let play = Paragraph::new("► ⏸︎").centered();
    let progress = Paragraph::new(progress_str);
    let misc = Paragraph::new(misc_str).centered();

    frame.render_widget(play, area[0]);
    frame.render_widget(progress, area[1]);
    frame.render_widget(misc, area[2]);
}
