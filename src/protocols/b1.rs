//pub mod api;
//pub mod b1_peer;
pub mod b1_configuration;
pub mod models;
//pub mod peer_finder;
//pub mod peer_info_trader;
//pub mod peers;
mod b1_datastore;

pub use b1_datastore::B1Datastore;

use crate::{common::datastore::Datastore, server_stuff::PluginData};
use std::sync::mpsc;

use super::{ChainMessage, Protocol};

pub struct B1Protocol {}

impl Protocol for B1Protocol {
    fn register(chain_channel_tx: mpsc::Sender<ChainMessage>, database: Datastore) -> PluginData {
        let database: B1Datastore = database.into();

        // INFO: This mpsc is used for communication of the client API to the B1 tasks
        let (b1_internal_tx, mut b1_internal_rx) = mpsc::channel::<B1InternalMessage>();

        // TODO: Create routes for the main API - Ensure clone of chain_channel_tx per necessary endpoint

        // TODO: Create tasks for the main runner to run - Ensure clone of chain_channel_tx per necessary task

        PluginData {
            plugin_id: "B1".to_owned(),
            route_definitions: Vec::new(),
            spawnable_tasks: Vec::new(),
        }
    }

    fn init() {}

    fn run() {}
}

// TODO: Flesh out the messages used between B1 client API and tasks
pub(crate) enum B1InternalMessage {}
