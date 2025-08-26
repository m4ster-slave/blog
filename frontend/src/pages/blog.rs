use leptos::prelude::*;

use crate::components::blog_list::BlogList;

#[component]
pub fn Blog() -> impl IntoView {
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

            <div class="container">
                <BlogList/>
            </div>
        </ErrorBoundary>
    }
}
