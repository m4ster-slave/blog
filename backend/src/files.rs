use crate::auth::AuthUser;
use crate::models::devlog::Devlog;
use axum::extract::{Path, State};
use axum::{Json, http::StatusCode};
use axum::{
    body::Bytes,
    extract::FromRequest,
    http::{Request, header},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::prelude::FromRow;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use crate::AppState;

#[derive(Debug, PartialEq, Eq)]
pub enum AllowedMime {
    // Images
    Jpeg,
    Png,
    Webp,
    Gif,
    // Archives & Documents
    Zip,
    Pdf,
    Gzip,
    Tar,
    Xz,
    Zstd,
    SevenZip,
    // Text
    TextPlain,
    TextCsv,
    TextMarkdown,
    Json,
}

impl AllowedMime {
    /// O(1) static string pattern match
    pub fn parse(raw_mime: &str) -> Option<Self> {
        // Strip parameters like '; charset=utf-8' or boundary tags
        let base_mime = raw_mime.split(';').next()?.trim();

        match base_mime {
            "image/jpeg" => Some(Self::Jpeg),
            "image/png" => Some(Self::Png),
            "image/webp" => Some(Self::Webp),
            "image/gif" => Some(Self::Gif),

            "application/zip" => Some(Self::Zip),
            "application/pdf" => Some(Self::Pdf),
            "application/gzip" => Some(Self::Gzip),
            "application/x-tar" => Some(Self::Tar),
            "application/x-xz" => Some(Self::Xz),
            "application/zstd" => Some(Self::Zstd),
            "application/x-7z-compressed" => Some(Self::SevenZip),

            "text/plain" => Some(Self::TextPlain),
            "text/csv" => Some(Self::TextCsv),
            "text/markdown" => Some(Self::TextMarkdown),
            "application/json" => Some(Self::Json),

            _ => None,
        }
    }

    pub const fn get_kind(&self) -> &'static str {
        match self {
            Self::Jpeg | Self::Png | Self::Webp | Self::Gif => "image",
            _ => "file",
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            // Images
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Webp => "image/webp",
            Self::Gif => "image/gif",

            // Archives & Documents
            Self::Zip => "application/zip",
            Self::Pdf => "application/pdf",
            Self::Gzip => "application/gzip",
            Self::Tar => "application/x-tar",
            Self::Xz => "application/x-xz",
            Self::Zstd => "application/zstd",
            Self::SevenZip => "application/x-7z-compressed",

            // Text
            Self::TextPlain => "text/plain",
            Self::TextCsv => "text/csv",
            Self::TextMarkdown => "text/markdown",
            Self::Json => "application/json",
        }
    }
}

/// Validates magic bytes against expected MIME target
pub fn verify_magic_bytes(mime: &AllowedMime, header: &[u8]) -> bool {
    match mime {
        // Images
        AllowedMime::Jpeg => header.starts_with(&[0xFF, 0xD8, 0xFF]),
        AllowedMime::Png => header.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]),
        AllowedMime::Gif => header.starts_with(b"GIF87a") || header.starts_with(b"GIF89a"),
        AllowedMime::Webp => {
            header.len() >= 12 && &header[0..4] == b"RIFF" && &header[8..12] == b"WEBP"
        }

        // Archives / Documents
        AllowedMime::Zip => header.starts_with(&[0x50, 0x4B, 0x03, 0x04]),
        AllowedMime::Pdf => header.starts_with(b"%PDF-"),
        AllowedMime::Gzip => header.starts_with(&[0x1F, 0x8B]),
        AllowedMime::Xz => header.starts_with(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]),
        AllowedMime::Zstd => header.starts_with(&[0x28, 0xB5, 0x2F, 0xFD]),
        AllowedMime::SevenZip => header.starts_with(&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C]),
        AllowedMime::Tar => header.len() >= 262 && &header[257..262] == b"ustar",

        // Textual content: Check valid UTF-8 and ensure no binary NUL (\0) bytes
        AllowedMime::TextPlain
        | AllowedMime::TextCsv
        | AllowedMime::TextMarkdown
        | AllowedMime::Json => !header.contains(&0x00) && std::str::from_utf8(header).is_ok(),
    }
}

/// Helper function to parse `filename="..."` out of Content-Disposition
fn extract_filename_from_header(header_val: &str) -> Option<String> {
    // Looks for 'filename=' in headers like 'attachment; filename="cat.jpg"'
    header_val.split(';').find_map(|part| {
        let part = part.trim();
        if part.starts_with("filename=") {
            let name = part.trim_start_matches("filename=").trim_matches('"');
            Some(name.to_string())
        } else {
            None
        }
    })
}

pub struct ValidatedBody {
    pub filename: String,
    pub mime: AllowedMime,
    pub bytes: Bytes,
}

