use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Style,
    widgets::{Block, Clear, Widget},
};

pub struct InputPopup {
    percent_x: u16,
    length_y: u16,
}

impl InputPopup {
    pub fn new(percent_x: u16, length_y: u16) -> Self {
        Self {
            percent_x,
            length_y,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let popup_block = Block::bordered()
            .title("Value")
            .title_alignment(Alignment::Left)
            .border_style(Style::default());

        let area = Self::create_popup_area(area, self.percent_x, self.length_y);
        Clear.render(area, frame.buffer_mut());

        frame.render_widget(popup_block, area);
    }

    fn create_popup_area(area: Rect, percent_x: u16, length_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Length(length_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}
