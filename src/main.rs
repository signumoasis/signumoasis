use std::sync::{Arc, Mutex};

use dioxus::{logger::tracing::debug, prelude::*};
use dioxus_fullstack::once_cell::sync::Lazy;

fn main() {
    // TODO: Do these:
    // 3. Add protocol module registration for axum server, new port app, and tasks registering with chain

    // Initialize logger since the dioxus::launch isn't around to do it
    dioxus::logger::initialize_default(); // TODO: Change this to my own telemetry

    #[cfg(feature = "web")]
    dioxus_web::launch::launch_cfg(App, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "server")]
    {
        use dioxus_fullstack::server::DioxusRouterExt;

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                //connect to database

                // TODO: Register plugin routes to this API
                // TODO: Add any plugin-returned router/port combinations to the list of servers to spawn
                let app =
                    axum::Router::new().serve_dioxus_application(ServeConfig::new().unwrap(), App);

                let socket_address = dioxus_cli_config::fullstack_address_or_localhost();
                let listener = tokio::net::TcpListener::bind(&socket_address)
                    .await
                    .unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            })
    }
    #[cfg(all(not(feature = "server"), not(feature = "web")))]
    // Launch in desktop mode if neither "server" nor "web" is enabled
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        p { "Hello, world" }
        ClientClickCounter{}
        ServerClickCounter{}
    }
}

#[component]
fn ClientClickCounter() -> Element {
    let mut count = use_signal(|| 5);
    rsx! {
        p { id: "count_display", "{count}" }
        button {
            id: "count_clicks",
            onclick: move |_| {
               *count.write() += 1;
            },
            "CLIENT - CLICK ME!"
        }
    }
}

#[component]
fn ServerClickCounter() -> Element {
    let mut server_count = use_signal(|| 0);

    rsx! {
        p { id: "server_count_display", "Server Count: {server_count}" }
        button {
            id: "server_count_clicks",
            onclick: move |_| async move {
                if let Ok(r) = serverside_click_counter().await {
                   server_count.set(r);
                }

            },
            "SERVER - CLICK ME!"
        }
    }
}

#[cfg(feature = "server")]
static GLOBAL_COUNTER: Lazy<Arc<Mutex<u32>>> = Lazy::new(|| Arc::new(Mutex::new(0)));

#[server(endpoint = "scount")]
async fn serverside_click_counter() -> Result<u32, ServerFnError> {
    let mut counter = GLOBAL_COUNTER.lock().unwrap();
    *counter += 1;
    debug!("Global Counter: {}", *counter);
    Ok(*counter)
}
