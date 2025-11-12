use axum::{Router, routing::get_service};
use color_eyre::eyre::Result;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::global::logic::data::get_art_cache_path;

pub const COVER_ART_ROUTE: &str = "/cover-art";

pub async fn run_cover_art_server(listener: TcpListener) -> Result<()> {
    let app = Router::new().nest_service(
        COVER_ART_ROUTE,
        get_service(ServeDir::new(get_art_cache_path()?)),
    );
    tokio::spawn(async move {
        log::info!(
            "Cover art server starting at {:?}...",
            listener.local_addr()
        );
        if let Err(e) = axum::serve(listener, app.into_make_service()).await {
            log::error!("Error serving: {e}");
        }

        log::info!("Server stopped.");
    });

    Ok(())
}
