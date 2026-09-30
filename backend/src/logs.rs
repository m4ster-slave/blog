use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::{collections::VecDeque, path::{Path, PathBuf}};

use crate::{AppState, auth::AuthUser};

#[derive(Debug, Deserialize)]
pub struct LogQuery {
    lines: Option<usize>,
}

pub async fn get_logs(
    State(state): State<std::sync::Arc<AppState>>,
    _auth: AuthUser,
    Query(query): Query<LogQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let line_limit = query.lines.unwrap_or(500).clamp(1, 5000);
    let path = latest_log_path(&state.log_dir).await?;
    let lines = read_last_lines(&path, line_limit).await?;

    Ok((
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        lines,
    ))
}

async fn latest_log_path(log_dir: &Path) -> Result<PathBuf, StatusCode> {
    let mut entries = tokio::fs::read_dir(log_dir)
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "failed to read log directory");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let mut latest: Option<(std::time::SystemTime, PathBuf)> = None;

    while let Some(entry) = entries.next_entry().await.map_err(|error| {
        tracing::error!(error = %error, "failed to enumerate log directory");
        StatusCode::INTERNAL_SERVER_ERROR
    })? {
        let path = entry.path();
        let is_log = path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with("backend.") && !name.ends_with(".gz"))
            .unwrap_or(false);
        if !is_log {
            continue;
        }

        let modified = entry
            .metadata()
            .await
            .and_then(|metadata| metadata.modified())
            .map_err(|error| {
                tracing::error!(error = %error, "failed to inspect log file");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        if latest.as_ref().map(|(time, _)| modified > *time).unwrap_or(true) {
            latest = Some((modified, path));
        }
    }

    latest.map(|(_, path)| path).ok_or_else(|| {
        tracing::warn!("no log file found");
        StatusCode::NOT_FOUND
    })
}

async fn read_last_lines(path: &Path, line_limit: usize) -> Result<String, StatusCode> {
    use tokio::io::{AsyncBufReadExt, BufReader};

    let file = tokio::fs::File::open(path).await.map_err(|error| {
        tracing::error!(error = %error, "failed to open log file");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let mut reader = BufReader::new(file).lines();
    let mut lines = VecDeque::with_capacity(line_limit);

    while let Some(line) = reader.next_line().await.map_err(|error| {
        tracing::error!(error = %error, "failed to read log file");
        StatusCode::INTERNAL_SERVER_ERROR
    })? {
        if lines.len() == line_limit {
            lines.pop_front();
        }
        lines.push_back(line);
    }

    Ok(lines.into_iter().collect::<Vec<_>>().join("\n") + "\n")
}