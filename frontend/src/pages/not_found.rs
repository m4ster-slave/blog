use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="Page Not Found | Lukiana's Blog" />
        <h1>"Uh oh :/" <br /> "We couldn't find that page!"</h1>
    }
}
