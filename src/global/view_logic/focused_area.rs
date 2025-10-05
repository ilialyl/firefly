use log::debug;
use ratatui::text::Line;
use strum::EnumCount;
use strum_macros::{Display, EnumCount, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, EnumCount)]
pub enum FocusedArea {
    #[default]
    #[strum(to_string = "Player")]
    Main,
    #[strum(to_string = "Playlist")]
    Playlist,
}

impl FocusedArea {
    pub fn cycle_right(&mut self) {
        let current_index = *self as usize;
        let mut next_index = current_index + 1;
        if next_index >= FocusedArea::COUNT {
            next_index = 0;
        }

        debug!("tab {}", current_index);
        *self = Self::from_repr(next_index).unwrap_or(*self);
    }

    pub fn title(self) -> Line<'static> {
        format!("  {self}  ").into()
    }
}
