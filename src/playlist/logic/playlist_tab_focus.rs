use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Default, EnumIter, EnumCount, FromRepr, Clone, Copy)]
pub enum PlaylistTabFocus {
    #[default]
    Playlists,
    Tracks,
}
