pub mod routes;
pub mod state;
pub mod ui;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

pub use routes::build_router;
pub use state::AppState;

pub async fn run_server(state: AppState, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = build_router(state);
    let listener = TcpListener::bind(addr).await?;
    info!("🚀 Paperclip control plane running on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
