use std::io::Cursor;

use color_eyre::eyre::Result;
use image::ImageReader;
use lofty::{file::TaggedFileExt, picture::Picture};
use ratatui::{Frame, layout::Rect};
use ratatui_image::{
    CropOptions, Resize, StatefulImage, picker::Picker, protocol::StatefulProtocol,
};

use crate::model::Model;

pub fn draw(area: Rect, frame: &mut Frame, model: &mut Model) {
    if let Some(ref current_track) = model.player.current
        && let Some(ref tag) = current_track.tagged_file
    {
        let picture = tag.primary_tag().unwrap().pictures().first().unwrap();
        let mut img = get_image_from_picture(picture).unwrap();

        let widget = StatefulImage::default().resize(Resize::Scale(None));
        frame.render_stateful_widget(widget, area, &mut img);
    }
}

fn get_image_from_picture(picture: &Picture) -> Result<StatefulProtocol> {
    

    let picker = Picker::from_query_stdio()?;

    let image = picker.new_resize_protocol(dyn_img);

    Ok(image)
}
