#[cfg(feature = "server")]
mod datastore;
#[cfg(feature = "server")]
mod response_error;

#[cfg(feature = "server")]
pub use datastore::Datastore;
#[cfg(feature = "server")]
pub use response_error::ResponseError;
pub mod models;

use dioxus::prelude::*;
use futures::StreamExt;
use models::DashboardData;

#[server(endpoint = "dashboard", output = server_fn::codec::StreamingJson)]
#[tracing::instrument(skip_all)]
pub async fn dashboard_stream() -> Result<server_fn::codec::JsonStream<DashboardData>, ServerFnError>
{
    tracing::trace!("Trying to get datastore from dioxus");
    let FromContext::<Datastore>(datastore) = extract().await?;

    let dashboard = datastore
        .get_dashboard_stream()
        .await
        .map_err(ServerFnError::new)?;

    tracing::trace!("Got surreal stream for dashboard");

    let stream = server_fn::codec::JsonStream::<DashboardData>::new(dashboard.map(|n| {
        tracing::debug!("Result<Notification>: {:#?}", &n);
        match n {
            Ok(notification) => {
                tracing::trace!("Notification: {:#?}", &notification);
                let result = notification.data;
                tracing::debug!("Notification value: {:#?}", &result);
                Ok(result)
            }
            Err(e) => {
                tracing::debug!("unable to get dashboard: {:#?}", &e);
                Err(ServerFnError::new("unable to get dashboard"))
            }
        }
    }));
    Ok(stream)
}

#[server(endpoint = "peer_list")]
#[tracing::instrument(skip_all)]
pub async fn peers_list() -> Result<Vec<String>, ServerFnError> {
    tracing::trace!("Trying to get datastore from dioxus");
    let FromContext::<Datastore>(datastore) = extract().await?;

    let peer_list = Vec::<String>::new();

    todo!()
}
