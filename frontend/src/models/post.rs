use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
}
