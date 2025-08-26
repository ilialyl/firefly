use std::path::PathBuf;

use color_eyre::eyre::Result;

use crate::{
    message::Message,
    model::{Model, RunningState, player::CONVERTED_TRACK},
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

    let track_temp: PathBuf = CONVERTED_TRACK.clone();
    if track_temp.exists() {
        std::fs::remove_file(track_temp).expect("Error removing temporary file.");
    }

    view::terminal::restore_terminal()
}
