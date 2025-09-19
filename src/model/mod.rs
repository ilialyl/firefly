use crate::{
    logic::{
        player::Player,
        playlist::playlist_controller::PlaylistController,
        session_state::Session,
        user_input::{InputMode, UserInput},
    },
    view::{main_tab::queue_view::QueueViewState, tabs::SelectedTab},
};

pub struct Model {
    pub session: Session,
    pub player: Player,
    pub playlist_ctl: PlaylistController,
    pub info_display: String,
    pub selected_tab: SelectedTab,
    pub queue_view: QueueViewState,
    pub input_mode: InputMode,
    pub user_input: UserInput,
}

impl Default for Model {
    fn default() -> Self {
        let session = Session::default();
        Self {
            player: Player::new(),
            playlist_ctl: PlaylistController::default(),
            session,
            info_display: String::new(),
            selected_tab: SelectedTab::default(),
            queue_view: QueueViewState::default(),
            input_mode: InputMode::default(),
            user_input: UserInput::new(),
        }
    }
}
