#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Busy,
    Done,
}

#[derive(Default)]
pub struct Session {
    pub state: RunningState,
}

