#![cfg(feature = "server")]
//pub mod api;
//pub mod b1_peer;
mod b1_configuration;
mod b1_datastore;
mod b1_peer;
pub mod models;
mod peer_finder;
mod peer_info_trader;
pub mod peers;
//pub mod peer_finder;
//pub mod peer_info_trader;
//pub mod peers;

pub use b1_configuration::B1Settings;
pub use b1_datastore::B1Datastore;
pub use b1_peer::B1Peer;
use peer_finder::*;
use peer_info_trader::*;

use crate::{common::datastore::Datastore, configuration::Settings, protocols::report_exit};
use axum::Router;
use std::sync::mpsc;

use super::{ChainMessage, PluginMessage, Protocol};

pub struct B1Protocol {
    datastore: B1Datastore,
    settings: B1Settings,
    _to_chain_tx: mpsc::Sender<ChainMessage>,
}

impl Protocol for B1Protocol {
    #[tracing::instrument(skip_all)]
    async fn run(&self) {
        tracing::info!("Starting B1 Protocol");
        let (_from_chain_tx, _from_chain_rx) = mpsc::channel::<PluginMessage>();

        // Create the Block Downloader task
        //let block_downloader_task = tokio::spawn(run_block_downloader_forever(
        //    database.clone(),
        //    configuration.clone(),
        //));

        // Create the p2p api webserver task
        //let p2p_api = SrsApiApplication::build(configuration.clone(), database.clone()).await?;
        //let p2p_api_task = tokio::spawn(p2p_api.run_until_stopped());

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
            //o = p2p_api_task => report_exit("P2P API Server", o),
            o = peer_finder_task => report_exit("Peer Finder", o),
            o = peer_info_trader_task => report_exit("Peer Info Trader", o),
        };
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

    fn register_routes(router: Router) -> Router {
        router
    }
}

// TODO: Flesh out the messages used between B1 client API and tasks
pub(crate) enum B1InternalMessage {}
