use std::sync::{Arc, mpsc::Sender};

use ratatui_image::picker::Picker;

use crate::{
    global::{
        logic::{confirmation::Confirmation, session_state::Session},
        message::Message,
        view_logic::focused_area::FocusedArea,
    },
    player::logic::Player,
    playlist::logic::playlist_controller::PlaylistController,
    user_input::logic::{InputMode, UserInput},
};

pub struct Model {
    pub session: Session,
    pub player: Player,
    pub playlist_ctl: PlaylistController,
    pub info_msg: String,
    pub focused_view_area: FocusedArea,
    pub input_mode: InputMode,
    pub user_input: UserInput,
    pub confirmation: Confirmation,
    pub show_help: bool,
    pub picker: Arc<Picker>,
}

impl Model {
    pub fn new(msg_tx: Sender<Message>) -> Self {
        log::debug!("Initialized Model");
        let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::from_fontsize((8, 12)));
        let session = Session::default();
        Self {
            player: Player::new(msg_tx, &session),
            session: session,
            playlist_ctl: PlaylistController::default(),
            info_msg: String::new(),
            focused_view_area: FocusedArea::default(),
            input_mode: InputMode::default(),
            user_input: UserInput::default(),
            confirmation: Confirmation::default(),
            show_help: false,
            picker: Arc::new(picker),
        }
    }
}
