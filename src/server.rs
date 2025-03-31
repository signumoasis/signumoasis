#![cfg(feature = "server")]
use std::sync::{mpsc, Arc};

use axum::{extract::FromRef, response::IntoResponse, routing::get};
use dioxus::prelude::*;
use dioxus_fullstack::ServeConfigBuilder;
use http::StatusCode;
use tokio::net::TcpListener;

use crate::{
    chain::{run_chain_forever, Chain},
    common::{report_exit, Datastore},
    configuration::Settings,
    protocols::{b1::B1Protocol, traits::Protocol, ChainMessage},
    ui::components::App,
};

#[tracing::instrument(name = "Main Server", skip_all)]
pub async fn run(settings: Settings) -> anyhow::Result<()> {
    tracing::info!("Connecting to database");
    let db = settings.database.get_db().await?;

    let axum_app_state = AppState {
        datastore: db.clone(),
        settings: settings.clone(),
    };

    // TODO: Replace this mess with `.context()` when released in future:
    // https://github.com/DioxusLabs/dioxus/pull/3483
    // Datastore context
    let dioxus_server_app_state_datastore = Arc::new(vec![Box::new({
        let local_state = axum_app_state.datastore.clone();
        move || Box::new(local_state.clone()) as Box<dyn std::any::Any>
    })
        as Box<dyn Fn() -> Box<dyn std::any::Any> + Send + Sync + 'static>]);

    // TODO: Replace this mess with `.context()` when released in future:
    // https://github.com/DioxusLabs/dioxus/pull/3483
    // Settings context
    let dioxus_server_app_state_settings = Arc::new(vec![Box::new({
        let local_state = axum_app_state.settings.clone();
        move || Box::new(local_state.clone()) as Box<dyn std::any::Any>
    })
        as Box<dyn Fn() -> Box<dyn std::any::Any> + Send + Sync + 'static>]);

    tracing::debug!("APPSTATE: {:#?}", &axum_app_state);
    let (chain_message_tx, _chain_message_rx) = mpsc::channel::<ChainMessage>();

    tracing::info!("Launching web and API server");
    let app = axum::Router::new().serve_dioxus_application(
        ServeConfigBuilder::new()
            .context_providers(server_only!(dioxus_server_app_state_settings))
            .context_providers(server_only!(dioxus_server_app_state_datastore)),
        App,
    );

    let b1 = B1Protocol::initialize(db.clone(), settings.clone(), chain_message_tx.clone());

    let app = app.route("/health_check", get(health_check));
    let app = app.with_state(axum_app_state);
    let app = b1.register_routes(app);

    tokio::spawn(async move { b1.run().await });

    let socket_address = dioxus_cli_config::fullstack_address_or_localhost();
    let listener = TcpListener::bind(&socket_address).await.unwrap();

    tracing::info!("Listening on {}", socket_address);

    // TODO: Set up a tokio_select task branch loop here
    let chain = Chain::new(db.into(), settings.clone().into());
    let chain_task = tokio::spawn(run_chain_forever(chain));

    // Select on all the tasks to report closure status
    tokio::select! {
        //o = block_downloader_task=> report_exit("Block Downloader", o),
        o = chain_task => report_exit("Chain task exited", o),
    };

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub datastore: Datastore,
    pub settings: Settings,
}

impl FromRef<AppState> for Datastore {
    fn from_ref(input: &AppState) -> Self {
        input.datastore.clone()
    }
}

impl FromRef<AppState> for Settings {
    fn from_ref(input: &AppState) -> Self {
        input.settings.clone()
    }
}

#[tracing::instrument(skip_all)]
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Healthy".to_owned())
}
