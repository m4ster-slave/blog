use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params_map;
use reqwasm::http::Request;

use crate::models::post::Post;

fn estimate_read_time(content: &str) -> u32 {
    const WORDS_PER_MINUTE: f64 = 220.0;
    const SECONDS_PER_IMAGE: f64 = 12.0;

    let word_count = content.split_whitespace().count() as f64;
    let image_count = content.matches("img").count() as f64;

    let minutes = (word_count / WORDS_PER_MINUTE) + (image_count * SECONDS_PER_IMAGE / 60.0);
    minutes.ceil().max(1.0) as u32
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
                Some(Ok(post)) => {
                    let content = post.content.as_deref().unwrap_or("");
                    let title = post.title.clone();
                    let description = post.summary.clone();

                    view! {
                    <Title text=format!("{} | Lukiana's Blog", title) />
                    <Meta name="description" content=description.clone() />
                    <Meta property="og:title" content=title.clone() />
                    <Meta property="og:description" content=description.clone() />
                    <Meta property="og:type" content="article" />
                    <Meta name="twitter:title" content=title />
                    <Meta name="twitter:description" content=description />
                    <article class="blog-post-content">
                        <header class="blog-post-content_header">
                            <h1>{post.title}</h1>
                            <div class="blog-post-content_meta">
                                <p><b>"published: " {post.published_at.unwrap().to_string()}</b></p>
                                <p>"created: " {post.created_at.to_string()}</p>
                                <p>"updated: " {post.updated_at.to_string()}</p>
                                <p>"read time estimate: ~" {estimate_read_time(content)} " min"</p>
                                <p>"word count: " {post.word_count} </p>
                                <p>"times modified: " {post.modify_count} </p>
                            </div>
                        </header>
                        <hr></hr>
                        <div class="blog-post-content_body" inner_html={crate::utils::markdown_to_html(content)} >
                        </div>
                    </article>
                }.into_any()},

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
