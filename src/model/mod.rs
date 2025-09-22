use crate::{
    global::{
        cmd::Confirmation, logic::session_state::Session, message::Message, view::tabs::SelectedTab,
    },
    player::{logic::Player, view::queue::QueueViewState},
    playlist::logic::playlist_controller::PlaylistController,
    user_input::logic::{InputMode, UserInput},
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
    pub ask_confirmation: Option<Message>,
    pub confirmation: Option<Confirmation>,
}

impl Default for Model {
    fn default() -> Self {
        let session = Session::default();
        log::debug!("Initialized Model");
        Self {
            player: Player::new(),
            playlist_ctl: PlaylistController::default(),
            session,
            info_display: String::new(),
            selected_tab: SelectedTab::default(),
            queue_view: QueueViewState::default(),
            input_mode: InputMode::default(),
            user_input: UserInput::new(),
            ask_confirmation: None,
            confirmation: None,
        }
    }
}
