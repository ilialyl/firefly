use axum::{Router, routing::get_service};
use color_eyre::eyre::{Result, eyre};
use local_ip_address::local_ip;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::global::logic::data::get_art_cache_path;

pub const COVER_ART_ROUTE: &str = "/cover-art";

pub struct CoverArtServer {
    listener: Option<TcpListener>,
    pub addr: Option<SocketAddr>,
}

impl CoverArtServer {
    pub async fn new() -> Result<CoverArtServer> {
        let (addr, listener) = if cfg!(target_os = "linux") {
            let listener = TcpListener::bind(format!("{}:0", local_ip()?)).await?;
            let addr = listener.local_addr()?;

            (Some(addr), Some(listener))
        } else {
            (None, None)
        };

        Ok(CoverArtServer { listener, addr })
    }

    pub async fn run_server(&mut self) -> Result<()> {
        if let Some(listener) = self.listener.take() {
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
        } else {
            Err(eyre!("Listener is None."))
        }
    }
}
