use std::hash::Hash;

use crate::AppState;
use crate::models::{AuthUser,  User};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use tokio::task;

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<AuthUser>,
) -> Result<StatusCode, (StatusCode, String)> {
    let hashed_password = task::spawn_blocking(move || hash(&body.password, DEFAULT_COST))
        .await
        .unwrap()
        .unwrap();

    match sqlx::query("INSERT INTO users (email, password_hash) VALUES (?,?)")
        .bind(&body.email)
        .bind(&hashed_password)
        .execute(&state)
        .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err((StatusCode::BAD_REQUEST, String::from("cant register user"))),
    }
}
