use dioxus::prelude::*;

use crate::ui::components::{ClientClickCounter, GetPeerCount, ServerClickCounter};

#[component]
pub fn App() -> Element {
    tracing::debug!("App UI component is rendering");
    rsx! {
        p { "Hello, world" }
        GetPeerCount{}
        ClientClickCounter{}
        ServerClickCounter{}
    }
}
