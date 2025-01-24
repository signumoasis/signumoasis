use dioxus::prelude::*;

#[component]
pub fn ClientClickCounter() -> Element {
    let mut count = use_signal(|| 5);
    rsx! {
        p { id: "count_display", "{count}" }
        button {
            id: "count_clicks",
            onclick: move |_| {
                tracing::debug!("Clicked client count button");
               *count.write() += 1;
            },
            "CLIENT - CLICK ME!"
        }
    }
}
