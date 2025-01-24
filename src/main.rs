use std::{env, thread};

use dioxus::prelude::*;
use signum_node_rs::{telemetry, ui::components::App};
use tracing::info;

fn main() {
    // TODO: Steps to finish:
    // * [x] Add own telemetry compatible with dx serve if possible
    // * [ ] Load plugins
    //   * [ ] Register plugin handlers with main axum app
    //   * [ ] Register new plugin-provided axum apps, as necessary, for alternate port requirements
    // * [ ] Launch all axum servers
    // * [ ] Launch plugin-based tasks
    // * [ ] Ensure desktop option stays launchable with backend
    // * [ ] Figure out how to securely store wallet credentials locally when in desktop mode
    //       and provide an automatic login capability - excluded from WASM/web mode
    // * [ ] Find out if wasm mode can securely store credentials without leaking them to the server

    // Begin by setting up tracing
    telemetry::init_subscriber("signum-node-rs".into(), "info".into(), std::io::stdout);

    let args: Vec<String> = env::args().collect();
    let headless = args.contains(&"--headless".to_owned());

    #[cfg(feature = "server")]
    info!("Loading server");
    #[cfg(feature = "server")]
    let server_join = thread::spawn(move || {
        use dioxus_fullstack::server::DioxusRouterExt;

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                // TODO: Change serve_dioxus_application to something else as this causes it to
                // launch the wasm app even if in desktop mode. Or decide this is ok.
                let app =
                    axum::Router::new().serve_dioxus_application(ServeConfigBuilder::new(), App);

                let socket_address = dioxus_cli_config::fullstack_address_or_localhost();
                let listener = tokio::net::TcpListener::bind(&socket_address)
                    .await
                    .unwrap();

                info!("Listening on {}", socket_address);

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();

                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {
                        tracing::info!("Received shutdown signal. Exiting web server.")
                    }
                }
            })
    });

    #[cfg(feature = "desktop")]
    if !headless {
        info!("Loading desktop gui");
        LaunchBuilder::desktop().launch(App);
    } else {
        info!("Running in headless mode. Stop with CTRL-C.");
    }

    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    LaunchBuilder::web().launch(App);

    #[cfg(feature = "server")]
    {
        // If headless, await ctrl-c
        server_join.join().unwrap();
        info!("Received CTRL-C. Exiting.")
    }
}
