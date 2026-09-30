use leptos::prelude::*;
use reqwasm::http::Request;
use serde::Serialize;
use uuid::Uuid;

use crate::models::post::{Post, PostSummary};
use crate::utils;

async fn fetch_admin_posts() -> Result<Vec<PostSummary>, String> {
    let resp = Request::get("/api/admin/posts")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("server returned status {}", resp.status()));
    }
    let json = resp.text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&json).map_err(|e| e.to_string())
}

async fn fetch_admin_post(id: Uuid) -> Result<Post, String> {
    let url = format!("/api/admin/posts/{}", id);
    let resp = Request::get(&url).send().await.map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("server returned status {}", resp.status()));
    }
    let json = resp.text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&json).map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct UpdatePostPayload {
    title: String,
    slug: String,
    summary: String,
    archived: bool,
    content: Option<String>,
}

async fn update_post(id: Uuid, payload: UpdatePostPayload) -> Result<String, String> {
    let url = format!("/api/admin/posts/{}", id);
    let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let resp = Request::put(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("server returned status {}", resp.status()));
    }
    let uuid = resp.text().await.map_err(|e| e.to_string())?;
    Ok(uuid)
}

async fn delete_post(id: Uuid) -> Result<String, String> {
    let url = format!("/api/admin/posts/{}", id);
    let resp = Request::delete(&url)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 204 {
        return Err(format!("server returned status {}", resp.status()));
    }

    let uuid = resp.text().await.map_err(|e| e.to_string())?;
    Ok(uuid)
}

async fn publish_post(id: Uuid) -> Result<(), String> {
    let url = format!("/api/admin/posts/{}/publish", id);
    let resp = Request::post(&url)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(format!("server returned status {}", resp.status()));
    }

    Ok(())
}

