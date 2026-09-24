use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived: bool,
    pub word_count: i32,
    pub modify_count: i32,
}

#[derive(Serialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostSummary {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub archived: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
