use std::sync::mpsc;

use crate::{common::datastore::Datastore, server_stuff::PluginData};

pub mod b1;

pub trait Protocol {
    /// Creates and returns a [`PluginData`] containing the items the plugin needs the host to run.
    fn register(chain_channel_tx: mpsc::Sender<ChainMessage>, db: Datastore) -> PluginData;
    fn init() {}
    fn run() {}
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
