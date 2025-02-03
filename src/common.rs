pub mod datastore;
pub mod models;

use dioxus::prelude::*;

#[cfg(any(feature = "server", feature = "web"))]
#[server(endpoint = "backend_list_peers")]
#[tracing::instrument(skip_all)]
pub async fn count_peers() -> Result<u32, ServerFnError> {
    tracing::debug!("In count_peers");
    use datastore::Datastore;
    use dioxus_elements::data;

    use crate::protocols::b1::B1Datastore;

    tracing::debug!("Get datastore");
    //let FromContext::<Datastore>(datastore) = extract().await?;
    let x = extract().await;
    let datastore = match x {
        Ok(FromContext::<Datastore>(ds)) => {
            tracing::debug!("Got datastore {:#?}", &ds);
            ds
        }
        Err(e) => {
            tracing::debug!("Can't get datastore: {:#?}", e);
            return Err(ServerFnError::new(e));
        }
    };

    tracing::debug!("Got datastore {:?}", &datastore);

    let datastore: B1Datastore = datastore.into();
    tracing::debug!("Getting peer count");
    let peers = datastore.count_peers().await.map_err(ServerFnError::new)?;
    Ok(peers)
}
