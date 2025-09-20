use ratatui::{
    Frame,
    layout::{Alignment, Position, Rect},
    style::Style,
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::{logic::user_input::UserInput, view::center_xy};

impl UserInput {
    pub fn render(
        &self,
        prompt: &str,
        percent_x: u16,
        length_y: u16,
        frame: &mut Frame,
        area: Rect,
    ) {
        let popup_block = Block::bordered()
            .title(prompt)
            .title_alignment(Alignment::Left)
            .border_style(Style::default());

        let area = center_xy(area, percent_x, length_y);
        Clear.render(area, frame.buffer_mut());

        let input = Paragraph::new(self.input.as_str());

        frame.render_widget(input.block(popup_block), area);
        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            area.x + self.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            area.y + 1,
        ))
    }
}
