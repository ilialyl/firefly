use std::{
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    time::SystemTime,
};

use color_eyre::eyre::{Result, eyre};
use log::info;

use crate::{
    logic::player,
    message::{Message, update::update},
    model::{Model, RunningState},
    view::view,
};

pub mod logic;
pub mod message;
pub mod model;
pub mod view;

fn main() -> Result<()> {
    view::terminal::install_panic_hook();
    color_eyre::install()?;
    setup_logger()?;
    info!("\n\n\nStarting a new session.");

    let mut terminal = view::terminal::init_terminal()?;
    let mut model = Model::default();
    let (msg_tx, msg_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (info_tx, info_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    while model.running_state != RunningState::Done {
        let mut result;
        let mut current_msg;

        (_, result) = update(&mut model, Message::Tick, &msg_tx, &info_tx);

        attach_errors(&result)?;

        terminal.draw(|f| view(&model, f))?;

        current_msg = message::handle::events()?;

        while current_msg.is_some() {
            (current_msg, result) = update(&mut model, current_msg.unwrap(), &msg_tx, &info_tx);
            attach_errors(&result)?;
        }

        if let Ok(msg) = msg_rx.try_recv() {
            (current_msg, result) = update(&mut model, msg, &msg_tx, &info_tx);
            while current_msg.is_some() {
                (current_msg, result) = update(&mut model, current_msg.unwrap(), &msg_tx, &info_tx);
            }
            attach_errors(&result)?;
        }

        if let Ok(info) = info_rx.try_recv() {
            model.info_display = info;
        }
    }

    clean_up()
}

fn clean_up() -> Result<()> {
    let track_temp: PathBuf = player::CONVERTED_TRACK.clone();
    if track_temp.exists() {
        std::fs::remove_file(track_temp)?;
    }

    info!("Cleaned up, restoring terminal.");

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

fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
