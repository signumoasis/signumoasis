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

    #[tracing::instrument(skip_all)]
    pub async fn get_dashboard_stream(&self) -> Result<Stream<Vec<DashboardData>>> {
        let response = self.db.select("dashboard").live().await?;
        tracing::trace!("Live Selected: {:#?}", &response);
        Ok(response)
    }
}

/// Represents a Datastore error.
#[derive(thiserror::Error)]
pub enum DatastoreError {
    /// An unexpected error. Will contain an [`anyhow::Error`] with additional details.
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

//impl ResponseError for DatastoreError {}

impl std::fmt::Debug for DatastoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error_chain_fmt(self, f)
    }
}
