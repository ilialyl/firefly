use std::{
    collections::VecDeque,
    fs,
    path::Path,
    sync::mpsc::{self},
};

use color_eyre::eyre::Result;

use firefly::{
    global::{
        logic::{
            cli::cli,
            data::{clear_cache, get_cache_dir},
            files::get_playlists_path,
            logger::{get_log_path, setup_logger},
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
    queue::message::QueueMessage,
};

fn main() -> Result<()> {
    dpi::enable_dpi_awareness();
    color_eyre::install()?;
    setup_logger()?;

    let cache_dir = get_cache_dir();
    if !cache_dir.exists() {
        fs::create_dir(&cache_dir)?;
    }

    let (msg_tx, msg_rx) = mpsc::channel::<Message>();
    let (info_tx, info_rx) = mpsc::channel::<String>();
    let mut model = Model::new(msg_tx.clone());

    // Clap stuff
    let cli_command = cli().get_matches();

    match cli_command.subcommand() {
        Some(("clean", _)) => {
            clear_cache(&cache_dir)?;
            println!("Success");
            return Ok(());
        }
        Some(("log", _)) => {
            println!("{}", get_log_path().display());
            return Ok(());
        }
        Some(("playlist", _)) => {
            println!("{}", get_playlists_path().display());
            return Ok(());
        }
        Some(("add", args)) => {
            if let Some(path_str) = args.get_one::<String>("path") {
                let path = Path::new(path_str);
                if path.exists() {
                    if path.is_file() {
                        println!("Your file is {:?}.", path);
                        msg_tx
                            .send(Message::Queue(QueueMessage::QueueFileManually(
                                path.to_path_buf(),
                            )))
                            .unwrap()
                    } else if path.is_dir() {
                        println!("Your directory is {:?}.", path);
                        msg_tx
                            .send(Message::Queue(QueueMessage::QueueDirManually(
                                path.to_path_buf(),
                            )))
                            .unwrap()
                    }
                } else {
                    println!("Path not found.");
                    return Ok(());
                }
            }
        }
        _ => {}
    };

    let mut terminal = init_terminal()?;
    install_panic_hook();

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
        while let Ok(msg) = msg_rx.try_recv() {
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
