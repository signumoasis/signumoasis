use dioxus::prelude::*;

use crate::common::count_peers;

#[component]
pub fn GetPeerCount() -> Element {
    let mut server_count_resource = use_server_future(count_peers)?;
    let server_count = server_count_resource().unwrap().unwrap_or_default();

    rsx! {
        p { id: "peer_count_display", "Peer Count: {server_count}" }
        button {
            id: "peer_count_clicks",
            onclick: move |_| async move {
                tracing::debug!("Clicked get peer count button");
                server_count_resource.restart();
            },
            "Get Peer Count"
        }
    }
}
