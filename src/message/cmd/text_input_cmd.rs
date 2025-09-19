use crate::{
    logic::user_input::{InputMode, InputTarget, PromptMsg},
    message::Message,
    model::Model,
};

pub fn enter_edit_mode(
    model: &mut Model,
    prompt: PromptMsg,
    to_edit: InputTarget,
) -> Option<Message> {
    model.input_mode = InputMode::Insert(prompt, to_edit);

    None
}

pub fn exit_edit_mode(model: &mut Model) -> Option<Message> {
    model.input_mode = InputMode::Commands;

    None
}

pub fn submit(to_edit: InputTarget, model: &mut Model) -> Option<Message> {
    model.user_input.submit_input();

    Some(Message::InputApply(to_edit))
}

pub fn enter_char(to_insert: char, model: &mut Model) -> Option<Message> {
    model.user_input.enter_char(to_insert);

    None
}

pub fn delete_char(model: &mut Model) -> Option<Message> {
    model.user_input.delete_char();

    None
}

pub fn move_cursor_left(model: &mut Model) -> Option<Message> {
    model.user_input.move_cursor_left();

    None
}

pub fn move_cursor_right(model: &mut Model) -> Option<Message> {
    model.user_input.move_cursor_right();

    None
}
