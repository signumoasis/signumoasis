#![cfg(feature = "server")]
use std::sync::mpsc;

use dioxus::prelude::*;
use dioxus_fullstack::ServeConfigBuilder;
use tokio::net::TcpListener;

use crate::{
    configuration::Settings,
    protocols::{b1::B1Protocol, ChainMessage, Protocol},
    ui::components::App,
};

#[tracing::instrument(skip_all)]
pub async fn run(settings: Settings) -> anyhow::Result<()> {
    tracing::info!("Connecting to database");
    let db = settings.database.get_db().await?;

    let (chain_message_tx, _chain_message_rx) = mpsc::channel::<ChainMessage>();

    tracing::info!("Launching web and API server");
    let app = axum::Router::new().serve_dioxus_application(ServeConfigBuilder::new(), App);

    let b1 = B1Protocol::initialize(db.clone(), settings.clone(), chain_message_tx.clone());
    let app = B1Protocol::register_routes(app);

    tokio::spawn(async move { b1.run().await });

    let socket_address = dioxus_cli_config::fullstack_address_or_localhost();
    let listener = TcpListener::bind(&socket_address).await.unwrap();

    tracing::info!("Listening on {}", socket_address);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
