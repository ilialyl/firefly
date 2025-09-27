use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::{global::view::center_xy, model::Model};

pub fn render(model: &Model, frame: &mut Frame, area: Rect) {
    let popup_block = Block::bordered()
        .title("Info ")
        .title_alignment(Alignment::Left)
        .border_style(Style::default());

    let area = center_xy(area, 30, 3);
    Clear.render(area, frame.buffer_mut());

    frame.render_widget(
        Paragraph::new(model.info_msg.clone()).block(popup_block),
        area,
    );
}
