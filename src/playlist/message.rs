use crate::{
    global::{logic::Index, message::Message, view_logic::terminal::CursorMovementDirection},
    playlist::logic::mini_metadata::MiniMetadata,
};

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
    LoadedMetadata(Index, MiniMetadata),
}
