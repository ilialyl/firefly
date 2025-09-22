use crate::{
    global::message::{Message, UserInputMessage},
    model::Model,
    user_input::cmd::*,
};

pub fn update_userinput(model: &mut Model, msg: UserInputMessage) -> Option<Message> {
    match msg {
        UserInputMessage::EnterEditMode(prompt, to_edit) => enter_edit_mode(model, prompt, to_edit),
        UserInputMessage::Exit => exit_edit_mode(model),
        UserInputMessage::ExitEarly(to_edit) => handle_exit_insert_early(to_edit, model),
        UserInputMessage::Submit(to_edit) => submit(to_edit, model),
        UserInputMessage::Insert(char) => enter_char(char, model),
        UserInputMessage::Delete => delete_char(model),
        UserInputMessage::MoveCursorLeft => move_cursor_left(model),
        UserInputMessage::MoveCursorRight => move_cursor_right(model),
        UserInputMessage::Apply(to_edit) => apply_input(to_edit, model),
    }
}
