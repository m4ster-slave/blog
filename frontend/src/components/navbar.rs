use leptos::prelude::*;

use crate::utils;

#[component]
pub fn Navbar() -> impl IntoView {
    let auth_status = LocalResource::new(utils::is_admin);
    view! {
        <nav class="navbar">
            <div class="navbar_back">
                <a href="/" class="link-reset">
                    cd ~
                </a>
            </div>

            <div class="navbar_nav">
                <a href="/about" class="link-reset">
                    about
                </a>
                <a href="/blog" class="link-reset">
                    blog
                </a>
                <a href="/devlog" class="link-reset">
                    devlog
                </a>
                {move || match auth_status.get() {
                Some(Ok(true)) => view! {
                    <a href="/admin" class="link-reset">
                        admin
                    </a>
                }.into_any(),
                _ => ().into_any()
            }}
            </div>

            <div class="navbar_back">
                <img src="./logo.svg" alt="TRANS" />
            </div>
        </nav>
    }
}