#[component]
pub fn EditPost() -> impl IntoView {
    let selected_id = RwSignal::new(None::<Uuid>);
    let refresh_trigger = RwSignal::new(0u32);

    let posts = LocalResource::new(move || {
        refresh_trigger.get();
        fetch_admin_posts()
    });

    view! {
        <section class="page-section">
            <Show
                when=move || selected_id.get().is_some()
                fallback=move || view! {
                    <div class="admin-post-list">
                        <h2>"Edit Posts"</h2>
                        {move || match posts.get() {
                            Some(Ok(posts)) => view! {
                                <ul class="blog-list_items">
                                    {posts.into_iter().map(|post| {
                                        let id = post.id;
                                        let is_archived = post.archived;
                                        view! {
                                            <li
                                                class="blog-card admin-post-card"
                                                on:click=move |_| selected_id.set(Some(id))
                                            >
                                                <h3 class="blog-card_title">{post.title.clone()}</h3>
                                                <p class="blog-card_meta">
                                                    {format!("{}", post.created_at)}
                                                    {if is_archived { " · unpublished" } else { "" }}
                                                </p>
                                                <p class="blog-card_summary">{post.summary.clone()}</p>
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
            >
                {move || {
                    let id = selected_id.get().unwrap();
                    view! {
                        <PostEditor
                            id=id
                            on_close=Callback::new(move |_| selected_id.set(None))
                            on_saved=Callback::new(move |_| {
                                selected_id.set(None);
                                refresh_trigger.update(|n| *n += 1);
                            })
                        />
                    }
                }}
            </Show>
        </section>
    }
}

#[component]
fn PostEditor(id: Uuid, on_close: Callback<()>, on_saved: Callback<()>) -> impl IntoView {
    let post_resource = LocalResource::new(move || fetch_admin_post(id));

    let title = RwSignal::new(String::new());
    let slug = RwSignal::new(String::new());
    let summary = RwSignal::new(String::new());
    let content = RwSignal::new(String::new());
    let archived = RwSignal::new(false);
    let was_archived = RwSignal::new(false);
    let loaded = RwSignal::new(false);
    let saving = RwSignal::new(false);
    let save_error = RwSignal::new(None::<String>);

    Effect::new(move |_| {
        if let Some(Ok(post)) = post_resource.get() {
            if !loaded.get() {
                title.set(post.title.clone());
                slug.set(post.slug.clone());
                summary.set(post.summary.clone());
                content.set(post.content.clone().unwrap_or_default());
                archived.set(post.archived);
                was_archived.set(post.archived);
                loaded.set(true);
            }
        }
    });

    let submit = move |_| {
        if archived.get() != was_archived.get() {
            let action = if archived.get() { "Archive" } else { "Unarchive" };
            let visibility = if archived.get() {
                "It will no longer be publicly visible."
            } else {
                "It may become publicly visible."
            };
            if !utils::confirm(&format!(
                "{action} '{}'? {visibility}",
                title.get()
            )) {
                return;
            }
        }

        let payload = UpdatePostPayload {
            title: title.get(),
            slug: slug.get(),
            summary: summary.get(),
            content: {
                let c = content.get();
                if c.trim().is_empty() {
                    None
                } else {
                    Some(c)
                }
            },
            archived: archived.get(),
        };
        saving.set(true);
        save_error.set(None);
        leptos::task::spawn_local(async move {
            match update_post(id, payload).await {
                Ok(_) => on_saved.run(()),
                Err(e) => save_error.set(Some(e)),
            }
            saving.set(false);
        });
    };

    let delete = move |_| {
        if !utils::confirm(&format!(
            "Delete '{}'? This cannot be undone.",
            title.get()
        )) {
            return;
        }

        saving.set(true);
        save_error.set(None);
        leptos::task::spawn_local(async move {
            match delete_post(id).await {
                Ok(_) => on_saved.run(()),
                Err(e) => save_error.set(Some(e)),
            }
            saving.set(false);
        });
    };

    let publish = move |_| {
        if !utils::confirm(&format!(
            "Publish '{}'? It will become publicly visible.",
            title.get()
        )) {
            return;
        }

        saving.set(true);
        save_error.set(None);
        leptos::task::spawn_local(async move {
            match publish_post(id).await {
                Ok(_) => on_saved.run(()),
                Err(e) => save_error.set(Some(e)),
            }
            saving.set(false);
        });
    };

    view! {
        <div class="admin-post-editor">
            {move || match post_resource.get() {
                Some(Ok(_)) => view! {
                    <form class="post-form" on:submit=submit>
                        <label>
                            "title"
                            <input
                                type="text"
                                prop:value=move || title.get()
                                on:input=move |ev| title.set(event_target_value(&ev))
                            />
                        </label>
                        <label>
                            "slug"
                            <input
                                type="text"
                                prop:value=move || slug.get()
                                on:input=move |ev| slug.set(event_target_value(&ev))
                            />
                        </label>
                        <label>
                            "summary"
                            <textarea
                                prop:value=move || summary.get()
                                on:input=move |ev| summary.set(event_target_value(&ev))
                            ></textarea>
                        </label>
                        <label>
                            "content"
                            <textarea
                                class="admin-post-editor_content"
                                prop:value=move || content.get()
                                on:input=move |ev| content.set(event_target_value(&ev))
                            ></textarea>
                        </label>
                        <label class="admin-post-editor_archived">
                            <input
                                type="checkbox"
                                prop:checked=move || archived.get()
                                on:change=move |ev| archived.set(event_target_checked(&ev))
                            />
                            "archived"
                        </label>

                        {move || save_error.get().map(|e| view! {
                            <div class="error-message"><p>{format!("Failed to save: {}", e)}</p></div>
                        })}

                        <div class="admin-post-editor_actions">
                            <button type="submit" disabled=move || saving.get()>
                                {move || if saving.get() { "Saving..." } else { "Save" }}
                            </button>
                            <button type="button" on:click=move |_| on_close.run(())>"Cancel"</button>
                            <button type="button" on:click=delete >"Delete"</button>
                            <button type="button" on:click=publish >"Publish"</button>
                        </div>
                    </form>
                }.into_any(),
                Some(Err(e)) => view! {
                    <div class="error-message">
                        <p>{format!("Failed to load post: {}", e)}</p>
                        <button on:click=move |_| on_close.run(())>"Back"</button>
                    </div>
                }.into_any(),
                None => view! {
                    <div class="loading-message"><p>"Loading post..."</p></div>
                }.into_any(),
            }}
        </div>
    }
}
