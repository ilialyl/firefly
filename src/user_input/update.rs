use crate::{
    app::App,
    global::message::Message,
    user_input::{cmd::*, message::UserInputMessage},
};

pub fn update_userinput(app: &mut App, msg: UserInputMessage) -> Option<Message> {
    match msg {
        UserInputMessage::EnterEditMode(prompt, to_edit) => enter_edit_mode(app, prompt, to_edit),
        UserInputMessage::Exit => exit_edit_mode(app),
        UserInputMessage::ExitEarly(to_edit) => handle_exit_insert_early(to_edit, app),
        UserInputMessage::Submit(to_edit) => submit(to_edit, app),
        UserInputMessage::Insert(char) => enter_char(char, app),
        UserInputMessage::Delete => delete_char(app),
        UserInputMessage::MoveCursorLeft => move_cursor_left(app),
        UserInputMessage::MoveCursorRight => move_cursor_right(app),
        UserInputMessage::Apply(to_edit) => apply_input(to_edit, app),
    }
}
