use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Default, EnumIter, EnumCount, FromRepr, Clone, Copy)]
pub enum PlaylistTabFocus {
    #[default]
    Playlists,
    Tracks,
}

impl PlaylistTabFocus {
    pub fn cycle_focus_left(&mut self) {
        let current_index = *self as usize;
        let next_index = current_index.checked_sub(1).unwrap_or(0);

        *self = Self::from_repr(next_index).unwrap_or(*self);
    }

    pub fn cycle_focus_right(&mut self) {
        let current_index = *self as usize;
        let next_index = (current_index + 1).min(PlaylistTabFocus::COUNT - 1);

        *self = Self::from_repr(next_index).unwrap_or(*self);
    }
}
