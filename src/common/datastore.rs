#![cfg(feature = "server")]

use surrealdb::{engine::any::Any, Surreal};

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
}
