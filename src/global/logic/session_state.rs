use std::sync::{Arc, atomic::AtomicBool};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    RunningFFmpeg,
    Done,
}

#[derive(Default)]
pub struct Session {
    pub state: RunningState,
    pub unlocked_tick_rate: Arc<AtomicBool>,
}
