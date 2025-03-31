#[cfg(feature = "server")]
mod datastore;
#[cfg(feature = "server")]
mod response_error;

#[cfg(feature = "server")]
pub use datastore::*;
#[cfg(feature = "server")]
pub use response_error::ResponseError;
pub mod models;

use dioxus::prelude::*;
use futures::StreamExt;
use models::DashboardData;

#[cfg(feature = "server")]
use tokio::task::JoinError;

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
    let FromContext::<Datastore>(_datastore) = extract().await?;

    let _peer_list = Vec::<String>::new();

    todo!()
}

#[cfg(feature = "server")]
pub fn report_exit(
    task_name: &str,
    outcome: Result<Result<(), impl std::fmt::Debug + std::fmt::Display>, JoinError>,
) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} task failed to complete",
                task_name
            )
        }
    }
}
