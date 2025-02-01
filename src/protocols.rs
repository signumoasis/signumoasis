#![cfg(feature = "server")]
use std::sync::mpsc;

use axum::Router;

use crate::{common::datastore::Datastore, configuration::Settings};

pub mod b1;

pub trait Protocol {
    fn register_settings();
    fn initialize(settings: Settings, router: Router, chain_message_tx: mpsc::Sender<ChainMessage>);
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
