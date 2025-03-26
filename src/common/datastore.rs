#![cfg(feature = "server")]

use anyhow::Result;
use surrealdb::{engine::any::Any, method::Stream, Surreal};

use super::models::DashboardData;

#[derive(Clone, Debug)]
pub struct Datastore {
    db: Surreal<Any>,
}

impl Datastore {
    /// Instantiates a new Datastore with a provided [`Surreal`]<[`Any`]>.
    pub fn new(db: Surreal<Any>) -> Self {
        Self { db }
    }

    /// Returns a clone of the raw Surrealdb handle for use with custom queries.
    ///
    /// For when the Datastore class just won't do.
    pub fn get_surreal_db(&self) -> Surreal<Any> {
        self.db.clone()
    }

    pub async fn get_dashboard_stream(&self) -> Result<Stream<Vec<DashboardData>>> {
        let response = self.db.select("dashboard").live().await?;
        Ok(response)
    }
}
