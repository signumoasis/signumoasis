use dioxus::prelude::*;

use crate::{
    protocols::b1::ui::B1PeerList,
    ui::components::{ClientClickCounter, GetPeerCount, ServerClickCounter},
};

#[component]
pub fn App() -> Element {
    tracing::debug!("App UI component is rendering");
    rsx! {
        p { "Hello, world" }
        GetPeerCount{}
        B1PeerList{}
        ClientClickCounter{}
        ServerClickCounter{}
    }
}
