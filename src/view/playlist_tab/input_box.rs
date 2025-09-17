use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Style,
    widgets::{Block, Clear, Widget},
};

pub struct InputBox {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// History of recorded messages
    messages: Vec<String>,
}

impl InputBox {
    pub const fn new() -> Self {
        Self {
            input: String::new(),
            messages: Vec::new(),
            character_index: 0,
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub const fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, percent_x: u16, length_y: u16) {
        let popup_block = Block::bordered()
            .title("Value")
            .title_alignment(Alignment::Left)
            .border_style(Style::default());

        let area = Self::create_popup_area(area, percent_x, length_y);
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

    // fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    //     loop {
    //         terminal.draw(|frame| self.render(frame))?;

    //         if let Some(key) = event::read()?.as_key_press_event() {
    //             match self.input_mode {
    //                 InputMode::Normal => match key.code {
    //                     KeyCode::Char('e') => {
    //                         self.input_mode = InputMode::Editing;
    //                     }
    //                     KeyCode::Char('q') => {
    //                         return Ok(());
    //                     }
    //                     _ => {}
    //                 },
    //                 InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
    //                     KeyCode::Enter => self.submit_message(),
    //                     KeyCode::Char(to_insert) => self.enter_char(to_insert),
    //                     KeyCode::Backspace => self.delete_char(),
    //                     KeyCode::Left => self.move_cursor_left(),
    //                     KeyCode::Right => self.move_cursor_right(),
    //                     KeyCode::Esc => self.input_mode = InputMode::Normal,
    //                     _ => {}
    //                 },
    //                 InputMode::Editing => {}
    //             }
    //         }
    //     }
    // }
}
