use dioxus::prelude::*;

use crate::ui::backend::{serverside_counter_get, serverside_counter_increment};

#[component]
pub fn ServerClickCounter() -> Element {
    let mut server_count_resource = use_server_future(serverside_counter_get)?;
    let server_count = server_count_resource().unwrap().unwrap_or_default();

    rsx! {
        p { id: "server_count_display", "Server Count: {server_count}" }
        button {
            id: "server_count_clicks",
            onclick: move |_| async move {
                tracing::trace!("Clicked server count button");
                let _ = serverside_counter_increment().await;
                server_count_resource.restart();
            },
            "SERVER - CLICK ME!"
        }
    }
}
