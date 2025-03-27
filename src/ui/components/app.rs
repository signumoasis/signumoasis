use dioxus::prelude::*;
use tracing::debug;

use crate::{
    protocols::b1::ui::B1PeerList,
    ui::components::{ClientClickCounter, GetPeerCount, ServerClickCounter},
};

#[component]
pub fn App() -> Element {
    debug!("App is rendering");
    rsx! {
        p { "Hello, world" }
        GetPeerCount{}
        B1PeerList{}
        ClientClickCounter{}
        ServerClickCounter{}
    }
}
