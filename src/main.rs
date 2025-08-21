use std::path::Path;

use color_eyre::eyre::Result;

use crate::{
    message::Message,
    model::{Model, RunningState},
    view::view,
};

pub mod message;
pub mod model;
pub mod view;

fn main() -> Result<()> {
    view::terminal::install_panic_hook();
    color_eyre::install()?;

    let mut terminal = view::terminal::init_terminal()?;
    let mut model = Model::default();

    while model.running_state != RunningState::Done {
        message::update(&mut model, Message::Tick, &mut terminal);
        terminal.draw(|f| view(&mut model, f))?;

        let mut current_msg = message::handle_events()?;

        while current_msg.is_some() {
            current_msg = message::update(&mut model, current_msg.unwrap(), &mut terminal);
        }
    }

    let temp_file = Path::new(model::player::CONVERTED_TRACK);
    if temp_file.exists() {
        std::fs::remove_file(temp_file).expect("Error removing temporary file.");
    }

    view::terminal::restore_terminal()
}
