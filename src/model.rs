use std::sync::Arc;

use color_eyre::eyre::Result;
use ratatui_image::picker::Picker;

use crate::{
    global::{
        logic::{confirmation::Confirmation, senders::Senders, session_state::Session},
        view_logic::focused_area::FocusedArea,
    },
    player::logic::Player,
    playlist::logic::playlist_controller::PlaylistController,
    queue::logic::TrackQueue,
    user_input::logic::{InputMode, UserInput},
};

pub struct Model {
    pub session: Session,
    pub player: Player,
    pub queue: TrackQueue,
    pub playlist_ctl: PlaylistController,
    pub info_msg: String,
    pub focused_view_area: FocusedArea,
    pub input_mode: InputMode,
    pub user_input: UserInput,
    pub user_confirmation: Confirmation,
    pub show_help: bool,
    pub picker: Arc<Picker>,
    pub senders: Senders,
}

impl Model {
    pub async fn new(senders: Senders) -> Result<Model> {
        log::info!("Initializing App State...");
        let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::from_fontsize((8, 12)));
        let session = Session::default();
        Ok(Self {
            player: Player::new(senders.async_msg.clone()).await?,
            queue: TrackQueue::new(senders.msg.clone(), session.unlocked_tick_rate.clone()),
            playlist_ctl: PlaylistController::new(
                senders.msg.clone(),
                session.unlocked_tick_rate.clone(),
            )?,
            info_msg: String::new(),
            focused_view_area: FocusedArea::default(),
            input_mode: InputMode::default(),
            user_input: UserInput::default(),
            user_confirmation: Confirmation::default(),
            show_help: false,
            picker: Arc::new(picker),
            session,
            senders,
        })
    }
}
