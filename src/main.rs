use axum::Router;
use axum::extract::State;
use axum::routing::{delete, get, post};
mod handlers;
mod models;
use handlers::auth::register;
use handlers::links::{create_link, delete_link, get_stats, redirect};
//use models::{CreateLink, StoredLink};
//use std::collections::HashMap;
//use std::sync::{Arc, Mutex};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

//type AppState = Arc<Mutex<HashMap<String, StoredLink>>>;
type AppState = SqlitePool;
#[tokio::main]
async fn main() {
    //let state: AppState = Arc::new(Mutex::new(HashMap::new()));
    let state: AppState = SqlitePool::connect("sqlite:urls.db?mode=rwc")
        .await
        .unwrap();

    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&state)
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        )",
    )
    .execute(&state)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS links (
            short_code TEXT PRIMARY KEY,
            original_url TEXT NOT NULL,
            clicks INTEGER NOT NULL DEFAULT 0,
            user_id INTEGER REFERENCES users(id)
        )",
    )
    .execute(&state)
    .await
    .unwrap();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/shorten", post(create_link))
        .route("/{code}", get(redirect))
        .route("/stats/{code}", get(get_stats))
        .route("/delete/{code}", delete(delete_link))
        .route("/auth/register", post(register))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
