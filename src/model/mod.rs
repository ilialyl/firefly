use crate::{
    global::{
        logic::{confirmation::Confirmation, session_state::Session},
        view::tabs::SelectedTab,
    },
    player::logic::Player,
    playlist::logic::playlist_controller::PlaylistController,
    user_input::logic::{InputMode, UserInput},
};

pub struct Model {
    pub session: Session,
    pub player: Player,
    pub playlist_ctl: PlaylistController,
    pub status_msg: String,
    pub selected_tab: SelectedTab,
    pub input_mode: InputMode,
    pub user_input: UserInput,
    pub confirmation: Confirmation,
}

impl Default for Model {
    fn default() -> Self {
        log::debug!("Initialized Model");
        Self {
            player: Player::new(),
            playlist_ctl: PlaylistController::default(),
            session: Session::default(),
            status_msg: String::new(),
            selected_tab: SelectedTab::default(),
            input_mode: InputMode::default(),
            user_input: UserInput::default(),
            confirmation: Confirmation::default(),
        }
    }
}
