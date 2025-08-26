use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex flex-row flex-auto">
            <div class="basis-1/6 justify-self-start font-[BBold]">
                <a href="/" class="link-reset">
                    cd ~/
                </a>
            </div>

            <div class="basis-4/6 justify-self-center gap-[1vw] flex justify-center">
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

            <div class="basis-1/6 justify-self-end-safe font-[BBold]">
                <a href="../" class="link-reset">
                    cd ..
                </a>
            </div>
        </div>

    }
}
