use dioxus::prelude::*;

use futures::StreamExt;

use crate::common::{dashboard_stream, models::DashboardData};

#[component]
pub fn GetPeerCount() -> Element {
    let mut dashboard_data = use_signal(DashboardData::default);
    //use_server_future(|| async move {
    //    let client = Client::builder().build().unwrap();
    //    let url = "localhost:8080";
    //})?;
    use_future(move || async move {
        if let Ok(stream) = dashboard_stream().await {
            let mut stream = stream.into_inner();
            while let Some(Ok(data)) = stream.next().await {
                tracing::debug!("Component count value: {:?}", &data);
                *dashboard_data.write() = data;
            }
        }
    });
    //let mut server_count_resource = use_server_future(count_peers)?;
    //let server_count = server_count_resource().unwrap().unwrap_or_default();

    let x = dashboard_data().b1_total_peers;
    rsx! {
        p { id: "peer_count_display", "Peer Count: {x}" }
        //button {
        //    id: "peer_count_clicks",
        //    onclick: move |_| async move {
        //        tracing::debug!("Clicked get peer count button");
        //        server_count_resource.restart();
        //    },
        //    "Get Peer Count"
        //}
    }
}
