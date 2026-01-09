use leptos::prelude::*;
use rand::seq::SliceRandom;
use reqwasm::http::Request;

async fn fetch_photos() -> Result<Vec<String>, String> {
    let mut rng = rand::rng();
    let resp = Request::get("/api/photos")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json = resp.text().await.map_err(|e| e.to_string())?;
    let mut posts: Vec<String> = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    posts.shuffle(&mut rng);

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

    <img src={photo} alt="photo" />
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
