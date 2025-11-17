use strum_macros::{EnumCount, EnumIter, FromRepr};

/// UI-related state.
#[derive(Default, EnumIter, EnumCount, FromRepr, Clone, Copy)]
pub enum PlaylistTabFocus {
    #[default]
    Playlists,
    Tracks,
}
