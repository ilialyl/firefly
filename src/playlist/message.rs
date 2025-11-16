use crate::{
    global::message::Message,
    playlist::logic::{mini_metadata::MiniMetadata, playlist_collection::Index},
    tui::Direction,
};

pub enum PlaylistMessage {
    Navigate(Direction),
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
