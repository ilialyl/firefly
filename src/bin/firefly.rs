use std::{
    collections::VecDeque,
    fs,
    path::PathBuf,
    sync::mpsc::{self},
};

use color_eyre::eyre::Result;

use firefly_music::global::logic::{data::clear_art_cache, senders::Senders};
use firefly_music::{
    app::App,
    global::{
        logic::{
            cli::cli,
            data::{clear_all_cache, get_cache_dir},
            files::get_playlists_path,
            logger::{get_log_path, setup_logger},
            session_state::SessionState,
        },
        message::Message,
        update::update_global,
        view::draw,
    },
    queue::message::QueueMessage,
    tui::{handle_events, init_terminal, install_panic_hook, restore_terminal},
};

#[tokio::main]
async fn main() -> Result<()> {
    dpi::enable_dpi_awareness();
    color_eyre::install()?;
    setup_logger()?;

    let (msg_tx, msg_rx) = mpsc::channel::<Message>();
    let (info_tx, info_rx) = mpsc::channel::<String>();
    let (msg_async_tx, mut msg_async_rx) = tokio::sync::mpsc::unbounded_channel();
    let senders = Senders {
        msg: msg_tx,
        info: info_tx,
        async_msg: msg_async_tx,
    };
    let mut app = App::new(senders).await?;
    if cfg!(target_os = "linux") {
        app.cover_art_server.run_server().await?;
    }

    // Clap stuff
    let cli_command = cli().get_matches();

    match cli_command.subcommand() {
        Some(("clean", _)) => {
            clear_all_cache()?;
            return Ok(());
        }
        Some(("log", _)) => {
            println!("{}", get_log_path()?.display());
            return Ok(());
        }
        Some(("playlist", _)) => {
            println!("{}", get_playlists_path()?.display());
            return Ok(());
        }
        Some(("with", args)) => {
            if let Some(path_strs) = args.get_many::<String>("paths") {
                let paths: Vec<PathBuf> = path_strs.map(PathBuf::from).collect();
                let valid_paths: Vec<PathBuf> =
                    paths.into_iter().filter(|path| path.exists()).collect();
                if valid_paths.is_empty() {
                    eprintln!("No path is valid.");
                    return Ok(());
                }
                app.senders
                    .msg
                    .send(Message::Queue(QueueMessage::QueuePaths(valid_paths)))
                    .expect("Error sending queue.");
            }
        }
        _ => {}
    };

    let mut terminal = init_terminal()?;
    install_panic_hook();

    while app.session_state != SessionState::Exit {
        // VecDeque is better for queues
        let mut msg_queue: VecDeque<Message> = VecDeque::new();

        // Tick at the start to keep things updated
        if let Some(msg) = update_global(&mut app, Message::Tick).await {
            msg_queue.push_back(msg);
        }

        // Draw TUI view
        terminal.draw(|f| draw(&mut app, f))?;

        // Handle terminal events
        if let Some(msg) = handle_events(&app)? {
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
            update_global(&mut app, Message::UpdateInfoMsg(info)).await;
        }

        // Consume messages
        while !msg_queue.is_empty()
            && let Some(msg) = msg_queue.pop_front()
        {
            if let Some(msg) = update_global(&mut app, msg).await {
                msg_queue.push_back(msg);
            }
        }
    }

    // Give terminal back to user
    restore_terminal()?;

    clear_art_cache()?;
    // Tell user they can clean up if they need to
    if fs::read_dir(get_cache_dir()?)?.count() != 0 {
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
