use crate::user_input::logic::{InputTarget, PromptMsg};

/// UserInput-related Messages.
pub enum UserInputMessage {
    Submit(InputTarget),
    Insert(char),
    Apply(InputTarget),
    Delete,
    MoveCursorLeft,
    MoveCursorRight,
    EnterEditMode(PromptMsg, InputTarget),
    Exit,
    ExitEarly(InputTarget),
}
