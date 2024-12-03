use anyhow::Result;
use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, post},
    Router,
};
use my_backend::{create_user_handler, delete_user_handler, login_handler, verify_token, AppState};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::Layer, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer as _,
};

const ADDR: &str = "127.0.0.1:";

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState::new().await?;

    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    registry().with(layer).init();

    let addr = format!("{}{}", ADDR, state.config.server.port);

    info!("Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;

    let app = Router::new()
        .route("/delete", delete(delete_user_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/login", post(login_handler))
        .route("/signup", post(create_user_handler))
        .route("/", get(index_handler))
        .with_state(state);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn index_handler() -> &'static str {
    "Hello World"
}
