use chrono::{DateTime, Utc};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use reqwasm::http::Request;

use crate::models::post::Post;

fn format_date(date_str: &str) -> String {
    match DateTime::parse_from_rfc3339(date_str) {
        Ok(datetime) => {
            let utc_datetime: DateTime<Utc> = datetime.with_timezone(&Utc);
            utc_datetime.format("%B %d, %Y").to_string()
        }
        Err(_) => date_str.to_string(), // Fallback to original string if parsing fails
    }
}

async fn fetch_post_by_slug(slug: String) -> Result<Post, String> {
    let resp = Request::get(&format!("/api/posts/{}", slug))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json = resp.text().await.map_err(|e| e.to_string())?;
    let post: Post = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    Ok(post)
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    let post_resource = LocalResource::new({
        let slug = slug();
        move || {
            let slug = slug.clone();
            fetch_post_by_slug(slug)
        }
    });

    view! {
        <div class="blog-post">
            {move || match post_resource.get() {
                Some(Ok(post)) => view! {
                    <article class="blog-post-content">
                        <header class="blog-post-content_header">
                            <h1 class="blog-post_title">{post.title}</h1>
                            <p class="blog-post-content_meta">{format_date(&post.created_at.to_rfc3339())}</p>
                        </header>
                        <div class="blog-post-content_body" inner_html={crate::utils::markdown_to_html(&post.content.unwrap())}>
                        </div>
                    </article>
                }.into_any(),

                Some(Err(e)) => view! {
                    <div class="error-message">
                        <h1>"Post Not Found"</h1>
                        <p>{format!("Error: {}", e)}</p>
                    </div>
                }.into_any(),

                None => view! {
                    <div class="loading-message">
                        <p>"Loading post..."</p>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}
