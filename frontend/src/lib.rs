use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod models;
mod pages;
mod utils;

use crate::components::blog_post::BlogPost;
use crate::components::navbar::Navbar;
use crate::pages::about::About;
use crate::pages::admin_panel::AdminPanel;
use crate::pages::blog::Blog;
use crate::pages::create_post::CreatePost;
use crate::pages::devlog::Devlog;
use crate::pages::edit_post::EditPost;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::not_found::NotFound;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark" />
        <Title text="Blog" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Navbar />

        <main class="main-content">
            <Router >
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/about") view=About/>
                    <Route path=path!("/blog") view=Blog />
                    <Route path=path!("/blog/:slug") view=BlogPost />
                    <Route path=path!("/devlog") view=Devlog/>
                    <Route path=path!("/admin/login") view=Login/>
                    <Route path=path!("/admin") view=AdminPanel/>
                    <Route path=path!("/admin/create") view=CreatePost/>
                    <Route path=path!("/admin/edit") view=EditPost/>
                </Routes>
            </Router>
        </main>
    }
}
