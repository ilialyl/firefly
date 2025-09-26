use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::Paragraph,
};

use crate::{global::logic::session_state::RunningState, model::Model};

pub fn draw(frame: &mut Frame, chunk: Rect, model: &Model) {
    let mut controls: Vec<Paragraph<'_>> = Vec::new();

    let mut toggle_play = Paragraph::new(" Play/Pause <Space>");
    if model.session.state == RunningState::Busy {
        toggle_play = toggle_play.crossed_out();
    }
    controls.push(toggle_play);

    controls.push(Paragraph::new(" Load Now <N>"));
    controls.push(Paragraph::new(" Queue <Q>"));
    controls.push(Paragraph::new(" Queue Dir <ShiftQ>"));
    let mut toggle_arrange = Paragraph::new(" Arrange Queue <A>");
    if model.player.queue.is_arrange() {
        toggle_arrange = toggle_arrange.fg(Color::Rgb(255, 192, 15));
    }
    controls.push(toggle_arrange);

    controls.push(Paragraph::new(" Move Queue Up <↑>"));
    controls.push(Paragraph::new(" Move Queue Down <↓>"));
    controls.push(Paragraph::new(" Prev/Skip <P/S>"));

    let mut rewind_seek = Paragraph::new(" Rewind/Seek <←/→>");
    if model.session.state == RunningState::Busy {
        rewind_seek = rewind_seek.crossed_out();
    }
    controls.push(rewind_seek);

    controls.push(Paragraph::new(" Volume <=/->"));
    controls.push(Paragraph::new(" Loop <L>"));
    controls.push(Paragraph::new(" Quit <Esc>"));

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

    controls
        .iter()
        .enumerate()
        .for_each(|(idx, ctl)| frame.render_widget(ctl, grid[idx]));
}
