use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct StoredLink {
    pub original_url: String,
    pub short_code: String,
    pub clicks: u32,
}

#[derive(Deserialize)]
pub struct CreateLink {
    pub original_url: String,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub password_hash: String,
}
