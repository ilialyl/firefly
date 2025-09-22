#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Busy,
    Done,
}

pub struct Session {
    pub state: RunningState,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            state: RunningState::default(),
        }
    }
}
