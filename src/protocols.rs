#![cfg(feature = "server")]
use std::{
    fmt::{Debug, Display},
    sync::mpsc,
};

use axum::Router;
use tokio::task::JoinError;

use crate::{common::datastore::Datastore, configuration::Settings};

pub mod b1;

#[allow(async_fn_in_trait)]
pub trait Protocol {
    async fn run(&self);
    fn initialize(
        datastore: Datastore,
        settings: Settings,
        chain_message_tx: mpsc::Sender<ChainMessage>,
    ) -> Self;
    fn register_routes(router: Router) -> Router;
}

/// Messages to the Chain
pub enum ChainMessage {
    RegisterPlugin(mpsc::Sender<PluginMessage>),
    GetBlock,
    ProcessBlocks,
}

/// Messages to the Plugins
pub enum PluginMessage {
    GetMoreBlocks,
    BadBlock,
}

pub fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} task failed to complete",
                task_name
            )
        }
    }
}
