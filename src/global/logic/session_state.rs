#[derive(Debug, Default, PartialEq, Eq)]
pub enum SessionState {
    #[default]
    Running,
    RunningFFmpeg,
    Exit,
}
