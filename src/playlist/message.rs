use crate::global::{message::Message, view_logic::terminal::CursorMovementDirection};

pub enum PlaylistMessage {
    LoadPlaylists,
    MoveCursor(CursorMovementDirection),
    Create,
    Rename,
    Delete,
    SaveSelected,
    AddTracks,
    AddDir,
    RemoveTrack,
    ToggleArrangeTracks,
    SendToPlayer,
    AskToSave(Box<Option<Message>>),
    ScrollToStart,
    ScrollToEnd,
}
