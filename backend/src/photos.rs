use axum::extract::State;
use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::AppState;

pub async fn get_photos(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Vec<Photo>>) {
    (StatusCode::OK, Json(state.photos.clone()))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Photo {
    path: String,
}

pub fn read_photo_files<P: AsRef<Path>>(
    dir_path: P,
) -> Result<Vec<Photo>, Box<dyn std::error::Error>> {
    let dir_path = dir_path.as_ref();

    let entries: Vec<PathBuf> = fs::read_dir(dir_path)?
        .filter_map(|res| res.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path())
        .collect();

    let photos = entries
        .into_iter()
        .filter_map(|path| {
            path.file_name().map(|fname| Photo {
                path: format!("/album/{}", fname.to_string_lossy()),
            })
        })
        .collect();

    Ok(photos)
}
