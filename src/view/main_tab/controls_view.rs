use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};

pub fn draw(frame: &mut Frame, chunk: Rect) {
    let controls = [
        " Play/Pause <Space>",
        " Load Now <N>",
        " Queue <Q>",
        " Queue Dir <ShiftQ>",
        " Toggle Q Arrange <A>",
        " Skip <S>",
        " Rewind/Seek <←/→>",
        " Volume <=/->",
        " Loop <L>",
        " Quit <Esc>",
    ];

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); 4])
        .spacing(1)
        .split(chunk);

    let grid: Vec<Rect> = rows
        .iter()
        .flat_map(|col| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(25); 4])
                .spacing(1)
                .split(*col)
                .to_vec()
        })
        .collect();

    for (idx, control) in controls.iter().enumerate() {
        frame.render_widget(Paragraph::new(*control), grid[idx])
    }
}
