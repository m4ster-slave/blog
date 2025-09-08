use chrono::{DateTime, Utc};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::query_signal;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

fn format_date(date_str: &str) -> String {
    match DateTime::parse_from_rfc3339(date_str) {
        Ok(datetime) => {
            let utc_datetime: DateTime<Utc> = datetime.with_timezone(&Utc);
            utc_datetime.format("%B %d, %Y").to_string()
        }
        Err(_) => date_str.to_string(), // Fallback to original string if parsing fails
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Post {
    title: String,
    date: String,
    summary: String,
    slug: String,
    content: Option<String>,
}

async fn fetch_posts(page: i32) -> Result<Vec<Post>, String> {
    let url = format!("/api/posts?page={}", page);
    let resp = Request::get(&url).send().await.map_err(|e| e.to_string())?;
    let json = resp.text().await.map_err(|e| e.to_string())?;
    let posts: Vec<Post> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(posts)
}

#[component]
pub fn BlogList() -> impl IntoView {
    let (page, _set_page) = query_signal::<i32>("page");
    let posts = LocalResource::new(move || {
        let current_page = page.get().unwrap_or(0);
        fetch_posts(current_page)
    });

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
                                            <p class="blog-card_meta">{format_date(&post.date)}</p>
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
