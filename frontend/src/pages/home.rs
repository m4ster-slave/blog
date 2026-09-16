use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
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
                <div class="home-container">
                    <div class="hero-section">
                        <h1 class="terminal-title">"Lukiana's blog"</h1>
                        <p class="hero-subtitle">"Welcome to my corner of the web"</p>
                    </div>

                    <div class="intro-section">
                        <p>"Hey there! I'm Lukiana this, is my digital space where I write about art, tech, music, and whatever else captures my interest."</p>
                    </div>
                </div>
            </section>
        </ErrorBoundary>
    }
}
