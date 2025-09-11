use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::Paragraph,
};

use crate::{logic::session_state::RunningState, model::Model};

pub fn draw(frame: &mut Frame, chunk: Rect, model: &Model) {
    let mut controls: Vec<Paragraph<'_>> = Vec::new();

    let mut toggle_play = Paragraph::new(" Play/Pause <Space>").fg(Color::White);
    if model.session.state == RunningState::Busy {
        toggle_play = toggle_play.crossed_out();
    }
    controls.push(toggle_play);

    controls.push(Paragraph::new(" Load Now <N>").fg(Color::White));
    controls.push(Paragraph::new(" Queue <Q>").fg(Color::White));
    controls.push(Paragraph::new(" Queue Dir <ShiftQ>").fg(Color::White));
    let mut toggle_arrange = Paragraph::new(" Queue Arrange <A>").fg(Color::White);
    if model.player.queue.is_arrange() {
        toggle_arrange = toggle_arrange.fg(Color::Rgb(255, 192, 15));
    }
    controls.push(toggle_arrange);

    controls.push(Paragraph::new(" Move Queue Up <↑>").fg(Color::White));
    controls.push(Paragraph::new(" Move Queue Down <↓>").fg(Color::White));
    controls.push(Paragraph::new(" Prev/Skip <P/S>").fg(Color::White));

    let mut rewind_seek = Paragraph::new(" Rewind/Seek <←/→>").fg(Color::White);
    if model.session.state == RunningState::Busy {
        rewind_seek = rewind_seek.crossed_out();
    }
    controls.push(rewind_seek);

    controls.push(Paragraph::new(" Volume <=/->").fg(Color::White));
    controls.push(Paragraph::new(" Loop <L>").fg(Color::White));
    controls.push(Paragraph::new(" Quit <Esc>").fg(Color::White));

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

    for (idx, control) in controls.into_iter().enumerate() {
        frame.render_widget(control, grid[idx])
    }
}
