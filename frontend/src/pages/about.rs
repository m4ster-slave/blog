use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
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
                    <div class="person-column">
                        <div class="person-header">
                            <h1>About me</h1>
                            <div class="person-image">
                                <img src="/assets/lukiana-photo.webp" alt="Lukiana" />
                            </div>
                        </div>

                        <div class="person-info">
                            <h2>whoami</h2>
                            <p>"Hey my name is Lukiana! I'm passionate about the computer, cyber security and OSS~ In my free time i ocassionaly dabble in hobby politics. Im a part time misanthropic drunken looser and a full time nerd. I like writing software In rust, use Arch Linux and use an old ThinkPad as my laptop - I am the whole stereotype package."</p>

                            <h2>Favorite Artists</h2>
                            <p>Pat The Bunny, Title Fight, The Front Bottoms Anda Morts, Modern Baseball, Pigeon Pit, Car Seat Headrest, ESA, Asking for it, Knochenfabrik</p>
                        </div>

                        <div class="gif-buttons">
                            <div class="gif-row">
                                <img src="/assets/lukiana/gifs/acab.gif" alt="acab" />
                                <img src="/assets/lukiana/gifs/antifa.gif" alt="antifa" />
                                <img src="/assets/lukiana/gifs/archlinux.gif" alt="archlinux" />
                                <img src="/assets/lukiana/gifs/transbian.svg" alt="transbian" />
                                <img src="/assets/lukiana/gifs/neovim.gif" alt="neovim" />
                            </div>
                            <div class="gif-row">
                                <img src="/assets/lukiana/gifs/paws.gif" alt="paws" />
                                <img src="/assets/lukiana/gifs/tranarchy.gif" alt="tranarchy" />
                                <img src="/assets/lukiana/gifs/transistor_cafe.gif" alt="transistor_cafe" />
                                <img src="/assets/lukiana/gifs/tsis.gif" alt="tsis" />
                                <img src="/assets/lukiana/gifs/tyg.gif" alt="tyg" />
                            </div>
                        </div>
                    </div>

                </div>
            </section>
        </ErrorBoundary>
    }
}
