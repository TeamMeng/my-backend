mod config;
mod error;
mod handler;
mod middleware;
mod model;
mod util;

use axum::{
    middleware::from_fn_with_state,
    routing::{delete, post},
    Router,
};
use sqlx::{Executor, PgPool};
use sqlx_db_tester::TestPg;
use std::{ops::Deref, path::Path, sync::Arc};

pub use config::AppConfig;
pub use error::AppError;
pub use handler::{
    change_user_message_handler, create_user_handler, delete_user_handler, login_handler,
};
pub use middleware::verify_token;
pub use model::{ChangeUser, CreateUser, LoginUser, User};
pub use util::{DecodingKey, EncodingKey};

pub const ADDR: &str = "127.0.0.1:";

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub config: AppConfig,
    pub pool: PgPool,
    pub ek: EncodingKey,
    pub dk: DecodingKey,
}

pub fn get_router(state: AppState) -> Result<Router, AppError> {
    let app = Router::new()
        .route("/delete", delete(delete_user_handler))
        .route("/change", post(change_user_message_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/login", post(login_handler))
        .route("/signup", post(create_user_handler))
        .with_state(state);

    Ok(app)
}

impl AppState {
    pub async fn new() -> Result<Self, AppError> {
        let config = AppConfig::new()?;
        let ek = EncodingKey::new(&config.auth.ek)?;
        let dk = DecodingKey::new(&config.auth.dk)?;

        let pool = PgPool::connect(&config.server.db_url).await?;

        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                pool,
                ek,
                dk,
            }),
        })
    }

    pub async fn new_for_test() -> Result<(TestPg, Self), AppError> {
        let config = AppConfig::new()?;
        let ek = EncodingKey::new(&config.auth.ek)?;
        let dk = DecodingKey::new(&config.auth.dk)?;

        let post = config
            .server
            .db_url
            .rfind('/')
            .expect("Database url should invalid");

        let database_url = &config.server.db_url[..post];
        let tdb = TestPg::new(database_url.to_string(), Path::new("./migrations"));

        let pool = tdb.get_pool().await;

        let sql = include_str!("../fixtures/test.sql").split(';');
        let mut ts = pool.begin().await.expect("begin transaction failed");
        for s in sql {
            if s.trim().is_empty() {
                continue;
            }
            ts.execute(s).await.expect("execute sql failed");
        }
        ts.commit().await.expect("commit transaction failed");

        Ok((
            tdb,
            Self {
                inner: Arc::new(AppStateInner {
                    config,
                    pool,
                    ek,
                    dk,
                }),
            },
        ))
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
