use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;
use leptos_router::hooks::query_signal;
use reqwasm::http::Request;

use crate::models::devlog_entry::*;
use crate::utils;

async fn fetch_entries(page: i32) -> Result<Vec<DevlogEntry>, String> {
    let url = format!("/api/devlog/entries?page={}", page);
    let resp = Request::get(&url).send().await.map_err(|e| e.to_string())?;
    let json = resp.text().await.map_err(|e| e.to_string())?;
    let entries: Vec<DevlogEntry> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(entries)
}

#[component]
pub fn Devlog() -> impl IntoView {
    let (page, _set_page) = query_signal::<i32>("page");
    let entries = LocalResource::new(move || {
        let current_page = page.get().unwrap_or(0);
        fetch_entries(current_page)
    });

    let auth_status = LocalResource::new(utils::is_admin);

    let content = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(false);
    let submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let content = content.get_untracked();

        loading.set(true);
        error.set(None);

        leptos::task::spawn_local(async move {
            let body = CreateDevlogEntryRequest { content };

            let result = async {
                let body = serde_json::to_string(&body).map_err(|e| e.to_string())?;

                let response = Request::post("/api/devlog/entries")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;

                match response.status() {
                    201 => Ok(()),
                    _ => Err("Creation failed".to_string()),
                }
            }
            .await;

            entries.refetch();
            loading.set(false);

            match result {
                Ok(()) => {}
                Err(message) => {
                    error.set(Some(message));
                }
            }
        });
    };

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>

            <section class="page-section">
                {move || match auth_status.get() {
                    Some(Ok(true)) => view! {
                        <form on:submit=submit>
                            <label>
                                "content"
                                <input
                                    type="content"
                                    prop:value=content
                                    on:input=move |ev| {
                                        content.set(event_target_value(&ev));
                                    }
                                />
                            </label>
                            <button
                                type="submit"
                                disabled=move || loading.get()
                            >
                                {move || if loading.get() {
                                    "Creating post..."
                                } else {
                                    "Submit"
                                }}
                            </button>

                            {move || error.get().map(|error| view! {
                                <p class="error">{error}</p>
                            })}
                        </form>

                    }.into_any(),
                    _ => view! { <p>"Access denied. Please log in."</p> }.into_any()
                }}

                <hr></hr>

                {move || match entries.get() {
                    Some(Ok(entries)) => view! {
                        <ul class="blog-list_items">
                            {entries.into_iter().map(|entry| {
                                view! {
                                        <li class="blog-card">
                                            <p class="blog-card_meta">{format!("{}", entry.created_at)}</p>
                                            <p class="blog-card_summary">{entry.content}</p>
                                        </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_any(),

                    Some(Err(e)) => view! {
                        <div class="error-message">
                            <p>{format!("Failed to fetch entries: {}", e)}</p>
                        </div>
                    }.into_any(),

                    None => view! {
                        <div class="loading-message">
                            <p>"Loading entries..."</p>
                        </div>
                    }.into_any(),
                }}

            </section>
        </ErrorBoundary>
    }
}
