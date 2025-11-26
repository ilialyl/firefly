use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::{
    app::App,
    global::view::{center_xy, terminal_is_small},
};

pub fn draw(app: &App, frame: &mut Frame, area: Rect) {
    let popup_block = Block::bordered()
        .title(format!("{} ", app.user_confirmation.prompt))
        .title_alignment(Alignment::Left)
        .border_style(Style::default());

    let area = if terminal_is_small(area) {
        center_xy(area, 90, 3)
    } else {
        center_xy(area, 30, 3)
    };
    Clear.render(area, frame.buffer_mut());

    frame.render_widget(Paragraph::new(" Y / N ").block(popup_block), area);
}
