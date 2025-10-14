#[derive(PartialEq, Clone, Copy)]
pub enum FormatConversion {
    Idle,
    Running,
    Done,
    Unnecessary,
}
