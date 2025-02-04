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
use futures::StreamExt;

#[server(endpoint = "backend_list_peers", output = server_fn::codec::StreamingJson)]
#[tracing::instrument(skip_all)]
pub async fn count_peers() -> Result<server_fn::codec::JsonStream<u32>, ServerFnError> {
    use crate::protocols::b1::B1Datastore;

    tracing::trace!("Trying to get datastore from dioxus context");
    let FromContext::<Datastore>(datastore) = extract().await?;

    let datastore: B1Datastore = datastore.into();
    let peers = datastore.count_peers().await.map_err(ServerFnError::new)?;
    tracing::trace!("Got surreal stream");
    let stream = server_fn::codec::JsonStream::<u32>::new(peers.map(|n| {
        //tracing::debug!("Notification Result: {:#?}", &n);
        match n {
            Ok(notification) => {
                //tracing::debug!("Notification: {:#?}", &notification);
                let result = notification.data.number_of_peers;
                tracing::debug!("Notification: {:#?}", &result);
                Ok(result)
            }
            Err(e) => {
                tracing::debug!("unable to get count: {:#?}", &e);
                Err(ServerFnError::new("unable to get count"))
            }
        }
    }));
    Ok(stream)
}
