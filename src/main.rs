use std::path::Path;

use color_eyre::eyre::Result;

use crate::ui::{
    message::{Message, handle_events, update},
    model::{Model, RunningState},
    view::view,
};

pub mod player;
pub mod ui;
pub mod update;

fn main() -> Result<()> {
    ui::install_panic_hook();
    color_eyre::install()?;

    let mut terminal = ui::init_terminal()?;
    let mut model = Model::default();

    while model.running_state != RunningState::Done {
        update(&mut model, Message::Tick);
        terminal.draw(|f| view(&mut model, f))?;

        let mut current_msg = handle_events()?;

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    let temp_file = Path::new(player::CONVERTED_TRACK);
    if temp_file.exists() {
        std::fs::remove_file(temp_file).expect("Error removing temporary file.");
    }

    ui::restore_terminal();
    Ok(())
}
