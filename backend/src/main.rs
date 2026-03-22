mod handlers;
mod models;
mod router;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_state = state::AppState::new();
    let app = router::get_router(app_state);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Server listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
