use std::{
    fs,
    path::Path,
    sync::mpsc::{self, Receiver, Sender},
    time::SystemTime,
};

use color_eyre::eyre::Result;
use glob::glob;
use log::info;

use crate::{
    logic::session_state::{RunningState, Session},
    message::{Message, update::update},
    model::Model,
    view::{terminal, view},
};

pub mod logic;
pub mod message;
pub mod model;
pub mod view;

#[cfg(test)]
mod tests;

const TEMP_DIR: &str = "firefly_temp";

fn main() -> Result<()> {
    view::terminal::install_panic_hook();
    color_eyre::install()?;
    setup_logger()?;

    let temp_dir = Path::new(TEMP_DIR);
    if !temp_dir.exists() {
        fs::create_dir(temp_dir).expect("Failed to create temp directory");
    }

    let mut terminal = view::terminal::init_terminal()?;
    let mut model = Model::default();
    let (msg_tx, msg_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (info_tx, info_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    info!("\n\n\nStarting session {}.", model.session.get_code());

    while model.session.state != RunningState::Done {
        let mut current_msg;

        // Tick
        update(&mut model, Message::Tick, &msg_tx, &info_tx);

        // Draw TUI view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle terminal events
        current_msg = terminal::handle_events()?;

        // Consume message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap(), &msg_tx, &info_tx);
        }

        // Receive message from other threads and consume it.
        if let Ok(msg) = msg_rx.try_recv() {
            current_msg = update(&mut model, msg, &msg_tx, &info_tx);
            while current_msg.is_some() {
                current_msg = update(&mut model, current_msg.unwrap(), &msg_tx, &info_tx);
            }
        }

        // Receive displayable info from other threads and consume it.
        if let Ok(info) = info_rx.try_recv() {
            current_msg = update(&mut model, Message::UpdateInfo(info), &msg_tx, &info_tx);
            while current_msg.is_some() {
                current_msg = update(&mut model, current_msg.unwrap(), &msg_tx, &info_tx);
            }
        }
    }

    clean_up(&model.session)
}

fn clean_up(session: &Session) -> Result<()> {
    let temp_pattern = glob(format!("firefly_temp/{}*", session.get_code()).as_str());
    for path in temp_pattern.expect("Failed to read glob pattern") {
        match path {
            Ok(path) => {
                if path.is_file() {
                    fs::remove_file(&path)?;
                    println!("Deleted {:?}", path);
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    info!("Cleaned up, restoring terminal.");

    view::terminal::restore_terminal()
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
