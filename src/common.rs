pub mod datastore;
pub mod models;

use dioxus::prelude::*;

#[cfg(any(feature = "server", feature = "web"))]
#[server(endpoint = "backend_list_peers")]
#[tracing::instrument(skip_all)]
pub async fn count_peers() -> Result<u32, ServerFnError> {
    tracing::debug!("In count_peers");
    use datastore::Datastore;

    use crate::{protocols::b1::B1Datastore, server::AppState};

    tracing::debug!("Get datastore");
    let FromContext::<AppState>(datastore) = extract().await?;

    tracing::debug!("Got datastore {:?}", &datastore);

    let datastore: B1Datastore = datastore.datastore.into();
    tracing::debug!("Getting peer count");
    let peers = datastore.count_peers().await.map_err(ServerFnError::new)?;
    Ok(peers)
}
