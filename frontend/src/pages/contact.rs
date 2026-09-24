use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <main class="legal-container">
                <header class="legal-header">
                    <p class="legal-eyebrow">"ping lukiana.gay"</p>
                    <h1>"How to contact me"</h1>
                </header>

            <article class="legal-content">
                <section>
                    <p>
                        "The best way to contact me is via my email (mail [at] weger [dot] dev)."
                    </p>
                    <p>
                        "This is my PGP key: "
                        <pre class="pgp-key">
"
-----BEGIN PGP PUBLIC KEY BLOCK-----
mDMEarTgFhYJKwYBBAHaRw8BAQdAAESmEosdbi6QKYgdB60VKuOewftH4WNRG1Fs
7knAtFi0Hkx1a2lhbmEgV2VnZXIgPG1haWxAd2VnZXIuZGV2PoiWBBMWCgA+FiEE
MPnNoEaayp/BsjOzGl0RyCJ9/xEFAmq04BYCGwMFCQWjmoAFCwkIBwIGFQoJCAsC
BBYCAwECHgECF4AACgkQGl0RyCJ9/xF+nQD+OnU8z2JloQE5pUAKn3PQkbP4JqmC
dBJWuo20sfG/B54A/3/dPRwm8L+667HOmAwMzy5edD6rDt6d6Jjhetko/NsCuDgE
arTgFhIKKwYBBAGXVQEFAQEHQKfOIQux+kT5CebsQvP7ErQSF96MLZ2niTicC5lT
/HYLAwEIB4h+BBgWCgAmFiEEMPnNoEaayp/BsjOzGl0RyCJ9/xEFAmq04BYCGwwF
CQWjmoAACgkQGl0RyCJ9/xFLEQEAurKEV7E4oIuAzNQBOefnFo0nXejIC9q4K7ky
nIvGwhoA/jdKa1NsKpf/e4VRGgSMR/ASi5ddHXpQ3lWw14n5UG4P=O9fq
-----END PGP PUBLIC KEY BLOCK-----
"
                        </pre>
                    </p>
                    <p>
                        "You can also reach me via my signal: @luk14n4.66"
                    </p>
                </section>
            </article>
        </main>
    }
}
