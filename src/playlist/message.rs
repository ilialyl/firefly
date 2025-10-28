use crate::{
    global::{message::Message, view_logic::terminal::CursorMovementDirection},
    playlist::logic::{mini_metadata::MiniMetadata, playlist_collection::Index},
};

pub enum PlaylistMessage {
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
