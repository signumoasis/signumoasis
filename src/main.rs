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

    // Begin by setting up tracing
    telemetry::init_subscriber("signum-node-rs".into(), "info".into(), std::io::stdout);

    let args: Vec<String> = env::args().collect();
    let headless = args.contains(&"--headless".to_owned());

    // TODO: Set up database here. Can be used to store app settings as well as plugin data
    // in different namespaces

    #[cfg(feature = "server")]
    let server_join: Option<JoinHandle<_>>;

    #[cfg(feature = "server")]
    {
        use server_stuff::load_plugins;

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
        if let Some(handle) = server_join {
            handle.join().unwrap();
            info!("Received CTRL-C. Exiting.")
        } else {
            error!("Not able to get server join handle");
        }
    }
}

#[cfg(feature = "server")]
mod server_stuff {
    use std::{future::Future, pin::Pin, sync::mpsc};

    use signum_node_rs::configuration::Settings;
    use surrealdb::{engine::any::Any, Surreal};

    pub fn load_plugins() -> Vec<PluginData> {
        let x = run_peer_finder_forever;
        Vec::new()
    }

    pub struct PluginData {
        plugin_id: String,
        route_definitions: Vec<axum::Router>,
        spawnable_tasks:
            Vec<fn(Surreal<Any>, Settings) -> Pin<Box<dyn Future<Output = anyhow::Result<()>>>>>,
    }

    pub trait Protocol {
        fn setup(chain_channel_tx: mpsc::Sender<ChainMessage>) -> PluginData;
    }

    /// Messages to the Chain
    pub enum ChainMessage {
        GetBlock,
        ProcessBlocks,
    }

    /// Messages to the Plugins
    pub enum PluginMessage {
        GetMoreBlocks,
        BadBlock,
    }

    pub async fn run_peer_finder_forever(database: String, settings: String) -> anyhow::Result<()> {
        todo!()
    }
}
