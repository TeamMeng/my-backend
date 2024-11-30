use anyhow::Result;
use axum::{routing::get, Router};
use my_backend::AppState;
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
        .route("/", get(index_handler))
        .with_state(state);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn index_handler() -> &'static str {
    "Hello World"
}
