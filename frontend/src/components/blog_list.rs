use leptos::prelude::*;
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

async fn fetch_single_posts(slug: &str) -> Result<Vec<Post>, String> {
    let resp = Request::get(&format!("http://127.0.0.1:3000/posts?p={}", slug))
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
        <div class="p-4 space-y-4">
            <h2 class="text-2xl font-bold">"Posts"</h2>
            <div>
                {move || match posts.get() {
                    Some(Ok(posts)) => view! {
                        <div>
                            <ul class="space-y-2">
                                {posts.into_iter().map(|post| view! {
                                    <li class="border p-2 rounded">
                                        <h3 class="font-semibold">{post.title}</h3>
                                        <p class="text-sm text-gray-600">{post.date}</p>
                                        <p class="mt-1">{post.summary}</p>
                                    </li>
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }.into_any(),
                    Some(Err(e)) => view! {
                        <div>
                            <p class="text-red-600">{format!("Failed to fetch posts: {}", e)}</p>
                        </div>
                    }.into_any(),
                    None => view! {
                        <div>
                            <p>"Loading posts..."</p>
                        </div>
                    }.into_any(),
                }}
            </div>
        </div>
    }
}
