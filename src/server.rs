#![cfg(feature = "server")]
use dioxus::prelude::*;
use dioxus_fullstack::ServeConfigBuilder;
use tokio::net::TcpListener;
use tracing::info;

use crate::ui::components::App;

pub async fn setup() {
    let app = axum::Router::new().serve_dioxus_application(ServeConfigBuilder::new(), App);

    let socket_address = dioxus_cli_config::fullstack_address_or_localhost();
    let listener = TcpListener::bind(&socket_address).await.unwrap();

    info!("Listening on {}", socket_address);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    tokio::select! {
        // TODO: Don't catch ctrl-c here. Tokio is not the main fn so won't set up
        // a catch. Catch it manually and send a signal to this instead via channel.
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Received shutdown signal. Exiting web server.")
        }
    }
}
