use leptos::prelude::*;

use crate::utils;

#[component]
pub fn AdminPanel() -> impl IntoView {
    let auth_status = LocalResource::new(utils::is_admin);
    view! {
        <Suspense fallback=move || view! { <p>"Checking authorization..."</p> }>
            {move || match auth_status.get() {
                Some(Ok(true)) => view! {
                    <section class="page-section admin-panel">
                        <h1>"Admin panel"</h1>
                        <div class="admin-panel_links">
                            <a href="/admin/edit">edit post</a>
                            <a href="/admin/create">new post</a>
                            <div class="admin-panel_links-sub">
                                <a href="/admin/files">view files</a>
                                <a href="/api/admin/logs" rel="external">view logs</a>
                            </div>
                        </div>
                    </section>
                }.into_any(),
                _ => view! { <p>"Access denied. Please log in."</p> }.into_any()
            }}
        </Suspense>
    }
}
