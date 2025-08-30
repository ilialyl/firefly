use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Span, Text},
    widgets::Paragraph,
};

use crate::model::Model;

pub fn draw(frame: &mut Frame, chunk: Rect, model: &Model) {
    let mut controls: Vec<Span<'_>> = Vec::new();

    let mut toggle_play = Span::from(" Play/Pause <Space>");
    if model.busy {
        toggle_play = toggle_play.crossed_out();
    }
    controls.push(toggle_play);

    controls.push(Span::from(" Load Now <N>"));
    controls.push(Span::from(" Queue <Q>"));
    controls.push(Span::from(" Queue Dir <ShiftQ>"));
    let mut toggle_arrange = Span::from(" Toggle Q Arrange <A>");
    if model.arrange_mode {
        toggle_arrange = toggle_arrange.bold().italic();
    }
    controls.push(toggle_arrange);

    controls.push(Span::from(" Skip <S>"));

    let mut rewind_seek = Span::from(" Rewind/Seek <←/→>");
    if model.busy {
        rewind_seek = rewind_seek.crossed_out();
    }
    controls.push(rewind_seek);

    controls.push(Span::from(" Volume <=/->"));
    controls.push(Span::from(" Loop <L>"));
    controls.push(Span::from(" Quit <Esc>"));

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
        frame.render_widget(Paragraph::new(Text::from(control.clone())), grid[idx])
    }
}
