use crate::{
    logic::user_input::{InputMode, InputTarget, PromptMsg},
    message::{Message, UserInputMessage, cmd::playlist_cmd::name_playlist},
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

    Some(Message::UserInput(UserInputMessage::Apply(to_edit)))
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

pub fn apply_input(to_edit: InputTarget, model: &mut Model) -> Option<Message> {
    match to_edit {
        InputTarget::PlaylistName(index) => name_playlist(index, model),
    }
}

pub fn handle_exit_insert_early(to_edit: InputTarget, model: &mut Model) -> Option<Message> {
    match to_edit {
        InputTarget::PlaylistName(index) => {
            if let Some(playlist) = model.playlist_ctl.playlist_coll.get_playlist(index) {
                if playlist.get_name().is_none() {
                    model.playlist_ctl.delete_playlist(index);
                }
            }

            Some(Message::UserInput(UserInputMessage::Exit))
        }
    }
}
