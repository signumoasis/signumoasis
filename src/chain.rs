use anyhow::Result;
use std::time::Duration;
use surrealdb::{engine::any::Any, Surreal};
use tracing::Instrument;
use uuid::Uuid;

use crate::common::Datastore;

pub mod models;

pub async fn run_chain_forever(datastore: ChainDatastore, settings: ChainSettings) -> Result<()> {
    tracing::info!("Starting Chain");
    loop {
        // Open the job-level span here so we also include the job_id in the error message if this result comes back Error.
        let span = tracing::span!(
            tracing::Level::INFO,
            "Chain Task",
            job_id = Uuid::new_v4().to_string()
        );
        let result = chain(datastore.clone(), settings.clone())
            .instrument(span)
            .await;
        if result.is_err() {
            tracing::error!("Error in peer finder: {:?}", result);
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

#[tracing::instrument(name = "Chain", skip_all)]
pub async fn chain(datastore: ChainDatastore, settings: ChainSettings) -> Result<()> {
    tracing::info!("CHAIN RUNNING");
    Ok(())
}

#[derive(Clone, Debug)]
pub struct ChainDatastore {
    db: Surreal<Any>,
}

impl ChainDatastore {}

impl From<Datastore> for ChainDatastore {
    fn from(value: Datastore) -> Self {
        Self {
            db: value.get_surreal_db(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChainSettings {}
