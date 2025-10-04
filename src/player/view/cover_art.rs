use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Widget},
};
use ratatui_image::{Resize, StatefulImage};

use crate::model::Model;

pub fn draw(area: Rect, frame: &mut Frame, model: &mut Model) {
    if let Some(current_track) = model.player.current.as_mut()
        && let Some(img) = current_track.dyn_img.as_mut()
    {
        let widget = StatefulImage::default().resize(Resize::Scale(None));
        frame.render_stateful_widget(widget, area, img);
    } else {
        Block::bordered().render(area, frame.buffer_mut());
    }
}
