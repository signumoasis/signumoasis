#![cfg(feature = "server")]
use std::sync::mpsc;

use axum::{extract::FromRef, response::IntoResponse, routing::get};
use dioxus::prelude::*;
use dioxus_fullstack::ServeConfigBuilder;
use http::StatusCode;
use tokio::net::TcpListener;

use crate::{
    common::datastore::Datastore,
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

    tokio::spawn(async move { b1.run().await });

    let appstate = AppState {
        datastore: db.clone(),
    };
    let app = B1Protocol::register_routes(app).route("/health_check", get(health_check));
    let app = app.with_state(appstate);

    let socket_address = dioxus_cli_config::fullstack_address_or_localhost();
    let listener = TcpListener::bind(&socket_address).await.unwrap();

    tracing::info!("Listening on {}", socket_address);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub datastore: Datastore,
}

impl FromRef<AppState> for Datastore {
    fn from_ref(input: &AppState) -> Self {
        input.datastore.clone()
    }
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Healthy".to_owned())
}
