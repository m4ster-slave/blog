use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos::web_sys::SubmitEvent;
use leptos_router::hooks::query_signal;
use reqwasm::http::Request;
use serde::Serialize;

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
pub fn Entry(entry: DevlogEntry) -> impl IntoView {
    view! {
            <li class="devlog-card">
                <div class="devlog-card_header">
                    <p class="devlog-card_date">{format!("{}", entry.created_at)}</p>
                </div>
                <div class="devlog-card_body" inner_html={crate::utils::markdown_to_html(&entry.content)}>
                </div>
            </li>
    }
}

async fn delete_entry(id: String) -> Result<String, String> {
    let url = format!("/api/devlog/entries/{}", id);
    let resp = Request::delete(&url)
        .header("Content-Type", "application/json")
        .header("X-CSRF-Token", &utils::csrf_token())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 204 {
        return Err(format!("server returned status {}", resp.status()));
    }

    let uuid = resp.text().await.map_err(|e| e.to_string())?;
    Ok(uuid)
}

#[derive(Serialize, Clone, Debug)]
struct UpdateEntryPayload {
    content: String,
}

async fn update_entry(id: String, payload: UpdateEntryPayload) -> Result<String, String> {
    let url = format!("/api/devlog/entries/{}", id);
    let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let resp = Request::put(&url)
        .header("Content-Type", "application/json")
        .header("X-CSRF-Token", &utils::csrf_token())
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

#[component]
pub fn AdminEntry(entry: DevlogEntry, on_delete: Callback<()>) -> impl IntoView {
    let editing = RwSignal::new(false);
    let content = RwSignal::new(entry.content.clone());
    let save_error = RwSignal::new(None::<String>);

    let entry_id = entry.id.clone();

    let delete_entry_fn = StoredValue::new({
        let entry_id = entry_id.clone();
        move |_| {
            if !utils::confirm("Delete this devlog entry? This cannot be undone.") {
                return;
            }

            let id = entry_id.clone();
            leptos::task::spawn_local(async move {
                match delete_entry(id).await {
                    Ok(_) => {
                        editing.set(false);
                        on_delete.run(());
                        save_error.set(None)
                    }
                    Err(e) => save_error.set(Some(e)),
                }
            });
        }
    });

    let submit_entry_fn = StoredValue::new({
        let entry_id = entry_id.clone();
        move |_| {
            let id = entry_id.clone();
            let payload = UpdateEntryPayload {
                content: content.get(),
            };

            leptos::task::spawn_local(async move {
                match update_entry(id, payload).await {
                    Ok(_) => {
                        editing.set(false);
                        save_error.set(None)
                    }
                    Err(e) => save_error.set(Some(e)),
                }
            });
        }
    });

    view! {
        <li class="devlog-card">
            <div class="devlog-card_header">
                <p class="devlog-card_date">
                    {format!("{}", entry.created_at)}
                </p>

                <button class="devlog-card_edit-button"
                    on:click=move |_| editing.update(|e| *e = !*e)
                >
                    {move || if editing.get() {"End edit"} else {"Edit"}}
                </button>
            </div>

            { move || if editing.get() {
                view! {
                    <label>
                        "Content"
                        <textarea
                            class="devlog-editor_content"
                            prop:value=content
                            on:input=move |ev| content.set(event_target_value(&ev))
                        ></textarea>
                    </label>
                    <div class="devlog-editor_actions">
                        <button on:click=move |ev| submit_entry_fn.with_value(|f| f(ev))>"Save"</button>
                        <button on:click=move |ev| delete_entry_fn.with_value(|f| f(ev))>"Delete"</button>
                    </div>
                    {move || save_error.get().map(|e| view! {
                        <div class="error-message">
                            <p>{format!("Failed to save: {}", e)}</p>
                        </div>
                    })}
                }.into_any()
            } else {
                view! {
                    <div class="devlog-card_body" inner_html={crate::utils::markdown_to_html(&content.get())}>
                    </div>
                }.into_any()
            }}
        </li>
    }
}

#[component]
pub fn Devlog() -> impl IntoView {
    let (page, set_page) = query_signal::<i32>("page");
    let decrement = move |_| set_page.set(Some((page.get().unwrap_or(0) - 1).max(0)));
    let increment = move |_| set_page.set(Some(page.get().unwrap_or(0) + 1));

    let refresh_trigger = RwSignal::new(0u32);
    let entries = LocalResource::new(move || {
        refresh_trigger.get();
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
                    .header("X-CSRF-Token", &utils::csrf_token())
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
        <Title text="Development Log | Lukiana's Blog" />
        <Meta name="description" content="Development notes and progress updates from Lukiana's projects." />
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
                        <form class="devlog-form" on:submit=submit>
                            <label>
                                <textarea prop:value=content on:input=move |ev| { content.set(event_target_value(&ev));}></textarea>
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
                    _ => ().into_any()
                }}


                {move || match entries.get() {
                    Some(Ok(entries)) => view! {
                        <div class = "devlog-entries">
                        <ul class="devlog-entries_items">
                            {entries.into_iter().map(|entry| {
                                match auth_status.get() {
                                    Some(Ok(true)) => view! {
                                        <AdminEntry
                                            entry = entry
                                            on_delete=Callback::new(move |_| {
                                                refresh_trigger.update(|n| *n += 1);
                                            })
                                        />
                                    }.into_any(),
                                    _ => view! {
                                        <Entry
                                            entry = entry
                                        />
                                    }.into_any()
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                        </div>
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

                <div class = "bloglist-nav">
                    <button class="arrow" on:click=decrement>"<-"</button>
                    <button class="arrow" on:click=increment>"->"</button>
                </div>

            </section>
        </ErrorBoundary>
    }
}
