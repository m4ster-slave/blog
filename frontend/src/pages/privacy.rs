use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn Privacy() -> impl IntoView {
    view! {
        <Title text="Privacy Policy | Lukiana's Blog" />
        <Meta name="description" content="Read the privacy policy for Lukiana's blog and learn how anonymous usage and technical data are handled." />
        <main class="legal-container">
                <header class="legal-header">
                    <p class="legal-eyebrow">"cat privacy_policy.md"</p>
                    <h1>"Privacy Policy"</h1>
                    <p class="legal-updated">"Last updated: September 23, 2026"</p>
                </header>

            <article class="legal-content">
                <section>
                    <h2>"1. Overview"</h2>
                    <p>
                        "This website is operated by me, Lukiana Weger. This privacy policy
                        explains what information may be collected when you use this website
                        and how that information is handled."
                    </p>
                </section>

                <img src="./memes/privacy_policy_meme.webp" alt="Privacy policy meme" width="350" height="350" loading="lazy" />

                <section>
                    <h2>"2. Information I Collect"</h2>
                    <p>
                        "This website collects anonymous usage data for post view counts and the open stats displayed in /stats. There is nothing malicious or fishy done with the data."
                    </p>
                    <p>
                        "Technical information such as your IP address, browser type,
                        operating system, and request timestamps may be processed
                        automatically by the my hosting infrastructure (HP Z220 in my basement) for security,
                        reliability, and operational purposes."
                    </p>
                </section>

                <section>
                    <h2>"3. Cookies"</h2>
                    <p>
                        "This website does not use cookies.
                        If this changes, this policy will be updated accordingly. The only cookies that exist are admin session cookies but only I have those hehe B)"
                    </p>
                </section>

                <section>
                    <h2>"4. Third-Party Services"</h2>
                    <p>
                        "External services may process technical information when you
                        access resources hosted by or linked to third-party providers.
                        Their own privacy policies apply to information they process. This website is behind the cloudflare proxy system so idk what they do with your data."
                    </p>
                </section>

                <section>
                    <h2>"5. External Links"</h2>
                    <p>
                        "This website may contain links to external websites. I am not
                        responsible for the privacy practices or content of third-party
                        websites."
                    </p>
                </section>

                <section>
                    <h2>"6. Your Rights"</h2>
                    <p>
                        "Depending on your jurisdiction, you may have rights regarding
                        personal data processed about you, including rights of access,
                        correction, deletion, restriction, or objection. I don't collect that kind of data tho... you stil have that right so good for you i guess yay"
                    </p>
                </section>

                <section>
                    <h2>"7. Contact"</h2>
                    <p>
                        "For privacy-related questions or requests, contact the site
                        me through the contact information provided on this website. Thank you!"
                    </p>
                </section>
            </article>
        </main>
    }
}
