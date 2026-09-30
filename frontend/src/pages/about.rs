use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About Lukiana | Lukiana's Blog" />
        <Meta name="description" content="Learn more about Lukiana, her interests in software, cyber security, open source, and music." />
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>

            <section class="page-section">
                <div class="about-container">
                        <div class="person-header">
                            <h1>About me</h1>
                            <div class="person-image">
                                <img src="./profile-image.jpg" alt="Portrait of Lukiana" width="508" height="680" />
                            </div>
                        </div>

                        <div class="person-info">
                            <h2><span class="shell_dollar">$</span> whoami</h2>
                            <p>"Hey my name is Lukiana (she/her)! I'm 18 years old and a computer science Student. I'm passionate about computers, cyber security and OSS~ In my free time I occasionally dabble in hobby politics. I'm a part time misanthropic drunken looser and a full time nerd. I like writing software In rust, use Arch Linux and an old ThinkPad as my laptop. I am also really into homelabbing, this site is also hosted at home, so please be nice to it. During the day time I'm a student at the TU Darmstadt."</p>

                            <h2><span class="shell_dollar">$</span> cat favorite_artists.txt</h2>
                            <p>"Pat The Bunny, Title Fight, The Front Bottoms, Anda Morts, Modern Baseball, Pigeon Pit, Car Seat Headrest, ESA, Asking for it"</p>
                        </div>

                        <div class="gif-buttons">
                            <div class="gif-row">
                                <img src="./gifs/acab.gif" alt="ACAB badge" width="88" height="31" loading="lazy" />
                                <img src="./gifs/antifa.gif" alt="Antifa badge" width="88" height="31" loading="lazy" />
                                <img src="./gifs/archlinux.gif" alt="Arch Linux badge" width="88" height="31" loading="lazy" />
                                <img src="./gifs/transbian.svg" alt="Transbian badge" width="88" height="31" loading="lazy" />
                                <img src="./gifs/neovim.gif" alt="Neovim badge" width="88" height="31" loading="lazy" />
                            </div>
                            <div class="gif-row">
                                <img src="./gifs/paws.gif" alt="Paws badge" width="88" height="31" loading="lazy" />
                                <img src="./gifs/tranarchy.gif" alt="Tranarchy badge" width="88" height="31" loading="lazy" />
                                <img src="./gifs/transistor_cafe.gif" alt="Transistor Cafe badge" width="88" height="31" loading="lazy" />
                                <img src="./gifs/tsis.gif" alt="TSIS badge" width="88" height="31" loading="lazy" />
                                <img src="./gifs/tyg.gif" alt="TYG badge" width="88" height="31" loading="lazy" />
                            </div>
                        </div>

                </div>
            </section>
        </ErrorBoundary>
    }
}
