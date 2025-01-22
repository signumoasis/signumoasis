use std::sync::atomic::{AtomicU32, Ordering};

use dioxus::{logger::tracing::debug, prelude::*};

fn main() {
    // TODO: Do these:
    // 3. Add protocol module registration for axum server, new port app, and tasks registering with chain

    // Initialize logger since the dioxus::launch isn't around to do it
    dioxus::logger::initialize_default(); // TODO: Change this to my own telemetry

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
                    axum::Router::new().serve_dioxus_application(ServeConfigBuilder::new(), App);

                let socket_address = dioxus_cli_config::fullstack_address_or_localhost();
                let listener = tokio::net::TcpListener::bind(&socket_address)
                    .await
                    .unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            })
    }
    #[cfg(not(feature = "server"))]
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
    let mut server_count_resource = use_server_future(serverside_counter_get)?;
    let server_count = server_count_resource().unwrap().unwrap_or_default();

    rsx! {
        p { id: "server_count_display", "Server Count: {server_count}" }
        button {
            id: "server_count_clicks",
            onclick: move |_| async move {
                serverside_counter_increment().await;
                server_count_resource.restart();
            },
            "SERVER - CLICK ME!"
        }
    }
}

#[cfg(feature = "server")]
static GLOBAL_COUNTER: AtomicU32 = AtomicU32::new(0);

#[server(endpoint = "get_counter")]
async fn serverside_counter_get() -> Result<u32, ServerFnError> {
    let counter = GLOBAL_COUNTER.load(Ordering::Relaxed);
    Ok(counter)
}

#[server(endpoint = "increment_counter")]
async fn serverside_counter_increment() -> Result<(), ServerFnError> {
    let counter = GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed);
    debug!("Global Counter: {}", counter);
    Ok(())
}
