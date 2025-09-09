use axum::{Json, http::StatusCode};
use rand::rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub async fn get_photos() -> (StatusCode, Json<Vec<Photo>>) {
    match read_photo_files("../album") {
        Ok(p) => (StatusCode::OK, Json(p)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Photo {
    path: String,
}

fn read_photo_files<P: AsRef<Path>>(dir_path: P) -> Result<Vec<Photo>, Box<dyn std::error::Error>> {
    let dir_path = dir_path.as_ref();

    let mut entries: Vec<PathBuf> = fs::read_dir(dir_path)?
        .filter_map(|res| res.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path())
        .collect();

    let mut rng = rng();
    entries.shuffle(&mut rng);

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
