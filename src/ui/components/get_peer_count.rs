use dioxus::prelude::*;

use futures::StreamExt;
use reqwest::Client;

use crate::protocols::b1::count_peers;

#[component]
pub fn GetPeerCount() -> Element {
    let mut peer_count = use_signal(|| 0u32);
    use_server_future(|| async move {
        let client = Client::builder().build().unwrap();
        let url = "localhost:8080";
    })?;
    use_future(move || async move {
        if let Ok(stream) = count_peers().await {
            let mut stream = stream.into_inner();
            while let Some(Ok(total)) = stream.next().await {
                tracing::debug!("Component count value: {:?}", &total);
                *peer_count.write() = total;
            }
        }
    });
    //let mut server_count_resource = use_server_future(count_peers)?;
    //let server_count = server_count_resource().unwrap().unwrap_or_default();

    rsx! {
        p { id: "peer_count_display", "Peer Count: {peer_count}" }
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
