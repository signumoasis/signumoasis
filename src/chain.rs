use anyhow::Result;
use serde::Deserialize;
use std::time::Duration;
use surrealdb::{engine::any::Any, Surreal};
use tracing::Instrument;
use uuid::Uuid;

use crate::{
    common::Datastore,
    configuration::{HistoricalMoments, Settings},
};

mod flux_capacitor;
pub use flux_capacitor::*;

pub mod models;

pub struct Chain {
    datastore: ChainDatastore,
    settings: ChainSettings,
    historical_moments: HistoricalMoments,
    flux_capacitor: FluxCapacitor,
}
impl Chain {
    pub fn new(
        datastore: ChainDatastore,
        settings: ChainSettings,
        historical_moments: HistoricalMoments,
    ) -> Self {
        Self {
            datastore,
            flux_capacitor: FluxCapacitor::new((), historical_moments.clone()),
            historical_moments,
            settings,
        }
    }

    #[tracing::instrument(name = "Chain Process", skip_all)]
    pub async fn process(&self) -> Result<()> {
        tracing::info!("CHAIN RUNNING");
        Ok(())
    }
}

pub async fn run_chain_forever(chain: Chain) -> Result<()> {
    tracing::info!("Starting Chain");
    loop {
        // Open the job-level span here so we also include the job_id in the error message if this result comes back Error.
        let span = tracing::span!(
            tracing::Level::INFO,
            "Chain Task",
            job_id = Uuid::new_v4().to_string()
        );
        let result = chain.process().instrument(span).await;
        if result.is_err() {
            tracing::error!("Error in peer finder: {:?}", result);
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

#[derive(Clone, Debug)]
pub struct ChainDatastore {
    db: Surreal<Any>,
}

impl ChainDatastore {
    //pub async fn store_block(&self, _block: Block) {}
    pub async fn get_blocks_from_height(&self, _height: u64, _number_of_blocks: u64) {}
    pub async fn get_block_by_id(&self, _block_id: u64) {}
}

impl From<Datastore> for ChainDatastore {
    fn from(value: Datastore) -> Self {
        Self {
            db: value.get_surreal_db(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChainSettings {
    placeholder: String,
}

impl From<Settings> for ChainSettings {
    fn from(value: Settings) -> Self {
        value.chain
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct HistoricalMoment(u32);
