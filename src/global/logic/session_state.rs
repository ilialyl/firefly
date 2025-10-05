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
}
