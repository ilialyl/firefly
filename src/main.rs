use std::{
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
};

use color_eyre::eyre::{Result, eyre};

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
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    while model.running_state != RunningState::Done {
        let mut result;
        let mut current_msg;

        (_, result) = message::update(&mut model, Message::Tick, &tx);

        attach_errors(&result)?;

        terminal.draw(|f| view(&mut model, f))?;

        current_msg = message::handle_events()?;

        while current_msg.is_some() {
            (current_msg, result) = message::update(&mut model, current_msg.unwrap(), &tx);
            attach_errors(&result)?;
        }

        if let Ok(msg) = rx.try_recv() {
            (current_msg, result) = message::update(&mut model, msg, &tx);
            while current_msg.is_some() {
                (current_msg, result) = message::update(&mut model, current_msg.unwrap(), &tx);
            }
            attach_errors(&result)?;
        }
    }

    clean_up()
}

fn clean_up() -> Result<()> {
    let track_temp: PathBuf = CONVERTED_TRACK.clone();
    if track_temp.exists() {
        std::fs::remove_file(track_temp)?;
    }

    view::terminal::restore_terminal()
}

fn attach_errors(result: &Result<()>) -> Result<()> {
    match result {
        Ok(_) => Ok(()),
        Err(e) => match clean_up() {
            Ok(_) => Err(eyre!(e.to_string())),
            Err(clean_err) => Err(eyre!("{}\nCleanup also failed: {}", e, clean_err)),
        },
    }
}
