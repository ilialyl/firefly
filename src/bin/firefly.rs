use std::fs;

use color_eyre::eyre::Result;

use firefly_music::{
    app::App,
    global::logic::{
        data::get_cache_dir, logger::setup_logger, windows::dpi::enable_dpi_awareness,
    },
    tui::{init_terminal, install_panic_hook, restore_terminal},
};
use firefly_music::{cli::handle_cli_commands, global::logic::data::clear_art_cache};

#[tokio::main]
async fn main() -> Result<()> {
    enable_dpi_awareness();
    color_eyre::install()?;
    setup_logger()?;

    let mut app = App::new().await?;

    if handle_cli_commands(&app)? {
        return Ok(());
    }

    let mut terminal = init_terminal()?;
    install_panic_hook();

    app.run(&mut terminal).await?;

    // Give terminal back to user
    restore_terminal()?;

    clear_art_cache()?;

    // Tell user they can clean up if they need to
    if fs::read_dir(get_cache_dir()?)?.count() != 0 {
        println!("run \"firefly clean\" or \"cargo run --release -- clean\" to clear FLAC cache.");
    }

    Ok(())
}
