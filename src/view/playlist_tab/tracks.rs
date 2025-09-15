use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

use crate::model::Model;

pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
    if let Some(selected) = model.playlist_controller.get_selected_playlist() {
        let name_lines: Vec<Line> = selected
            .entries
            .iter()
            .map(|e| {
                e.file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap_or("[Invalid UTF-8 name]")
            })
            .map(|n| Line::from(n))
            .collect();

        let paragraph = Paragraph::new(name_lines);

        frame.render_widget(paragraph, area);
    }
}
