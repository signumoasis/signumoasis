use std::env;

use dioxus::prelude::*;
use signum_node_rs::{telemetry, ui::components::App};
use tracing::info;

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

fn main() {
    // INFO: Begin by setting up tracing
    telemetry::init_subscriber("signum-node-rs".into(), "info".into(), std::io::stdout);

    // INFO: Get any args from CLI
    let args: Vec<String> = env::args().collect();
    let headless = args.contains(&"--headless".to_owned());

    // TODO: Set up database here. Can be used to store app settings as well as plugin data
    // in different namespaces

    // INFO: Do things only necessary on the server
    #[cfg(feature = "server")]
    {
        info!("Loading server");

        use signum_node_rs::server;
        let runner = tokio::runtime::Runtime::new().expect("unable to get a tokio runtime");
        runner.spawn(server::setup());
        if headless || !cfg!(feature = "desktop") {
            info!("Running in headless mode. Stop with CTRL-C.");
            runner.block_on(async {
                tokio::signal::ctrl_c()
                    .await
                    .expect("couldn't listen for ctrl-c for some reason");
            });
        }
        // TODO: Pass database and settings into this...somehow
    }

    // INFO: Launch desktop app code
    #[cfg(feature = "desktop")]
    if !headless {
        info!("Loading desktop gui");
        LaunchBuilder::desktop().launch(App);
    }

    // INFO: WASM-only code. Only runs in the WASM bin
    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    LaunchBuilder::web().launch(App);
}
