use crate::auth::AuthUser;
use crate::models::devlog::Devlog;
use axum::extract::{Path, State};
use axum::{Json, http::StatusCode};
use axum::{
    body::Bytes,
    extract::FromRequest,
    http::{Request, header},
};
use std::sync::Arc;

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

pub struct ValidatedBody {
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

        // 2. Stream body bytes (up to max payload limit, e.g., 2MB)
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        // 3. Inspect magic bytes / UTF-8
        let sample_len = bytes.len().min(512);
        if !verify_magic_bytes(&mime, &bytes[..sample_len]) {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }

        Ok(Self { mime, bytes })
    }
}

pub async fn get_file_entries(
    State(state): State<Arc<AppState>>,
    _auth: AuthUser,
) -> Result<(StatusCode, Json<Vec<Devlog>>), StatusCode> {
    let entries = sqlx::query_as::<_, Devlog>(
        r#"
        SELECT
            id,
            key,
            bytes,
            original_name,
            created_at
        FROM files
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(entries)))
}

pub async fn create_file(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    ValidatedBody { mime, bytes }: ValidatedBody,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    println!("The user {}, created a file entry", auth.user.username);
    println!(
        "Successfully received {:?} payload of size {} bytes",
        mime,
        bytes.len()
    );

    let id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
    INSERT INTO files(
        id
    )
    VALUES ($1)
    "#,
    )
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Errror: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(id)))
}

pub async fn delete_file_entry(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<uuid::Uuid>), StatusCode> {
    println!("The user {}, deleted a devlog entry", auth.user.username);

    sqlx::query(
        r#"
            DELETE FROM files 
            WHERE id = $1
            "#,
    )
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        println!("Error deleting file entry: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // also delete file on disk here!!

    unimplemented!();
    Ok((StatusCode::NO_CONTENT, Json(id)))
}
