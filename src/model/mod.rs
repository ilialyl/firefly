use crate::{
    logic::{player::Player, session_state::Session},
    view::tabs::SelectedTab,
};

pub struct Model {
    pub session: Session,
    pub player: Player,
    pub info_display: String,
    pub selected_tab: SelectedTab,
}

impl Default for Model {
    fn default() -> Self {
        let session = Session::default();
        Self {
            player: Player::new(),
            session,
            info_display: String::new(),
            selected_tab: SelectedTab::default(),
        }
    }
}
