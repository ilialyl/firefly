use crate::{
    global::{
        cmd::Confirmation, logic::session_state::Session, message::Message, view::tabs::SelectedTab,
    },
    player::{logic::Player, view::queue::QueueViewState},
    playlist::{logic::playlist_controller::PlaylistController, view::PlaylistViewState},
    user_input::logic::{InputMode, UserInput},
};

pub struct Model {
    pub session: Session,
    pub player: Player,
    pub playlist_ctl: PlaylistController,
    pub info_display: String,
    pub selected_tab: SelectedTab,
    pub queue_view: QueueViewState,
    pub playlist_view: PlaylistViewState,
    pub input_mode: InputMode,
    pub user_input: UserInput,
    pub ask_confirmation: Option<Message>,
    pub confirmation: Option<Confirmation>,
    pub confirmation_prompt: String,
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
            playlist_view: PlaylistViewState::default(),
            input_mode: InputMode::default(),
            user_input: UserInput::default(),
            ask_confirmation: None,
            confirmation: None,
            confirmation_prompt: String::new(),
        }
    }
}
