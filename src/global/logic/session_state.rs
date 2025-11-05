use std::{
    net::SocketAddr,
    sync::{Arc, atomic::AtomicBool},
};

use color_eyre::eyre::Result;
use local_ip_address::local_ip;
use tokio::net::TcpListener;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    RunningFFmpeg,
    Exit,
}

pub struct Session {
    pub state: RunningState,
    pub unlocked_tick_rate: Arc<AtomicBool>,
    pub cover_listener: Option<TcpListener>,
    pub cover_server_addr: Option<SocketAddr>,
}

impl Session {
    pub async fn new() -> Result<Self> {
        let (cover_server_addr, cover_listener) = if cfg!(target_os = "linux") {
            let listener = TcpListener::bind(format!("{}:0", local_ip()?)).await?;
            let addr = listener.local_addr()?;

            (Some(addr), Some(listener))
        } else {
            (None, None)
        };
        Ok(Self {
            state: RunningState::default(),
            unlocked_tick_rate: Arc::new(AtomicBool::default()),
            cover_listener,
            cover_server_addr,
        })
    }
}
