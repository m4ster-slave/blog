use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar_back">
                <a href="/" class="link-reset">
                    cd ~/
                </a>
            </div>

            <div class="navbar_nav">
                <a href="/" class="link-reset">
                   home
                </a>
                <a href="/about" class="link-reset">
                    about
                </a>
                <a href="/blog" class="link-reset">
                    blog
                </a>
            </div>

            <div class="navbar_back">
                <a href="../" class="link-reset">
                    cd ..
                </a>
            </div>
        </nav>
    }
}
