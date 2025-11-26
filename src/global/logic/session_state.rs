/// States that affect functionalities of the App.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum SessionState {
    #[default]
    Running,
    RunningFFmpeg,
    Exit,
}
