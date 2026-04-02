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
