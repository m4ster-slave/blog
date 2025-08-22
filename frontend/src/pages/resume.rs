use leptos::prelude::*;

#[component]
pub fn Resume() -> impl IntoView {
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
            <h1>Resume</h1>
            <iframe src="/public/resume.pdf" width="100%" height="1000px"></iframe>
            </div>
        </ErrorBoundary>
    }
}
