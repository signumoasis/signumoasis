use axum::{routing::IntoMakeService, serve::Serve, Router};
use tokio::net::TcpListener;

use crate::protocols::b1::{B1Datastore, B1Settings};

use super::signum_api_handler;

pub type AppServer = Serve<IntoMakeService<Router>, Router>;

pub struct B1ApiApplication {
    port: u16,
    server: AppServer,
}

impl B1ApiApplication {
    pub async fn build(
        configuration: B1Settings,
        database: B1Datastore,
    ) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.listen_address.clone(),
            configuration.listen_port.clone()
        );

        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, database, configuration.clone()).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    #[tracing::instrument(name = "B1 API Server Runner", skip_all)]
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        tracing::info!("Starting B1 API Application");
        self.server.await
    }
}

// fn get_connection_pool(configuration: &DatabaseSettings) -> Result<SqlitePool, anyhow::Error> {
//     Ok(SqlitePoolOptions::new().connect_lazy_with(configuration.get_writable_db()?))
// }

pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    db: B1Datastore,
    settings: B1Settings,
) -> Result<AppServer, anyhow::Error> {
    let app = Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }));

    let server = axum::serve(listener, app.into_make_service());

    Ok(server)
}
