use dioxus::prelude::*;

#[component]
pub fn B1PeerList() -> Element {
    let _peer_count = use_signal(|| 0u32);
    use_future(move || async move {
        //if let Ok(stream) = dashboard_stream().await {
        //    let mut stream = stream.into_inner();
        //    while let Some(Ok(total)) = stream.next().await {
        //        tracing::debug!("Component count value: {:?}", &total);
        //        *peer_count.write() = total;
        //    }
        //}
    });

    rsx! {
        p { id: "b1_peer_list", "B1 Peers" }
    }
}
