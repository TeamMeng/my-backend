use crate::{AppError, AppState, ChangeUser, CreateUser, LoginUser, User};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthOutput {
    token: String,
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    state.create_user(input).await?;
    Ok(StatusCode::CREATED)
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(input): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.login_user(input).await?;
    let token = state.ek.sign(user)?;
    Ok((StatusCode::OK, Json(AuthOutput { token })))
}

pub async fn delete_user_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_user_by_email(&user.email).await?;
    Ok(StatusCode::OK)
}

pub async fn change_user_message_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<ChangeUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.change_user_message(user, input).await?;
    let token = state.ek.sign(user)?;
    Ok((StatusCode::OK, Json(AuthOutput { token })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_router;
    use anyhow::Result;
    use reqwest::Client;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;

    const ADDR: &str = "0.0.0.0:0";

    struct Server {
        addr: SocketAddr,
        token: String,
        client: Client,
    }

    #[derive(Deserialize)]
    struct AuthToken {
        token: String,
    }

    #[tokio::test]
    async fn server_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let mut server = Server::new(state).await?;
        server.token = server.change().await?;
        server.delete().await?;

        Ok(())
    }

    impl Server {
        async fn new(state: AppState) -> Result<Self> {
            let app = get_router(state)?;
            let listener = TcpListener::bind(ADDR).await?;
            let addr = listener.local_addr()?;

            tokio::spawn(async move {
                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap()
            });

            let client = Client::new();

            let mut ret = Self {
                addr,
                client,
                token: "".to_string(),
            };

            ret.signup().await?;

            ret.token = ret.login().await?;

            Ok(ret)
        }

        async fn signup(&self) -> Result<()> {
            let res = self
                .client
                .post(format!("http://{}/signup", self.addr))
                .header("Content-Type", "application/json")
                .body(r#"{"email": "Meng@acme.org","name": "TeamMeng", "password":"123456"}"#)
                .send()
                .await?;

            assert_eq!(res.status(), 201);

            Ok(())
        }

        async fn login(&self) -> Result<String> {
            let res = self
                .client
                .post(format!("http://{}/login", self.addr))
                .header("Content-Type", "application/json")
                .body(r#"{"email": "Meng@acme.org", "password":"123456"}"#)
                .send()
                .await?;

            assert_eq!(res.status(), 200);

            let ret: AuthToken = res.json().await?;

            Ok(ret.token)
        }

        async fn change(&self) -> Result<String> {
            let res = self
                .client
                .post(format!("http://{}/change", self.addr))
                .header("Authorization", format!("Bearer {}", self.token))
                .header("Content-Type", "application/json")
                .body(r#"{"email": "Alice@acme.org", "name": "TeamAlice", "password":"hunter42"}"#)
                .send()
                .await?;

            assert_eq!(res.status(), 200);

            let ret: AuthToken = res.json().await?;
            Ok(ret.token)
        }

        async fn delete(&self) -> Result<()> {
            let res = self
                .client
                .delete(format!("http://{}/delete", self.addr))
                .header("Authorization", format!("Bearer {}", self.token))
                .send()
                .await?;

            assert_eq!(res.status(), 200);

            Ok(())
        }
    }
}
