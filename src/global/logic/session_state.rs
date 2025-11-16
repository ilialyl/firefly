use std::sync::{Arc, atomic::AtomicBool};

use color_eyre::eyre::Result;

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
}

impl Session {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            state: RunningState::default(),
            unlocked_tick_rate: Arc::new(AtomicBool::default()),
        })
    }
}
