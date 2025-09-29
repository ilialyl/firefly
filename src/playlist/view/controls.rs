use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Text},
    widgets::{Block, Clear, Padding, Paragraph, Widget},
};

pub fn draw(frame: &mut Frame, area: Rect) {
    let controls = get_controls();
    let max_len = controls
        .iter()
        .map(|(desc, _)| desc.len())
        .max()
        .unwrap_or(0);

    let lines: Vec<Line> = controls
        .into_iter()
        .map(|(desc, key)| {
            let padded_desc = format!("{:<width$}", desc, width = max_len);
            Line::from(format!("{}  {}", padded_desc, key))
        })
        .collect();

    let para = Paragraph::new(Text::from(lines)).block(
        Block::bordered()
            .padding(Padding::new(2, 2, 1, 1))
            .title("Controls")
            .title_alignment(Alignment::Right)
            .border_style(Style::default()),
    );

    Clear.render(area, frame.buffer_mut());
    para.render(area, frame.buffer_mut());
}

fn get_controls() -> Vec<(String, String)> {
    vec![
        ("Navigate", "<↑↓→←>"),
        ("New Playlist", "<N>"),
        ("Rename Playlist", "<F2>"),
        ("Save Playlist", "<F5>"),
        ("Delete Playlist", "<F9>"),
        ("Add Tracks", "<Q>"),
        ("Add Directory", "<ShiftQ>"),
        ("Remove Track", "<Del>"),
        ("Arrange Track", "<A>"),
        ("Send Playlist to Player", "<F1>"),
        ("Hide this panel", "<C>"),
    ]
    .iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect()
}
