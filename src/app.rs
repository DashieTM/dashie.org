use futures::executor;
use futures::Future;
use rust_wasm_websys_utils::websysmod::*;
use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::Document;
use web_sys::*;
use yew::prelude::*;
// #[wasm_bindgen(module = "/stapid.js")]
//  extern {
//     async fn initwasm() -> JsValue;
//  }
#[wasm_bindgen(module = "/stapid.js")]
extern "C" {
    fn unhide();
    fn hide();
}

// async fn init_stapid() {
//        initwasm().await;
// }
// fn init_stapid() {
//        initwasm();
// }

#[function_component(App)]
pub fn app() -> Html {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let matrix_button = Callback::from(move |_| {
        web_sys::window()
            .expect("The window is gone, wtf?")
            .location()
            .assign("https://matrix.to/#/%21sdKHhlAnzYUzkxvRfX%3Amatrix.org?via=matrix.org");
    });
    let youtube_button = Callback::from(move |_| {
        web_sys::window()
            .expect("The window is gone, wtf?")
            .location()
            .assign("https://www.youtube.com/channel/UCFWznIjnswyoEv67WbHiJzg");
    });
    let mail_button = Callback::from(move |_| {
        web_sys::window()
            .expect("The window is gone, wtf?")
            .location()
            .assign("mailto:derkaktus.yt@gmail.com");
    });
    let nextcloud_button = Callback::from(move |_| {
        web_sys::window()
            .expect("The window is gone, wtf?")
            .location()
            .assign("https://dashie.org:12002");
    });

    let play_stapid = Callback::from(move |_| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let games = document.get_element_by_id("Games");
        games
            .expect("The window is gone, wtf?")
            .class_list()
            .add_1("hidden");
        // executor::block_on(init_stapid());
        // init_stapid();
        unhide();
    });

    let home_button_click = Callback::from(move |_| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let games = document.get_element_by_id("Games");
        let home = document.get_element_by_id("Home");
        games
            .expect("The window is gone, wtf?")
            .class_list()
            .add_1("hidden");
        home.expect("The window is gone, wtf?")
            .class_list()
            .remove_1("hidden");
        hide();
    });

    let games_button_click = Callback::from(move |_| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let home = document.get_element_by_id("Home");
        let games = document.get_element_by_id("Games");
        games
            .expect("The window is gone, wtf?")
            .class_list()
            .remove_1("hidden");
        home.expect("The window is gone, wtf?")
            .class_list()
            .add_1("hidden");
        hide();
    });

    html! {
    <>
        <main>
            <nav>
              <button id="HomeButton" class="button" onclick={ home_button_click }>{ "Home" }</button>
              <button id="GamesButton" onclick={ games_button_click }>{ "Games" }</button>
            </nav>
            <article id="Home" class="grid">
              <h1>{ "Just another dev, or at least pretending to be." }</h1>
              <h2>{ "Some Experience" }</h2>
              <img src="https://img.shields.io/badge/HTML5%20-%23E34F26.svg?style=for-the-badge&logo=html5&logoColor=white" alt="HTML"/>
              <img src="https://img.shields.io/badge/CSS%20-%231572B6.svg?style=for-the-badge&logo=css3&logoColor=white" alt=""/>
              <img src="https://img.shields.io/badge/JavaScript%20-%23F7DF1E.svg?style=for-the-badge&logo=javascript&logoColor=black" alt=""/>
              <img src="https://img.shields.io/badge/C%20-%232370ED.svg?style=for-the-badge&logo=c&logoColor=white" alt=""/>
              <img src="https://img.shields.io/badge/lua-%232C2D72.svg?style=for-the-badge&logo=lua&logoColor=white" alt=""/>
              <img src="https://img.shields.io/badge/Haskell-5e5086?style=for-the-badge&logo=haskell&logoColor=white" alt=""/>
              <h2>{ "Decent Experience" }</h2>
              <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt=""/>
              <img src="https://img.shields.io/badge/C++%20-%2300599C.svg?style=for-the-badge&logo=c%2B%2B&logoColor=white" alt=""/>
              <img src="https://img.shields.io/badge/shell_script-%23121011.svg?style=for-the-badge&logo=gnu-bash&logoColor=white" alt=""/>
              <img src="https://img.shields.io/badge/latex-%23008080.svg?style=for-the-badge&logo=latex&logoColor=white" alt=""/>
              <img src="https://img.shields.io/badge/Python%20-%2314354C.svg?style=for-the-badge&logo=python&logoColor=white" alt=""/>
              <h2>{ "Tools" }</h2>
              <img src="https://img.shields.io/badge/git-%23F05033.svg?style=for-the-badge&logo=git&logoColor=white" alt=""/>
              <img src="https://img.shields.io/badge/Penguin_OS-FCC624?style=for-the-badge&logo=linux&logoColor=black" alt=""/>
              <img src="https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white" alt=""/>
              <img src="https://img.shields.io/badge/NeoVim-%2357A143.svg?&style=for-the-badge&logo=neovim&logoColor=white" alt=""/>
            <aside>
              <button id="Nextcloud" onclick={ nextcloud_button }>{ "NextCloud" }</button>
              <button id="Youtube" onclick={ youtube_button }>{ "Youtube" }</button>
              <button id="Email" onclick={ mail_button }>{ "Email" }</button>
              <button id="Matrix" onclick={ matrix_button } >{ "Matrix" }</button>
            </aside>
            </article>
            <article id="Games" class="hidden">
              <h2>{ "Shitgaem" }</h2>
              <iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/Z9ddOhKWpTQ" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true class="video"></iframe>
                <h2>
                <a href="https://github.com/Xetibo/stapid">{ "Stapid" }</a>
                </h2>
              <div>
                <button id="StapidButton" onclick={ play_stapid }>{ "Play Stapid" }</button>
              </div>
              <img src="./websrc/stapid.png" alt="Picture of Stapid"/>
            </article>
        </main>
    </>
    }
}
