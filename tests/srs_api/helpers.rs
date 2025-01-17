use once_cell::sync::Lazy;
use signum_node_rs::{
    configuration::get_configuration,
    models::datastore::Datastore,
    srs_protocol::api::SrsApiApplication,
    telemetry::{get_subscriber, init_subscriber},
};

// Ensure that `tracing` stack is only initialized once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    // Randomize config to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("failed to read configuration");
        c.database.filename = "mem://".to_string();
        c.srs_api.listen_port = 0;

        // Set up config for testing
        c.p2p.my_address = "http://localhost".to_string();
        c.p2p.platform = "Test".to_string();
        c.p2p.share_address = true;
        c.p2p.network_name = "TEST".to_string();
        c.p2p.snr_reward_address = "SNRADDRESS".to_string();
        c
    };

    // Configure and migrate the database
    let datastore = configuration.database.get_db().await.unwrap();

    // Launch the application as a background task
    let application = SrsApiApplication::build(configuration.clone(), datastore.clone())
        .await
        .expect("failed to build application");
    let application_port = application.port();

    tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        address: format!("http://localhost:{}", application_port),
        _datastore: datastore,
        _port: application_port,
        _api_client: client,
    }
}

pub struct TestApp {
    pub address: String,
    pub _datastore: Datastore,
    pub _port: u16,
    pub _api_client: reqwest::Client,
}

impl TestApp {}

// async fn configure_database(configuration: &DatabaseSettings) -> Result<SqlitePool, anyhow::Error> {
//     // Create in-memory database and migrate it
//     let connection_pool = SqlitePool::connect_with(configuration.get_writable_db()?)
//         .await
//         .expect("failed to connect to in-memory database");
//     sqlx::migrate!("./migrations")
//         .run(&connection_pool)
//         .await
//         .expect("failed to migrate the in-memory database");
//
//     Ok(connection_pool)
// }
