use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;
use leptos_router::hooks::use_navigate;
use reqwasm::http::Request;

use crate::models::login::LoginRequest;

#[component]
pub fn Login() -> impl IntoView {
    let navigate = use_navigate();

    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(false);

    let submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username = username.get_untracked();
        let password = password.get_untracked();

        loading.set(true);
        error.set(None);

        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            let body = LoginRequest { username, password };

            let result = async {
                let body = serde_json::to_string(&body).map_err(|e| e.to_string())?;

                let response = Request::post("/api/admin/login")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;

                match response.status() {
                    200 => Ok(()),
                    _ => Err("Login failed".to_string()),
                }
            }
            .await;

            loading.set(false);

            match result {
                Ok(()) => {
                    navigate("/admin", Default::default());
                }
                Err(message) => {
                    error.set(Some(message));
                }
            }
        });
    };

    view! {
        <section class="page-section">
            <div class="auth-card">
            <form class = "auth-card_form" on:submit=submit>
                <label>
                    <span>"username"</span>
                    <input
                        type="text"
                        prop:value=username
                        on:input=move |ev| {
                            username.set(event_target_value(&ev));
                        }
                    />
                </label>

                <label>
                    <span>"password"</span>
                    <input
                        type="password"
                        prop:value=password
                        on:input=move |ev| {
                            password.set(event_target_value(&ev));
                        }
                    />
                </label>

                <button
                    type="submit"
                    disabled=move || loading.get()
                >
                    {move || if loading.get() {
                        "Logging in..."
                    } else {
                        "Log in"
                    }}
                </button>

                {move || error.get().map(|error| view! {
                    <p class="error">{error}</p>
                })}
            </form>
            </div>
        </section>
    }
}
