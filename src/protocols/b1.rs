#![cfg(feature = "server")]
//pub mod api;
//pub mod b1_peer;
pub mod b1_configuration;
pub mod models;
//pub mod peer_finder;
//pub mod peer_info_trader;
//pub mod peers;
mod b1_datastore;

pub use b1_datastore::B1Datastore;

use crate::{common::datastore::Datastore, configuration::Settings, server_stuff::PluginData};
use axum::Router;
use std::sync::mpsc;

use super::{ChainMessage, Protocol};

pub struct B1Protocol {}

impl Protocol for B1Protocol {
    fn register_settings() {}

    fn initialize(
        settings: Settings,
        router: Router,
        chain_message_tx: mpsc::Sender<ChainMessage>,
    ) {
        // TODO: get b1datastore from settings
        //let _database: B1Datastore = database.into();

        // INFO: This mpsc is used for communication of the client API to the B1 tasks
        let (_b1_internal_tx, _b1_internal_rx) = mpsc::channel::<B1InternalMessage>();

        // TODO: Create routes for the main API - Ensure clone of chain_channel_tx per necessary endpoint

        // TODO: Create tasks for the main runner to run - Ensure clone of chain_channel_tx per necessary task
    }
}

// TODO: Flesh out the messages used between B1 client API and tasks
pub(crate) enum B1InternalMessage {}
