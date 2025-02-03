pub mod datastore;
pub mod models;

use dioxus::prelude::*;

#[cfg(any(feature = "server", feature = "web"))]
#[server(endpoint = "backend_list_peers")]
#[tracing::instrument(skip_all)]
pub async fn count_peers() -> Result<u32, ServerFnError> {
    use datastore::Datastore;

    use crate::protocols::b1::B1Datastore;

    tracing::debug!("Trying to get datastore from dioxus context");
    let FromContext::<Datastore>(datastore) = extract().await?;

    let datastore: B1Datastore = datastore.into();
    let peers = datastore.count_peers().await.map_err(ServerFnError::new)?;
    tracing::debug!("Peer count: {}", &peers);

    Ok(peers)
}
