use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos::wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Response};

const GIT_HASH: &str = env!("GIT_HASH");

async fn get_wasm_size() -> Option<u64> {
    let win = window()?;
    let resp: Response = JsFuture::from(win.fetch_with_str("/blog_bg.wasm"))
        .await
        .ok()?
        .dyn_into()
        .ok()?;
    resp.headers()
        .get("content-length")
        .ok()
        .flatten()
        .and_then(|s| s.parse().ok())
}

#[component]
pub fn Stats() -> impl IntoView {
    let wasm_size = LocalResource::new(get_wasm_size);

    view! {
        <Title text="Site Stats | Lukiana's Blog" />
        <Meta name="description" content="View technical and usage statistics for Lukiana's blog." />
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
                <p>"current commit hash: " {GIT_HASH}</p>
                <hr></hr>
                <p>"bundle size: "
                    {move || {
                        wasm_size
                            .get()
                            .map(|opt| match opt {
                                Some(bytes) => format!("{} bytes", bytes),
                                None => "unavailable".to_string(),
                            })
                            .unwrap_or_else(|| "measuring…".to_string())
                    }}
                </p>

            </section>
        </ErrorBoundary>
    }
}
