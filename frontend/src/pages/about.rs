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

                    <div class="person-column">
                        <div class="person-header">
                            <h1>Leonor</h1>
                            <div class="person-image">
                                <img src="/assets/leonor-photo.jpeg" alt="Leonor" />
                            </div>
                        </div>

                        <div class="person-info">
                            <h2>whomi</h2>
                            <p>"Hi im leonor :3 Im 18 and right now go to school for technical theatre. My goal one day is to work board for different venues or things that need a sound tech to be there, and also get married. I play magic the gathering and  trumpet and have ever since i was small. I love my gf sm and she is the one who made this website bc shes super cool and knows how to do that somehow. I like to smoke weed and watch tv, and i rlly love my gf."</p>

                            <h2>Favorite Artists</h2>
                            <p>underscores, grimes, arca, alice longyu gao, femtanyl, shygirl, azealia banks</p>
                        </div>

                        <div class="gif-buttons">
                            <div class="gif-row">
                                <img src="/assets/leonor/gifs/4ever.gif" alt="4ever" />
                                <img src="/assets/leonor/gifs/12men.gif" alt="12men" />
                                <img src="/assets/leonor/gifs/boc.gif" alt="boc" />
                                <img src="/assets/leonor/gifs/eat.gif" alt="eat" />
                                <img src="/assets/leonor/gifs/forever_online.gif" alt="forever online" />
                            </div>
                            <div class="gif-row">
                                <img src="/assets/leonor/gifs/happy-mix_button.gif" alt="happy mix" />
                                <img src="/assets/leonor/gifs/hartscorned.gif" alt="hartscorned" />
                                <img src="/assets/leonor/gifs/notperfect.gif" alt="notperfect" />
                                <img src="/assets/leonor/gifs/onionlink1098.gif" alt="onionlink1098" />
                                <img src="/assets/leonor/gifs/transnow2.gif" alt="transnow" />
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </ErrorBoundary>
    }
}
