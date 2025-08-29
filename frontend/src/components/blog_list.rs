use leptos::prelude::*;
use leptos_router::components::A;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Post {
    title: String,
    date: String,
    summary: String,
    slug: String,
    content: Option<String>,
}

async fn fetch_posts() -> Result<Vec<Post>, String> {
    let resp = Request::get("http://127.0.0.1:3000/posts")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json = resp.text().await.map_err(|e| e.to_string())?;
    let posts: Vec<Post> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(posts)
}

#[component]
pub fn BlogList() -> impl IntoView {
    let posts = LocalResource::new(fetch_posts);

    view! {
        <div class="blog-list">
                {move || match posts.get() {
                    Some(Ok(posts)) => view! {
                        <ul class="blog-list_items">
                            {posts.into_iter().map(|post| {
                                let slug = post.slug.clone();
                                view! {
                                    <A href={format!("/blog/{}", slug)} attr:class="blog-card-link">
                                        <li class="blog-card">
                                            <h3 class="blog-card_title">{post.title}</h3>
                                            <p class="blog-card_meta">{post.date}</p>
                                            <p class="blog-card_summary">{post.summary}</p>
                                        </li>
                                    </A>
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
