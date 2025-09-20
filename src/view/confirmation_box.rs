use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::view::center_xy;

pub fn render(frame: &mut Frame, area: Rect) {
    let popup_block = Block::bordered()
        .title("Are you sure?")
        .title_alignment(Alignment::Left)
        .border_style(Style::default());

    let area = center_xy(area, 30, 3);
    Clear.render(area, frame.buffer_mut());

    frame.render_widget(Paragraph::new("Y / N").block(popup_block), area);
}