impl<S> FromRequest<S> for ValidatedBody
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 1. Fast Content-Type check (Header phase)
        let raw_mime = req
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::BAD_REQUEST)?;

        let mime = AllowedMime::parse(raw_mime).ok_or(StatusCode::UNSUPPORTED_MEDIA_TYPE)?;

        let raw_filename = req
            .headers()
            .get(header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .and_then(extract_filename_from_header)
            .unwrap_or_else(|| "file".to_string());

        let filename = sanitize_filename(&raw_filename);

        // 2. Stream body bytes (up to max payload limit, e.g., 2MB)
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        // 3. Inspect magic bytes / UTF-8
        let sample_len = bytes.len().min(512);
        if !verify_magic_bytes(&mime, &bytes[..sample_len]) {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }

        Ok(Self {
            filename,
            mime,
            bytes,
        })
    }
}

pub fn sanitize_filename(name: &str) -> String {
    let file_name = std::path::Path::new(name)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("file");

    let sanitized: String = file_name
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '.' | '-' | '_' => c,
            _ => '_',
        })
        .collect();

    // fallback if sanitization results in an empty string or leading dot hiding the file
    let trimmed = sanitized.trim_start_matches('.');
    if trimmed.is_empty() {
        "file".to_string()
    } else {
        trimmed.to_string()
    }
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct FileEntry {
    pub id: uuid::Uuid,
    pub sha256: String,
    pub mime: String,
    pub kind: String,
    pub bytes: i64,
    pub original_name: String,
    pub created_at: DateTime<Utc>,
}

pub async fn get_file_entries(
    State(state): State<Arc<AppState>>,
    _auth: AuthUser,
) -> Result<(StatusCode, Json<Vec<FileEntry>>), StatusCode> {
    let entries = sqlx::query_as::<_, FileEntry>(
        r#"
        SELECT
            id,
            sha256,
            mime,
            kind,
            bytes,
            original_name,
            created_at
        FROM files
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        println!("Error getting file entries: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, Json(entries)))
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn fast_std_hash(bytes: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:016x}", hash)
}

pub async fn create_file(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    ValidatedBody {
        filename,
        mime,
        bytes,
    }: ValidatedBody,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    println!("The user {}, created a file entry", auth.user.username);
    println!(
        "Successfully received {:?} payload of size {} bytes -> {}",
        mime,
        bytes.len(),
        filename
    );

    let hash = fast_std_hash(&bytes);
    let id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
    INSERT INTO files(
        id,
        sha256,
        mime,
        kind,
        bytes,
        original_name
    )
    VALUES ($1, $2, $3, $4, $5, $6)
    "#,
    )
    .bind(id)
    .bind(&hash)
    .bind(mime.as_str())
    .bind(mime.get_kind())
    .bind(bytes.len() as i64)
    .bind(&filename)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Errror: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let base_dir = std::path::Path::new("./media");
    let target_dir = base_dir.join(mime.get_kind()).join(hash);
    let destination_path = target_dir.join(filename);

    let file_result: Result<(), StatusCode> = async {
        tokio::fs::create_dir_all(&target_dir).await.map_err(|e| {
            println!("Failed to create target dir: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let mut file = tokio::fs::File::create(&destination_path)
            .await
            .map_err(|e| {
                println!("Failed to create file: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        file.write_all(&bytes).await.map_err(|e| {
            println!("Failed to write file bytes: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }
    .await;

    if let Err(status) = file_result {
        delete_entry_query(id, &state.pool).await?;
        return Err(status);
    }

    Ok((StatusCode::CREATED, Json(id)))
}

async fn delete_entry_query(id: uuid::Uuid, pool: &PgPool) -> Result<(), StatusCode> {
    sqlx::query(
        r#"
            DELETE FROM files 
            WHERE id = $1
            "#,
    )
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| {
        println!("Error deleting file entry: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}

#[derive(Debug, Deserialize, FromRow)]
struct FileRecordDel {
    kind: String,
    sha256: String,
}

pub async fn delete_file_entry(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    println!("The user {}, deleted a devlog entry", auth.user.username);

    let file_record = sqlx::query_as::<_, FileRecordDel>(
        r#"
        SELECT kind, sha256 
        FROM files 
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        println!("Database query error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    let target_dir = std::path::Path::new("./media")
        .join(&file_record.kind)
        .join(&file_record.sha256);

    if target_dir.exists() {
        tokio::fs::remove_dir_all(&target_dir).await.map_err(|e| {
            println!("Failed to remove directory {:?}: {}", target_dir, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    delete_entry_query(id, &state.pool).await?;

    Ok((StatusCode::NO_CONTENT, Json(id)))
}
