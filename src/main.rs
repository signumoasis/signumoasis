use std::{
    env,
    thread::{self, JoinHandle},
};

use dioxus::prelude::*;
use signum_node_rs::{telemetry, ui::components::App};
use tracing::{error, info};

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

    // INFO: Begin by setting up tracing
    telemetry::init_subscriber("signum-node-rs".into(), "info".into(), std::io::stdout);

    // INFO: Get any args from CLI
    let args: Vec<String> = env::args().collect();
    let headless = args.contains(&"--headless".to_owned());

    // TODO: Set up database here. Can be used to store app settings as well as plugin data
    // in different namespaces

    // INFO Create a place to store a JoinHandle outside of the server-only scope
    #[cfg(feature = "server")]
    let server_join: Option<JoinHandle<_>>;

    // INFO: Do things only necessary on the server
    #[cfg(feature = "server")]
    {
        use signum_node_rs::server_stuff::load_plugins;

        info!("Loading server");

        let plugin_package = load_plugins();

        // TODO: pass all plugindata to the tokio runtim on the server below to allow it to register stuff
        server_join = Some(thread::spawn(move || {
            use signum_node_rs::server;
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(server::setup())
        }));
    }

    // INFO: Launch desktop app code
    #[cfg(feature = "desktop")]
    if !headless {
        info!("Loading desktop gui");
        LaunchBuilder::desktop().launch(App);
    } else {
        info!("Running in headless mode. Stop with CTRL-C.");
    }

    // INFO: WASM-only code. Only runs in the WASM bin
    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    LaunchBuilder::web().launch(App);

    // INFO: This is needed to we can run the server with or without the desktop/app/wasm GUI
    #[cfg(feature = "server")]
    {
        // If headless, await ctrl-c
        if let Some(handle) = server_join {
            handle.join().unwrap();
            info!("Received CTRL-C. Exiting.")
        } else {
            error!("Not able to get server join handle");
        }
    }
}
