use std::sync::{Arc, atomic::AtomicBool};

use color_eyre::eyre::Result;
use ratatui_image::picker::Picker;

use crate::{
    global::{
        logic::{
            confirmation::Confirmation, cover_art_server::CoverArtServer, senders::Senders,
            session_state::SessionState,
        },
        view::focused_area::FocusedArea,
    },
    player::logic::Player,
    playlist::logic::playlist_controller::PlaylistController,
    queue::logic::TrackQueue,
    user_input::logic::{InputMode, UserInput},
};

/// Stores app state.
pub struct App {
    pub focused_view_area: FocusedArea,
    pub input_mode: InputMode,
    pub show_help: bool,
    pub session_state: SessionState,
    pub unlocked_tick_rate: Arc<AtomicBool>,
    pub senders: Senders,
    pub player: Player,
    pub queue: TrackQueue,
    pub playlist_ctl: PlaylistController,
    pub info_msg: String,
    pub user_input: UserInput,
    pub user_confirmation: Confirmation,
    pub picker: Arc<Picker>,
    pub cover_art_server: CoverArtServer,
}

impl App {
    pub async fn new(senders: Senders) -> Result<App> {
        log::info!("Initializing App State...");
        let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::from_fontsize((8, 12)));
        let cover_art_server = CoverArtServer::new().await?;
        let unlocked_tick_rate = Arc::new(AtomicBool::default());
        Ok(Self {
            player: Player::new(senders.async_msg.clone(), cover_art_server.addr).await?,
            queue: TrackQueue::new(senders.msg.clone(), unlocked_tick_rate.clone()),
            playlist_ctl: PlaylistController::new(senders.msg.clone(), unlocked_tick_rate.clone())?,
            info_msg: String::new(),
            focused_view_area: FocusedArea::default(),
            input_mode: InputMode::default(),
            user_input: UserInput::default(),
            user_confirmation: Confirmation::default(),
            show_help: false,
            picker: Arc::new(picker),
            senders,
            cover_art_server,
            session_state: SessionState::default(),
            unlocked_tick_rate,
        })
    }
}
