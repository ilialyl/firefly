pub mod album_art;
pub mod controls;
pub mod now_playing;
pub mod queue;

use ratatui::layout::{Layout, Rect};
use ratatui::{
    Frame,
    layout::{Constraint, Direction},
};

use crate::model::Model;

pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    let left_panel = panels[0];
    let right_panel = panels[1];

    let left_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(left_panel.width / 2),
            Constraint::Min(0),
        ])
        .split(left_panel);

    let right_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .margin(2)
        .split(right_panel);

    album_art::draw(left_panel_chunks[0], frame, model);
    queue::draw(left_panel_chunks[1], frame, model);
    now_playing::draw(right_panel_chunks[0], frame, model);
    controls::draw(right_panel_chunks[1], frame, model);
}

// pub fn draw(model: &mut Model, frame: &mut Frame, area: Rect) {
//     let inner_layout = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
//         .split(area);

//     let main_chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
//         .split(inner_layout[1]);

//     let (term_width, term_height) = crossterm::terminal::size().unwrap();
//     let term_too_small = term_width < 133 || term_height < 30;
//     let queue_panel_constant = if term_too_small {
//         vec![Constraint::Percentage(70), Constraint::Percentage(30)]
//     } else {
//         vec![
//             Constraint::Length(inner_layout[0].width),
//             Constraint::Fill(1),
//         ]
//     };

//     let left_panel_border = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints(&queue_panel_constant)
//         .split(inner_layout[0]);

//     let left_panel_chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints(queue_panel_constant)
//         .split(inner_layout[0]);

//     queue::draw(model, frame, left_panel_chunks[0]);

//     Block::bordered()
//         .title(Line::style(Line::from("Player"), Style::new()))
//         .border_style(Style::default())
//         .title_alignment(Alignment::Right)
//         .render(main_chunks[0], frame.buffer_mut());

//     Block::bordered()
//         .title(Line::style(Line::from("Control"), Style::new()))
//         .border_style(Style::default())
//         .title_alignment(Alignment::Right)
//         .render(main_chunks[1], frame.buffer_mut());

//     Block::bordered()
//         .title(Line::style(Line::from("Queue"), Style::new()))
//         .border_style(Style::default())
//         .title_alignment(Alignment::Left)
//         .render(left_panel_border[0], frame.buffer_mut());

//     if term_too_small {
//         Block::bordered()
//             .title(Line::style(Line::from("Warning"), Style::new()))
//             .border_style(Style::default())
//             .title_alignment(Alignment::Left)
//             .render(left_panel_chunks[1], frame.buffer_mut());

//         let warning_chunk = Layout::default()
//             .direction(Direction::Vertical)
//             .constraints(vec![Constraint::Percentage(100)])
//             .margin(1)
//             .split(left_panel_chunks[1]);

//         frame.render_widget(
//             Paragraph::new("Your terminal size may be too small to display UI properly.")
//                 .wrap(Wrap { trim: true }),
//             warning_chunk[0],
//         )
//     }

//     let player_chunks_const = match &mut model.player.current {
//         Some(current_track) if current_track.has_title => {
//             vec![Constraint::Percentage(60), Constraint::Percentage(40)]
//         }
//         _ => vec![Constraint::Percentage(100)],
//     };

//     let player_chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints(player_chunks_const)
//         .margin(2)
//         .split(main_chunks[0]);

//     now_playing::draw(model, frame, player_chunks);

//     let control_chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints(vec![Constraint::Percentage(100)])
//         .margin(2)
//         .horizontal_margin(3)
//         .split(main_chunks[1]);

//     controls::draw(frame, control_chunks[0], model);
// }
