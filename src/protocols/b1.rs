pub mod models;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::B1Datastore;
#[cfg(feature = "server")]
pub use server::B1Peer;
#[cfg(feature = "server")]
pub use server::B1Protocol;
#[cfg(feature = "server")]
pub use server::B1Settings;

#[cfg(feature = "server")]
use crate::common::datastore::Datastore;
use dioxus::prelude::*;

#[server(endpoint = "backend_list_peers")]
#[tracing::instrument(skip_all)]
pub async fn count_peers() -> Result<u32, ServerFnError> {
    use crate::protocols::b1::B1Datastore;

    tracing::debug!("Trying to get datastore from dioxus context");
    let FromContext::<Datastore>(datastore) = extract().await?;

    let datastore: B1Datastore = datastore.into();
    let peers = datastore.count_peers().await.map_err(ServerFnError::new)?;
    tracing::debug!("Peer count: {}", &peers);

    Ok(peers)
}
