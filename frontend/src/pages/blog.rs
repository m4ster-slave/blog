use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::query_signal;

use crate::components::blog_list::BlogList;

#[component]
pub fn Blog() -> impl IntoView {
    let (count, set_count) = query_signal::<i32>("page");
    let decrement = move |_| set_count.set(Some((count.get().unwrap_or(0) - 1).max(0)));
    let increment = move |_| set_count.set(Some(count.get().unwrap_or(0) + 1));

    view! {
        <Title text="Blog Posts | Lukiana's Blog" />
        <Meta name="description" content="Read Lukiana's latest writing about technology, art, music, software, and life." />
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
                <BlogList/>
                <div class = "bloglist-nav">
                    <button class="arrow" on:click=decrement>"<-"</button>
                    <button class="arrow" on:click=increment>"->"</button>
                </div>
            </section>
        </ErrorBoundary>
    }
}
