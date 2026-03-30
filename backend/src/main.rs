mod handlers;
mod models;
mod router;
mod state;

use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .compact()
        .with_file(false)
        .without_time()
        .init();

    let addr = "0.0.0.0:3000";

    info!("booting_server");

    let app_state = state::AppState::new();
    info!("app_state_initialized");

    let app = router::get_router(app_state);
    info!("router_initialized");

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => {
            info!(address = %addr, "tcp_listener_bound");
            l
        }
        Err(err) => {
            error!(address = %addr, error = %err, "failed_to_bind");
            return;
        }
    };

    info!(address = %addr, "server_listening");

    if let Err(err) = axum::serve(listener, app).await {
        error!(error = %err, "server_crashed");
    }
}
