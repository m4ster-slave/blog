use leptos::prelude::*;

#[component]
pub fn Tos() -> impl IntoView {
    view! {
        <main class="legal-container">
            <header class="legal-header">
                <p class="legal-eyebrow">"cat tos.md"</p>
                <h1>"Terms of Service"</h1>
                <p class="legal-updated">"Last updated: September 23, 2026"</p>
            </header>

            <article class="legal-content">
                <section>
                    <h2>"1. Acceptance of Terms"</h2>
                    <p>
                        "By accessing or using this website, you agree to these Terms
                        of Service. If you do not agree with these terms, you should
                        not use the website :P"
                    </p>
                </section>

                <section>
                    <h2>"2. Use of the Website"</h2>
                    <p>
                        "You may use this website for lawful purposes only. You agree
                        not to misuse the website, attempt to gain unauthorized access
                        to its systems, interfere with its operation, or use it in a
                        way that violates applicable laws."
                    </p>
                </section>

                <section>
                    <h2>"3. Intellectual Property"</h2>
                    <p>
                        "Unless otherwise stated, the content, design, source code,
                        graphics, and other materials on this website are owned by
                        Lukiana Weger or used with permission. GPLv3 license applies, would still be nice to ask me tho :3"
                    </p>
                    <p>
                        "You may not reproduce, redistribute, or exploit anything on here for commercial purposes, or use
                        website content inappropriately."
                    </p>
                    <img src="./memes/sharing_data_meme.webp"></img>
                </section>

                <section>
                    <h2>"4. External Links"</h2>
                    <p>
                        "This website may link to third-party websites. These links are
                        provided for convenience and do not imply control or endorsement
                        of those websites."
                    </p>
                </section>

                <section>
                    <h2>"5. Availability"</h2>
                    <p>
                        "The website is provided on an availability basis. No guarantee
                        is made that the website will always be available, uninterrupted,
                        secure, or free from errors."
                    </p>

                    <img src="./memes/testing_in_prod_meme.webp"></img>
                </section>

                <section>
                    <h2>"6. Limitation of Liability"</h2>
                    <p>
                        "To the extent permitted by applicable law, the site owner shall
                        not be liable for indirect, incidental, or consequential damages
                        arising from your use of or inability to use this website."
                    </p>
                </section>

                <section>
                    <h2>"7. Changes"</h2>
                    <p>
                        "These terms may be updated from time to time. The updated
                        version will be published on this page with a revised date."
                    </p>
                </section>

                <section>
                    <h2>"8. Contact"</h2>
                    <p>
                        "If you have questions regarding these terms, contact
                        me through the contact information provided on this website."
                    </p>
                </section>
                <img src="./memes/terms_and_conditions_meme.webp"></img>
            </article>
        </main>
    }
}
