use crate::{logic::input_box::InputMode, message::Message, model::Model};

pub fn enter_edit_mode(model: &mut Model) -> Option<Message> {
    model.input_mode = InputMode::Editing;

    None
}

pub fn exit_edit_mode(model: &mut Model) -> Option<Message> {
    model.input_mode = InputMode::Normal;

    None
}
