use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;
use leptos_router::hooks::use_navigate;
use reqwasm::http::Request;

use crate::models::post::*;

#[component]
pub fn CreatePost() -> impl IntoView {
    let navigate = use_navigate();

    let title = RwSignal::new(String::new());
    let slug = RwSignal::new(String::new());
    let summary = RwSignal::new(String::new());
    let content = RwSignal::new(String::new());

    let error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(false);

    let submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let title = title.get_untracked();
        let slug = slug.get_untracked();
        let summary = summary.get_untracked();
        let content = content.get_untracked();

        loading.set(true);
        error.set(None);

        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            let body = CreatePostRequest {
                title,
                slug,
                summary,
                content,
            };

            let result = async {
                let body = serde_json::to_string(&body).map_err(|e| e.to_string())?;

                let response = Request::post("/api/admin/posts")
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

            loading.set(false);

            match result {
                Ok(()) => {
                    navigate("/admin/edit", Default::default());
                }
                Err(message) => {
                    error.set(Some(message));
                }
            }
        });
    };

    view! {
        <section class="page-section">
            <form class="post-form" on:submit=submit>
                <label>
                    "title"
                    <input
                        type="text"
                        prop:value=title
                        on:input=move |ev| {
                            title.set(event_target_value(&ev));
                        }
                    />
                </label>

                <label>
                    "slug"
                    <input
                        type="text"
                        prop:value=slug
                        on:input=move |ev| {
                            slug.set(event_target_value(&ev));
                        }
                    />
                </label>
                <label>
                    "summary"
                    <textarea
                        prop:value=summary
                        on:input=move |ev| {
                            summary.set(event_target_value(&ev));
                        }
                    ></textarea>
                </label>
                <label>
                    "content"
                    <textarea
                        prop:value=content
                        on:input=move |ev| content.set(event_target_value(&ev))
                    ></textarea>
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
        </section>
    }
}
