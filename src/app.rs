use std::{
    collections::{HashMap, VecDeque},
    io::Stdout,
    sync::{Arc, atomic::AtomicBool},
};

use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use rand::{Rng, distr::Alphanumeric};
use ratatui::{Terminal, prelude::CrosstermBackend};
use ratatui_image::picker::Picker;
use rodio::SampleRate;

use crate::{
    global::{
        logic::{
            channels::{Receivers, Senders, channels},
            confirmation::Confirmation,
            cover_art_server::CoverArtServer,
            data::{ConfigKeys, load_config},
            session_state::SessionState,
        },
        message::Message,
        update::update_global,
        view::{draw, focused_area::FocusedArea},
    },
    player::logic::{DEFAULT_SAMPLE_RATE, Player},
    playlist::logic::playlist_controller::PlaylistController,
    queue::logic::TrackQueue,
    tui::handle_events,
    user_input::logic::{InputMode, UserInput},
};

lazy_static! {
    pub static ref SESSION_ID: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();
}

/// Stores app state.
pub struct App {
    pub focused_view_area: FocusedArea,
    pub input_mode: InputMode,
    pub show_help: bool,
    pub session_state: SessionState,
    pub tick_rate_unlocked: Arc<AtomicBool>,
    pub senders: Senders,
    pub receivers: Receivers,
    pub player: Player,
    pub queue: TrackQueue,
    pub playlist_ctl: PlaylistController,
    pub info_msg: String,
    pub user_input: UserInput,
    pub user_confirmation: Confirmation,
    pub picker: Arc<Picker>,
    pub cover_art_server: CoverArtServer,
    pub config: HashMap<ConfigKeys, String>,
}

impl App {
    pub async fn new() -> Result<App> {
        log::info!("Initializing App State...");
        let config = load_config()?;
        let (senders, receivers) = channels();
        let cover_art_server = CoverArtServer::new().await?;
        let tick_rate_unlocked = Arc::new(AtomicBool::default());

        Ok(Self {
            player: Player::new(
                senders.async_msg.clone(),
                cover_art_server.addr,
                config
                    .get(&ConfigKeys::SampleRate)
                    .and_then(|v| v.parse::<SampleRate>().ok())
                    .unwrap_or(DEFAULT_SAMPLE_RATE),
            )
            .await?,
            queue: TrackQueue::new(senders.msg.clone(), tick_rate_unlocked.clone()),
            playlist_ctl: PlaylistController::new(senders.msg.clone(), tick_rate_unlocked.clone())?,
            info_msg: String::new(),
            focused_view_area: FocusedArea::default(),
            input_mode: InputMode::default(),
            user_input: UserInput::default(),
            user_confirmation: Confirmation::default(),
            show_help: false,
            picker: Arc::new(
                Picker::from_query_stdio().unwrap_or_else(|_| Picker::from_fontsize((8, 12))),
            ),
            senders,
            receivers,
            cover_art_server,
            session_state: SessionState::default(),
            tick_rate_unlocked,
            config,
        })
    }

    /// Run the main app loop.
    pub async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        if cfg!(target_os = "linux") {
            self.cover_art_server.run_server().await?;
        }

        while self.session_state != SessionState::Exit {
            // VecDeque is better for queues
            let mut msg_queue: VecDeque<Message> = VecDeque::new();

            // Tick at the start to keep things updated
            if let Some(msg) = update_global(self, Message::Tick).await {
                msg_queue.push_back(msg);
            }

            // Draw TUI view
            terminal.draw(|f| draw(self, f))?;

            // Handle terminal events
            if let Some(msg) = handle_events(self)? {
                msg_queue.push_back(msg);
            }

            // Receive message from other threads
            while let Ok(msg) = self.receivers.msg.try_recv() {
                msg_queue.push_back(msg);
            }

            // Async receiver
            while let Ok(msg) = self.receivers.async_msg.try_recv() {
                msg_queue.push_back(msg);
            }

            // Display info sent from other threads
            if let Ok(info) = self.receivers.info.try_recv() {
                update_global(self, Message::UpdateInfoMsg(info)).await;
            }

            // Consume messages
            while !msg_queue.is_empty()
                && let Some(msg) = msg_queue.pop_front()
            {
                if let Some(msg) = update_global(self, msg).await {
                    msg_queue.push_back(msg);
                }
            }
        }

        Ok(())
    }
}
