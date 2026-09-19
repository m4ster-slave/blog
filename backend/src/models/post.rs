use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
