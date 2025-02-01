use config::{builder::DefaultState, ConfigBuilder, ConfigError};
use serde::Deserialize;
use surrealdb::{
    engine::any::{self, Any},
    opt::auth::Root,
    Surreal,
};

use crate::{common::datastore::Datastore, protocols::b1::b1_configuration::B1Settings};

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Get the base execution director
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    // Set the configuration file
    let configuration_file = "configuration.yml";

    let settings = config::Config::builder();

    let settings = HistoricalMoments::set_defaults(settings)?;

    #[cfg(feature = "server")]
    let settings = B1Settings::set_defaults(settings)?;

    let settings = settings
        //add values from a file
        .add_source(config::File::from(base_path.join(configuration_file)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    let settings: Result<Settings, config::ConfigError> = settings.try_deserialize();
    tracing::debug!("Settings values: {:#?}", &settings);
    settings
}

#[allow(dead_code)]
trait ConfigBuilderExtensions {
    fn add_defaults(self) -> Result<Self, config::ConfigError>
    where
        Self: Sized;
}

/// Settings for the node.
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    #[cfg(feature = "server")]
    pub b1protocol: B1Settings,
    pub database: DatabaseSettings,
    pub historical_moments: HistoricalMoments,
    pub node: NodeSettings,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NodeSettings {
    pub cash_back_id: String,
    pub network: String,
}

/// Database settings.
#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseSettings {
    pub filename: String,
}

impl DatabaseSettings {
    #[tracing::instrument(skip_all)]
    pub async fn get_db(&self) -> Result<Datastore, anyhow::Error> {
        tracing::info!("Getting database");
        let db = any::connect(&self.filename).await?;
        // let db = any::connect(format!("speedb:{}", self.filename)).await?;

        if !self.filename.starts_with("surrealkv:")
            && !self.filename.starts_with("rocksdb:")
            && !&self.filename.starts_with("mem:")
        {
            db.signin(Root {
                username: "signum",
                password: "signum",
            })
            .await?;
        }

        let namespace = "signum";
        let database = "signum";
        db.use_ns(namespace).use_db(database).await?;

        tracing::info!(
            "Opened surrealdb file db {}, using namespace {} and database {}",
            &self.filename,
            namespace,
            database
        );

        tracing::info!("Initializing database");
        let db = initialize_database(db).await?;

        Ok(Datastore::new(db))
    }
}

#[tracing::instrument(skip_all)]
async fn initialize_database(db: Surreal<Any>) -> Result<Surreal<Any>, anyhow::Error> {
    tracing::info!("Defining unique index on announced_address field");
    db.query(
        r#"
            DEFINE INDEX unique_announced_address ON peer COLUMNS announced_address UNIQUE
        "#,
    )
    .await?;

    Ok(db)
}

/// This settings struct represents any overrides for the historical moments. All values are optional.
#[derive(Clone, Debug, Deserialize)]
pub struct HistoricalMoments {
    pub genesis: u32,
    pub reward_recipient_enable: u32,
    pub digital_goods_store_enable: u32,
    pub automated_transaction_enable: u32,
    pub automated_transaction_fix_1: u32,
    pub automated_transaction_fix_2: u32,
    pub automated_transaction_fix_3: u32,
    pub pre_poc2: u32,
    pub poc2_enable: u32,
    pub sodium_enable: u32,
    pub signum_name_change: u32,
    pub poc_plus_enable: u32,
    pub speedway_enable: u32,
    pub smart_token_enable: u32,
    pub smart_fees_enable: u32,
    pub smart_ats_enable: u32,
    pub automated_transaction_fix_4: u32,
    pub distribution_fix_enable: u32,
    pub pk_freeze: u32,
    pub pk_freeze_2: u32,
    pub smart_alias_enable: u32,
    pub next_fork: u32,
}

// Defaults for HistoricalMoments
impl HistoricalMoments {
    fn set_defaults(
        builder: ConfigBuilder<DefaultState>,
    ) -> Result<ConfigBuilder<DefaultState>, ConfigError> {
        builder
            .set_default("historical_moments.genesis", 0)?
            .set_default("historical_moments.reward_recipient_enable", 6_500)?
            .set_default("historical_moments.digital_goods_store_enable", 11_800)?
            .set_default("historical_moments.automated_transaction_enable", 49_200)?
            .set_default("historical_moments.automated_transaction_fix_1", 67_000)?
            .set_default("historical_moments.automated_transaction_fix_2", 92_000)?
            .set_default("historical_moments.automated_transaction_fix_3", 255_000)?
            .set_default("historical_moments.pre_poc2", 500_000)?
            .set_default("historical_moments.poc2_enable", 502_000)?
            .set_default("historical_moments.sodium_enable", 765_000)?
            .set_default("historical_moments.signum_name_change", 875_500)?
            .set_default("historical_moments.poc_plus_enable", 878_000)?
            .set_default("historical_moments.speedway_enable", 941_100)?
            .set_default("historical_moments.smart_token_enable", 1_029_000)?
            .set_default("historical_moments.smart_fees_enable", 1_029_000)?
            .set_default("historical_moments.smart_ats_enable", 1_029_000)?
            .set_default("historical_moments.automated_transaction_fix_4", 1_051_900)?
            .set_default("historical_moments.distribution_fix_enable", 1_051_900)?
            .set_default("historical_moments.pk_freeze", 1_099_400)?
            .set_default("historical_moments.pk_freeze_2", 1_150_000)?
            .set_default("historical_moments.smart_alias_enable", 1_150_000)?
            .set_default("historical_moments.next_fork", u32::MAX)
    }
}
