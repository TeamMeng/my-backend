use anyhow::Result;
use my_backend::{get_router, AppState, ADDR};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::Layer, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer as _,
};

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState::new().await?;

    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    registry().with(layer).init();

    let addr = format!("{}{}", ADDR, state.config.server.port);

    info!("Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;

    let app = get_router(state)?;
    axum::serve(listener, app).await?;

    Ok(())
}
