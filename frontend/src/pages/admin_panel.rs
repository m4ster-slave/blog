use leptos::prelude::*;

use crate::utils;

#[component]
pub fn AdminPanel() -> impl IntoView {
    let auth_status = LocalResource::new(utils::is_admin);
    view! {
        <Suspense fallback=move || view! { <p>"Checking authorization..."</p> }>
            {move || match auth_status.get() {
                Some(Ok(true)) => view! {
                    <section class="page-section">
                        <h1>"Admin panel"</h1>
                        <a href="/admin/edit">Edit a post</a>
                        <br></br>
                        <a href="/admin/create">Create a new post</a>
                    </section>
                }.into_any(),
                _ => view! { <p>"Access denied. Please log in."</p> }.into_any()
            }}
        </Suspense>
    }
}
