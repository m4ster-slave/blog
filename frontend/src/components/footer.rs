use chrono::Datelike;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <nav class="footer">
            <div class="footer_copyright">
                <p>
                    Copyright (c) {format!(" {} ", chrono::Utc::now().year())} Lukiana Weger.
                </p>
            </div>

            <div class="footer_statements">
                <a href="/privacy" class="link-reset">
                    <span>privacy</span>
                </a>
                <a href="/tos" class="link-reset">
                    <span>tos</span>
                </a>
            </div>

            <div class="footer_socials">
                <a href="https://github.com/m4ster-slave">
                    github
                </a>
                <a href="https://weger.dev">
                    resume
                </a>
            </div>
        </nav>
    }
}
