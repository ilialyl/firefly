use crate::{message::Message, model::Model, view::input_box::InputMode};

pub fn enter_edit_mode(model: &mut Model) -> Option<Message> {
    model.input_mode = InputMode::Editing;

    None
}

pub fn exit_edit_mode(model: &mut Model) -> Option<Message> {
    model.input_mode = InputMode::Normal;

    None
}
