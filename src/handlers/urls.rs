use crate::{AppError, AppState, CreateUrl, User};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use reqwest::StatusCode;

pub async fn shorten_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateUrl>,
) -> Result<impl IntoResponse, AppError> {
    let id = state.shorten(user.id, input).await?;

    Ok((StatusCode::OK, Json(id)))
}

pub async fn get_all_urls_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let ret = state.get_all_url(user.id).await?;

    Ok((StatusCode::OK, Json(ret)))
}

pub async fn redirect_handler(
    Path(id): Path<String>,
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let url = state.get_url(user.id, id).await?;

    Ok((StatusCode::OK, Json(url)))
}
