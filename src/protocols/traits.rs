#![cfg(feature = "server")]
use std::sync::mpsc;

use axum::Router;

use crate::{common::Datastore, configuration::Settings};

use super::ChainMessage;

#[allow(async_fn_in_trait)]
pub trait Protocol {
    async fn run(&self) -> Result<(), anyhow::Error>;
    fn initialize(
        datastore: Datastore,
        settings: Settings,
        chain_message_tx: mpsc::Sender<ChainMessage>,
    ) -> Self;
    fn register_routes(&self, router: Router) -> Router;
}
