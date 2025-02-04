#![cfg(feature = "server")]
use std::sync::{mpsc, Arc};

use axum::{extract::FromRef, response::IntoResponse, routing::get};
use dioxus::prelude::*;
use dioxus_fullstack::ServeConfigBuilder;
use http::StatusCode;
use tokio::net::TcpListener;

use crate::{
    common::datastore::Datastore,
    configuration::Settings,
    protocols::{b1::B1Protocol, traits::Protocol, ChainMessage},
    ui::components::App,
};

#[tracing::instrument(skip_all)]
pub async fn run(settings: Settings) -> anyhow::Result<()> {
    tracing::info!("Connecting to database");
    let db = settings.database.get_db().await?;

    let axum_app_state = AppState {
        datastore: db.clone(),
    };

    // TODO: Replace this mess with `.context()` when released in future:
    // https://github.com/DioxusLabs/dioxus/pull/3483
    let dioxus_server_app_state = Arc::new(vec![Box::new({
        let local_state = axum_app_state.datastore.clone();
        move || Box::new(local_state.clone()) as Box<dyn std::any::Any>
    })
        as Box<dyn Fn() -> Box<dyn std::any::Any> + Send + Sync + 'static>]);

    tracing::debug!("APPSTATE: {:#?}", &axum_app_state);
    let (chain_message_tx, _chain_message_rx) = mpsc::channel::<ChainMessage>();

    tracing::info!("Launching web and API server");
    let app = axum::Router::new().serve_dioxus_application(
        ServeConfigBuilder::new().context_providers(server_only!(dioxus_server_app_state)),
        App,
    );

    let b1 = B1Protocol::initialize(db.clone(), settings.clone(), chain_message_tx.clone());

    tokio::spawn(async move { b1.run().await });

    let app = B1Protocol::register_routes(app).route("/health_check", get(health_check));
    let app = app.with_state(axum_app_state);

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

#[tracing::instrument(skip_all)]
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Healthy".to_owned())
}
