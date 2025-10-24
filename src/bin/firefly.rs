use std::{
    collections::VecDeque,
    fs,
    path::PathBuf,
    sync::mpsc::{self},
};

use async_std::task;
use color_eyre::eyre::{Result, eyre};

use firefly::global::logic::{mpris::run_server, senders::Senders};
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

#[async_std::main]
async fn main() -> Result<()> {
    dpi::enable_dpi_awareness();
    color_eyre::install()?;
    setup_logger()?;

    let cache_dir = get_cache_dir();
    if !cache_dir.exists() {
        fs::create_dir(&cache_dir)?;
    }

    let (msg_tx, msg_rx) = mpsc::channel::<Message>();
    let (info_tx, info_rx) = mpsc::channel::<String>();
    let senders = Senders {
        msg: msg_tx,
        info: info_tx,
    };
    let mut model = Model::new(senders);

    let (msg_async_tx, msg_async_rx) = async_std::channel::unbounded::<Message>();
    std::thread::spawn(move || {
        task::block_on(async {
            if let Err(e) = run_server(msg_async_tx).await {
                eprintln!("Server error: {e}");
            }
        });
    });

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
        Some(("with", args)) => {
            if let Some(path_strs) = args.get_many::<String>("paths") {
                let paths: Vec<PathBuf> = path_strs.map(PathBuf::from).collect();
                let valid_paths: Vec<PathBuf> =
                    paths.into_iter().filter(|path| path.exists()).collect();
                if valid_paths.is_empty() {
                    return Err(eyre!("No path is valid."));
                }
                model
                    .senders
                    .msg
                    .send(Message::Queue(QueueMessage::QueuePaths(valid_paths)))
                    .unwrap();
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
        if let Some(msg) = update_global(&mut model, Message::Tick) {
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

        // Async receiver
        while let Ok(msg) = msg_async_rx.try_recv() {
            msg_queue.push_back(msg);
        }

        // Display info sent from other threads
        if let Ok(info) = info_rx.try_recv() {
            update_global(&mut model, Message::UpdateInfoMsg(info));
        }

        // Consume messages
        while !msg_queue.is_empty()
            && let Some(msg) = msg_queue.pop_front()
        {
            if let Some(msg) = update_global(&mut model, msg) {
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
