use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FileEntry {
    pub id: uuid::Uuid,
    pub sha256: String,
    pub mime: String,
    pub kind: String,
    pub bytes: i64,
    pub original_name: String,
    pub created_at: DateTime<Utc>,
}
