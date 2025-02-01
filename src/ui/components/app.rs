use dioxus::prelude::*;
use tracing::debug;

use crate::ui::components::{ClientClickCounter, GetPeerCount, ServerClickCounter};

#[component]
pub fn App() -> Element {
    debug!("App is rendering");
    rsx! {
        p { "Hello, world" }
        GetPeerCount{}
        ClientClickCounter{}
        ServerClickCounter{}
    }
}
