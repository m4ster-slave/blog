use leptos::prelude::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Photo {
    path: String,
}

async fn fetch_photos() -> Result<Vec<Photo>, String> {
    let resp = Request::get("/api/photos")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json = resp.text().await.map_err(|e| e.to_string())?;
    let posts: Vec<Photo> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(posts)
}

#[component]
pub fn Photos() -> impl IntoView {
    let photos = LocalResource::new(fetch_photos);

    view! {
            <div class="blog-list">
                    {move || match photos.get() {
                        Some(Ok(photos)) => view! {
                            <ul class="photo-grid">
                                {photos.into_iter().map(|photo| {
                                    view! {
                                            <li >

    <img src={photo.path} alt="photo" />
                                            </li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                        }.into_any(),

                        Some(Err(e)) => view! {
                            <div class="error-message">
                                <p>{format!("Failed to fetch posts: {}", e)}</p>
                            </div>
                        }.into_any(),

                        None => view! {
                            <div class="loading-message">
                                <p>"Loading posts..."</p>
                            </div>
                        }.into_any(),
                    }}
            </div>

        }
}
