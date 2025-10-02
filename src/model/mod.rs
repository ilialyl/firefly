use std::sync::Arc;

use ratatui_image::picker::Picker;

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
    pub info_msg: String,
    pub show_controls: bool,
    pub picker: Arc<Picker>,
}

impl Default for Model {
    fn default() -> Self {
        log::debug!("Initialized Model");
        let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::from_fontsize((8, 12)));
        Self {
            player: Player::new(),
            playlist_ctl: PlaylistController::default(),
            session: Session::default(),
            status_msg: String::new(),
            selected_tab: SelectedTab::default(),
            input_mode: InputMode::default(),
            user_input: UserInput::default(),
            confirmation: Confirmation::default(),
            info_msg: String::new(),
            show_controls: false,
            picker: Arc::new(picker),
        }
    }
}
