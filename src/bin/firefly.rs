use std::{
    fs,
    sync::mpsc::{self, Receiver, Sender},
};

use color_eyre::eyre::Result;

use firefly::{
    global::{
        logic::{
            cli::{clear_cache, cli},
            data::get_cache_dir,
            logger::setup_logger,
            session_state::RunningState,
        },
        message::Message,
        update::update_global,
        view::render_tui,
        view_logic::terminal::{
            handle_events, init_terminal, install_panic_hook, restore_terminal,
        },
    },
    model::Model,
};

#[allow(clippy::single_match)]
fn main() -> Result<()> {
    dpi::enable_dpi_awareness();
    color_eyre::install()?;
    setup_logger()?;

    let cache_dir = get_cache_dir();
    if !cache_dir.exists() {
        fs::create_dir(&cache_dir).expect("Failed to create cache directory.");
    }

    let cli_command = cli().get_matches();

    match cli_command.subcommand() {
        Some(("clean", _)) => {
            clear_cache(&cache_dir)?;
            println!("Success");
            return Ok(());
        }
        _ => {}
    };

    install_panic_hook();
    let mut terminal = init_terminal()?;
    let mut model = Model::default();
    let (msg_tx, msg_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (info_tx, info_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    while model.session.state != RunningState::Done {
        let mut current_msg;

        // Tick
        current_msg = update_global(&mut model, Message::Tick, &msg_tx, &info_tx);
        while current_msg.is_some() {
            current_msg = update_global(&mut model, current_msg.unwrap(), &msg_tx, &info_tx);
        }

        // Draw TUI view
        terminal.draw(|f| render_tui(&mut model, f))?;

        // Handle terminal events
        current_msg = handle_events(&model)?;

        // Consume message
        while current_msg.is_some() {
            current_msg = update_global(&mut model, current_msg.unwrap(), &msg_tx, &info_tx);
        }

        // Receive message from other threads and consume it.
        if let Ok(msg) = msg_rx.try_recv() {
            current_msg = update_global(&mut model, msg, &msg_tx, &info_tx);
            while current_msg.is_some() {
                current_msg = update_global(&mut model, current_msg.unwrap(), &msg_tx, &info_tx);
            }
        }

        // Receive displayable info from other threads and consume it.
        if let Ok(info) = info_rx.try_recv() {
            current_msg = update_global(
                &mut model,
                Message::UpdateStatusMsg(info),
                &msg_tx,
                &info_tx,
            );
            while current_msg.is_some() {
                current_msg = update_global(&mut model, current_msg.unwrap(), &msg_tx, &info_tx);
            }
        }
    }

    restore_terminal()?;

    // If cache dir is not empty
    if fs::read_dir(&cache_dir).unwrap().count() != 0 {
        println!("run \"firefly clean\" or \"cargo run --release -- clean\" to clear cache.");
    }

    println!("Thank you for using Firefly.");

    Ok(())
}

#[cfg(target_os = "windows")]
mod dpi {
    use windows_sys::Win32::Foundation::E_FAIL;
    use windows_sys::Win32::UI::HiDpi::{PROCESS_PER_MONITOR_DPI_AWARE, SetProcessDpiAwareness};

    pub fn enable_dpi_awareness() {
        unsafe {
            let result = SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
            if result != 0 && result != E_FAIL {
                eprintln!("Failed to set DPI awareness, error code: {}", result);
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod dpi {
    pub fn enable_dpi_awareness() {}
}
