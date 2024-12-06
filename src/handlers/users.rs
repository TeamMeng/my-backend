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
