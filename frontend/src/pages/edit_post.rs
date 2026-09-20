use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;
use leptos_router::hooks::use_navigate;
use reqwasm::http::Request;
use serde::Serialize;

#[derive(Serialize)]
struct CreatePostRequest {
    title: String,
    slug: String,
    summary: String,
    content: String,
}

#[component]
pub fn EditPost() -> impl IntoView {
    view! {
        <section class="page-section">
            Edit Post Section
        </section>
    }
}
