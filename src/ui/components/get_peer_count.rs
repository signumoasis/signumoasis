use dioxus::prelude::*;

use futures::StreamExt;

use crate::{
    common::{dashboard_stream, models::DashboardData},
    ui::shleep,
};

#[component]
#[tracing::instrument()]
pub fn GetPeerCount() -> Element {
    let mut dashboard_data = use_signal(DashboardData::default);
    //use_server_future(|| async move {
    //    let client = Client::builder().build().unwrap();
    //    let url = "localhost:8080";
    //})?;
    use_future(move || async move {
        loop {
            tracing::debug!("Connecting to dashboard");
            // INFO: Loop forever to auto-reconnect if it dies
            if let Ok(stream) = dashboard_stream().await {
                let mut stream = stream.into_inner();
                while let Some(Ok(data)) = stream.next().await {
                    tracing::debug!("Component count value: {:?}", &data);
                    *dashboard_data.write() = data;
                }
            }
            shleep(1000).await;
        }
    });
    //let mut server_count_resource = use_server_future(count_peers)?;
    //let server_count = server_count_resource().unwrap().unwrap_or_default();

    let total_peers = dashboard_data().b1_total_peers;
    let allowed_peers = dashboard_data().b1_allowed_peers;
    let blacklisted_peers = dashboard_data().b1_blacklisted_peers;
    rsx! {
        p { id: "peer_count_display", "Total Peers: {total_peers}" }
        p { id: "allowed_peer_count_display", "Allowed Peers: {allowed_peers}" }
        p { id: "blacklisted_peer_count_display", "Blacklisted Peers: {blacklisted_peers}" }
    }
}
