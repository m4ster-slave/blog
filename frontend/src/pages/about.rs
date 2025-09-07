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
                            <h1>Lukiana</h1>
                            <div class="person-image">
                                <img src="/assets/lukiana-photo.jpg" alt="Lukiana" />
                            </div>
                        </div>

                        <div class="person-info">
                            <h2>whomi</h2>
                            <p>"I'm passionate about the computer, cyber security and OSS~ In my free time i ocassionaly dabble in hobby politics. Im a part time misanthropic drunken looser and a full time wife lover (Seriously she is the love of my life and i love her soo much). An interesting funfact about me is that i wear the same patched up pants everyday (one might call them patchpants but im not ready to commit to being a punk like that) and i dont do it because im poor but because im autistic and like wearing the same pants everyday"</p>

                            <h2>Favorite Artists</h2>
                            <p>Title Fight, The Front Bottoms Anda Morts, Modern Baseball, Pigeon Pit, Car Seat Headrest, ESA, Asking for it, JHONNY HOBO AND THE FREIGHT TRAINS, Knochenfabrik</p>
                        </div>

                        <div class="gif-buttons">
                            <div class="gif-row">
                                <img src="/assets/lukiana/gifs/acab.gif" alt="acab" />
                                <img src="/assets/lukiana/gifs/antifa.gif" alt="antifa" />
                                <img src="/assets/lukiana/gifs/archlinux.gif" alt="archlinux" />
                                <img src="/assets/lukiana/gifs/ffmpeg.gif" alt="ffmpeg" />
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

                    <div class="person-column">
                        <div class="person-header">
                            <h1>Leonor</h1>
                            <div class="person-image">
                                <img src="/assets/leonor-photo.png" alt="Leonor" />
                            </div>
                        </div>

                        <div class="person-info">
                            <h2>whomi</h2>
                            <p>"I'm deeply into visual arts, illustration, and graphic design. I spend my days creating digital artwork, sketching in my notebooks, and exploring different art mediums. I also love watching indie films, collecting vintage books, and practicing yoga. Gardening and cooking are my zen activities."</p>

                            <h2>Favorite Artists</h2>
                            <p>underscores, grimes, arca, alice longyu gao, femtanyl</p>
                        </div>

                        <div class="gif-buttons">
                            <div class="gif-row">
                                <img src="https://cyber.dabamos.de/88x31/misc/piracy_now.gif" alt="Piracy Now" />
                                <img src="https://cyber.dabamos.de/88x31/misc/fuck_nfts.gif" alt="Fuck NFTs" />
                                <img src="https://cyber.dabamos.de/88x31/misc/made_with_notepad.gif" alt="Made with Notepad" />
                                <img src="https://cyber.dabamos.de/88x31/misc/hand_coded.gif" alt="Hand Coded" />
                                <img src="https://cyber.dabamos.de/88x31/misc/best_viewed_on_crt.gif" alt="Best viewed on CRT" />
                            </div>
                            <div class="gif-row">
                                <img src="https://cyber.dabamos.de/88x31/misc/html5_powered.gif" alt="HTML5 Powered" />
                                <img src="https://cyber.dabamos.de/88x31/misc/css3_powered.gif" alt="CSS3 Powered" />
                                <img src="https://cyber.dabamos.de/88x31/misc/javascript_free.gif" alt="JavaScript Free" />
                                <img src="https://cyber.dabamos.de/88x31/misc/geocities.gif" alt="Geocities" />
                                <img src="https://cyber.dabamos.de/88x31/misc/queer_pride.gif" alt="Queer Pride" />
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </ErrorBoundary>
    }
}
