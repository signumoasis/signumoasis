mod b1_configuration;
mod b1_datastore;
mod b1_peer;
//pub mod client_api;
pub mod p2p_api;
pub mod peer_finder;
pub mod peer_info_trader;
pub mod peers;

use std::sync::mpsc;

use axum::Router;
pub use b1_configuration::*;
pub use b1_datastore::*;
pub use b1_peer::*;
use p2p_api::B1ApiApplication;
//use client_api::{client_api_handler, get_peers};
use peer_finder::run_peer_finder_forever;
use peer_info_trader::run_peer_info_trader_forever;

use crate::{
    common::Datastore,
    configuration::Settings,
    protocols::{report_exit, traits::Protocol, ChainMessage, PluginMessage},
};

const BRS_VERSION: &str = "3.8.4";

pub struct B1Protocol {
    datastore: B1Datastore,
    settings: B1Settings,
    _to_chain_tx: mpsc::Sender<ChainMessage>,
}

impl Protocol for B1Protocol {
    #[tracing::instrument(skip_all)]
    async fn run(&self) -> Result<(), anyhow::Error> {
        tracing::info!("Starting B1 Protocol");
        let (_from_chain_tx, _from_chain_rx) = mpsc::channel::<PluginMessage>();

        // Create the Block Downloader task
        //let block_downloader_task = tokio::spawn(run_block_downloader_forever(
        //    database.clone(),
        //    configuration.clone(),
        //));

        // Create the p2p api webserver task
        let p2p_api =
            B1ApiApplication::build(self.settings.clone(), self.datastore.clone()).await?;
        let p2p_api_task = tokio::spawn(p2p_api.run_until_stopped());

        // Create the peer finder task
        let peer_finder_task = tokio::spawn(run_peer_finder_forever(
            self.datastore.clone(),
            self.settings.clone(),
        ));

        // Create the peer info trader task
        let peer_info_trader_task =
            tokio::spawn(run_peer_info_trader_forever(self.datastore.clone()));

        // Select on all the tasks to report closure status
        tokio::select! {
            //o = block_downloader_task=> report_exit("Block Downloader", o),
            o = p2p_api_task => report_exit("P2P API Server", o),
            o = peer_finder_task => report_exit("Peer Finder", o),
            o = peer_info_trader_task => report_exit("Peer Info Trader", o),
        };
        Ok(())
    }

    fn initialize(
        db: Datastore,
        settings: Settings,
        _chain_message_tx: mpsc::Sender<ChainMessage>,
    ) -> Self {
        // INFO: This mpsc is used for communication of the client API to the B1 tasks
        let (_b1_internal_tx, _b1_internal_rx) = mpsc::channel::<B1InternalMessage>();

        // TODO: Create routes for the main API - Ensure clone of chain_channel_tx per necessary endpoint

        // TODO: Create tasks for the main runner to run - Ensure clone of chain_channel_tx per necessary task
        Self {
            datastore: db.into(),
            settings: settings.b1protocol,
            _to_chain_tx: _chain_message_tx,
        }
    }

    fn register_routes(&self, router: Router) -> Router {
        let b1router = Router::new()
            // TODO: Client API should not exist here. If we want to emulate the BRS client API
            // as well as the Oasis API, it should exist in a BRS module outside of the B1 protocol
            // as the protocols are only for sending p2p data...
            // TODO: The routes here should be main-port mirrors of the B1 p2p port API routes
            // to prepare for hopeful upcoming changes to the routing of the signum BRS node's APIs.
            //.route("/", get(client_api_handler))
            .with_state(self.datastore.clone());
        router.nest("/api/b1", b1router)
    }
}

// TODO: Flesh out the messages used between B1 client API and tasks
pub(crate) enum B1InternalMessage {}
