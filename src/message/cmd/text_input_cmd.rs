use crate::{message::Message, model::Model, view::terminal::InputMode};

pub fn enter_edit_mode(model: &mut Model) -> Option<Message> {
    model.input_mode = InputMode::Editing;

    None
}

pub fn exit_edit_mode(model: &mut Model) -> Option<Message> {
    model.input_mode = InputMode::Normal;

    None
}

pub fn submit(model: &mut Model) -> Option<Message> {
    model.input_box.submit_message();

    Some(Message::ExitEditMode)
}

pub fn enter_char(to_insert: char, model: &mut Model) -> Option<Message> {
    model.input_box.enter_char(to_insert);

    None
}

pub fn delete_char(model: &mut Model) -> Option<Message> {
    model.input_box.delete_char();

    None
}

pub fn move_cursor_left(model: &mut Model) -> Option<Message> {
    model.input_box.move_cursor_left();

    None
}

pub fn move_cursor_right(model: &mut Model) -> Option<Message> {
    model.input_box.move_cursor_right();

    None
}
