use crate::{
    app::App,
    global::message::Message,
    playlist::cmd::name_playlist,
    user_input::{
        logic::{InputMode, InputTarget, PromptMsg},
        message::UserInputMessage,
    },
};

pub fn enter_edit_mode(app: &mut App, prompt: PromptMsg, to_edit: InputTarget) -> Option<Message> {
    app.input_mode = InputMode::Insert(prompt, to_edit);

    None
}

pub fn exit_edit_mode(app: &mut App) -> Option<Message> {
    app.input_mode = InputMode::Commands;

    None
}

pub fn submit(to_edit: InputTarget, app: &mut App) -> Option<Message> {
    app.user_input.submit_input();

    Some(Message::UserInput(UserInputMessage::Apply(to_edit)))
}

pub fn enter_char(to_insert: char, app: &mut App) -> Option<Message> {
    app.user_input.enter_char(to_insert);

    None
}

pub fn delete_char(app: &mut App) -> Option<Message> {
    app.user_input.delete_char();

    None
}

pub fn move_cursor_left(app: &mut App) -> Option<Message> {
    app.user_input.move_cursor_left();

    None
}

pub fn move_cursor_right(app: &mut App) -> Option<Message> {
    app.user_input.move_cursor_right();

    None
}

pub fn apply_input(to_edit: InputTarget, app: &mut App) -> Option<Message> {
    match to_edit {
        InputTarget::PlaylistName(index) => name_playlist(index, app),
    }
}

pub fn handle_exit_insert_early(to_edit: InputTarget, app: &mut App) -> Option<Message> {
    match to_edit {
        InputTarget::PlaylistName(index) => {
            if let Some(playlist) = app.playlist_ctl.playlist_coll.get_playlist(index)
                && playlist.get_name().is_none()
                && let Err(e) = app.playlist_ctl.delete_playlist(index)
            {
                log::error!("Error deleting playlist: {e}");
            };

            Some(Message::UserInput(UserInputMessage::Exit))
        }
    }
}
