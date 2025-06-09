
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Title { "tpl-dioxus" }
        document::Meta { charset: "UTF-8" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: "https://cdn.master.co/normal.css" }
        document::Script { src: "https://cdn.master.co/css-runtime@rc" }
        Configure {}

        main {
            h1 {
                class: "italic",
                "Welcome to tpl-dioxus!"
            }
        }
    }
}

#[component]
fn Configure() -> Element {
    rsx! {
        
    }
}
