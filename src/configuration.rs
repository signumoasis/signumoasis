use config::{builder::DefaultState, ConfigBuilder, ConfigError};
use serde::Deserialize;
use surrealdb::{
    engine::any::{self, Any},
    opt::auth::Root,
    Surreal,
};

use crate::{
    chain::{ChainSettings, HistoricalMoment},
    common::Datastore,
    protocols::b1::B1Settings,
};

#[tracing::instrument(skip_all)]
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Get the base execution director
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    // Set the configuration file
    let configuration_file = "configuration.yml";

    // Create settings builder
    let settings = config::Config::builder();

    // INFO: Add defaults for settings
    let settings = HistoricalMoments::set_defaults(settings)?;
    #[cfg(feature = "server")]
    let settings = B1Settings::set_defaults(settings)?;

    // Override defaults and finalize settings import
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

/// Settings for the node.
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    #[cfg(feature = "server")]
    pub b1protocol: B1Settings,
    pub chain: ChainSettings,
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
            DEFINE TABLE IF NOT EXISTS b1_peer SCHEMALESS;

            DEFINE INDEX unique_announced_address ON b1_peer COLUMNS announced_address UNIQUE;

            DEFINE TABLE IF NOT EXISTS dashboard AS
                SELECT
                    (
                        SELECT VALUE count()
                        FROM ONLY b1_peer
                        GROUP ALL
                        LIMIT 1
                    ).count ?? 0 as b1_total_peers,
                    (
                        SELECT VALUE count()
                        FROM ONLY b1_peer
                        WHERE blacklist.until < time::now()
                        GROUP ALL LIMIT 1
                    ).count ?? 0 as b1_allowed_peers,
                    (
                        SELECT VALUE count()
                        FROM ONLY b1_peer
                        WHERE blacklist.until >= time::now()
                        GROUP ALL LIMIT 1
                    ).count ?? 0 as b1_blacklisted_peers
                FROM b1_peer
                GROUP ALL;
        "#,
    )
    .await?;

    Ok(db)
}

/// This settings struct represents any overrides for the historical moments. All values are optional.
#[derive(Clone, Debug, Deserialize)]
pub struct HistoricalMoments {
    pub genesis: HistoricalMoment,
    pub reward_recipient_enable: HistoricalMoment,
    pub digital_goods_store_enable: HistoricalMoment,
    pub automated_transaction_enable: HistoricalMoment,
    pub automated_transaction_fix_1: HistoricalMoment,
    pub automated_transaction_fix_2: HistoricalMoment,
    pub automated_transaction_fix_3: HistoricalMoment,
    pub pre_poc2: HistoricalMoment,
    pub poc2_enable: HistoricalMoment,
    pub sodium_enable: HistoricalMoment,
    pub signum_name_change: HistoricalMoment,
    pub poc_plus_enable: HistoricalMoment,
    pub speedway_enable: HistoricalMoment,
    pub smart_token_enable: HistoricalMoment,
    pub smart_fees_enable: HistoricalMoment,
    pub smart_ats_enable: HistoricalMoment,
    pub automated_transaction_fix_4: HistoricalMoment,
    pub distribution_fix_enable: HistoricalMoment,
    pub pk_freeze: HistoricalMoment,
    pub pk_freeze_2: HistoricalMoment,
    pub smart_alias_enable: HistoricalMoment,
    pub next_fork: HistoricalMoment,
}

// Defaults for HistoricalMoments
impl HistoricalMoments {
    #[tracing::instrument(skip_all)]
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
