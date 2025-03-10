#![cfg(feature = "server")]
use std::sync::mpsc;

use axum::{extract::FromRef, Router};

use crate::{common::datastore::Datastore, configuration::Settings};

use super::ChainMessage;

#[allow(async_fn_in_trait)]
pub trait Protocol {
    async fn run(&self);
    fn initialize(
        datastore: Datastore,
        settings: Settings,
        chain_message_tx: mpsc::Sender<ChainMessage>,
    ) -> Self;
    fn register_routes(&self, router: Router) -> Router;
}
