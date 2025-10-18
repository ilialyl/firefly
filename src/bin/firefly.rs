use std::{
    collections::VecDeque,
    fs,
    sync::mpsc::{self},
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
        view::draw,
        view_logic::terminal::{
            handle_events, init_terminal, install_panic_hook, restore_terminal,
        },
    },
    model::Model,
};

#[cfg(not(target_os = "windows"))]
use firefly::global::logic::cli::display_nlog;

#[allow(clippy::single_match)]
fn main() -> Result<()> {
    dpi::enable_dpi_awareness();
    color_eyre::install()?;
    setup_logger()?;

    let cache_dir = get_cache_dir();
    if !cache_dir.exists() {
        fs::create_dir(&cache_dir)?;
    }

    // Clap stuff
    let cli_command = cli().get_matches();

    match cli_command.subcommand() {
        Some(("clean", _)) => {
            clear_cache(&cache_dir)?;
            println!("Success");
            return Ok(());
        }
        Some(("log", args)) => {
            if let Some(_n_line) = args.get_one::<usize>("nlines") {
                println!();
                #[cfg(not(target_os = "windows"))]
                display_nlog(*_n_line);
                println!();
            }

            return Ok(());
        }
        _ => {}
    };

    install_panic_hook();
    let (msg_tx, msg_rx) = mpsc::channel::<Message>();
    let (info_tx, info_rx) = mpsc::channel::<String>();
    let mut model = Model::new(msg_tx.clone());
    let mut terminal = init_terminal()?;

    while model.session.state != RunningState::Exit {
        // VecDeque is better for queues
        let mut msg_queue: VecDeque<Message> = VecDeque::new();

        // Tick at the start to keep things updated
        if let Some(msg) = update_global(&mut model, Message::Tick, &msg_tx, &info_tx) {
            msg_queue.push_back(msg);
        }

        // Draw TUI view
        terminal.draw(|f| draw(&mut model, f))?;

        // Handle terminal events
        if let Some(msg) = handle_events(&model)? {
            msg_queue.push_back(msg);
        }

        // Receive message from other threads
        if let Ok(msg) = msg_rx.try_recv() {
            msg_queue.push_back(msg);
        }

        // Display info sent from other threads
        if let Ok(info) = info_rx.try_recv() {
            update_global(&mut model, Message::UpdateInfoMsg(info), &msg_tx, &info_tx);
        }

        // Consume messages
        while !msg_queue.is_empty()
            && let Some(msg) = msg_queue.pop_front()
        {
            if let Some(msg) = update_global(&mut model, msg, &msg_tx, &info_tx) {
                msg_queue.push_back(msg);
            }
        }
    }

    // Give terminal back to user
    restore_terminal()?;

    // Tell user they can clean up if they need to
    if fs::read_dir(&cache_dir)?.count() != 0 {
        println!("run \"firefly clean\" or \"cargo run --release -- clean\" to clear cache.");
    }

    Ok(())
}

#[cfg(target_os = "windows")]
mod dpi {
    use windows_sys::Win32::Foundation::E_FAIL;
    use windows_sys::Win32::UI::HiDpi::{PROCESS_PER_MONITOR_DPI_AWARE, SetProcessDpiAwareness};

    // Otherwise file dialog on Windows will be blurry
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
