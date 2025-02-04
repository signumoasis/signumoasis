use config::{builder::DefaultState, ConfigBuilder, ConfigError};
use serde::Deserialize;

use crate::common::models::PeerAddress;

/// Settings for the signum-style API.
#[derive(Clone, Debug, Deserialize)]
pub struct B1Settings {
    pub base_url: String,
    pub listen_address: String,
    pub listen_port: u16,
    pub p2p: PeerToPeerSettings,
}

impl B1Settings {
    pub(crate) fn set_defaults(
        builder: ConfigBuilder<DefaultState>,
    ) -> Result<ConfigBuilder<DefaultState>, ConfigError> {
        PeerToPeerSettings::set_defaults(builder)
    }
}

/// Peer to Peer settings.
#[derive(Clone, Debug, Deserialize)]
pub struct PeerToPeerSettings {
    /// Peer addresses to use if none are in the database already.
    pub bootstrap_peers: Vec<PeerAddress>,
    /// Address that peers should attempt to connect to.
    pub my_address: String,
    /// A string indicating the platform in use. Often set to a signum address for SNR rewards.
    pub platform: String,
    /// Whether or not peers should pass along your address to their own peers.
    pub share_address: bool,
    /// The name of the network to which this node is connecting.
    pub network_name: String,
    /// The address to which SNR awards should be paid. Currently unused on the network.
    pub snr_reward_address: String,
}

impl PeerToPeerSettings {
    /// Set defaults for the [`PeerToPeerSettings`].
    fn set_defaults(
        builder: ConfigBuilder<DefaultState>,
    ) -> Result<ConfigBuilder<DefaultState>, ConfigError> {
        builder
            .set_default("b1protocol.p2p.bootstrap_peers", {
                vec![
                    "australia.signum.network:8123",
                    "brazil.signum.network:8123",
                    "canada.signum.network:8123",
                    "europe.signum.network:8123",
                    "europe1.signum.network:8123",
                    "europe2.signum.network:8123",
                    "europe3.signum.network:8123",
                    "latam.signum.network:8123",
                    "singapore.signum.network:8123",
                    "ru.signum.network:8123",
                    "us-central.signum.network:8123",
                    "us-east.signum.network:8123",
                ]
            })?
            //TODO: Figure out a way to get external IP and populate it
            .set_default("b1protocol.p2p.my_address", String::new())?
            .set_default("b1protocol.p2p.platform", String::new())?
            .set_default("b1protocol.p2p.share_address", true)?
            .set_default("b1protocol.p2p.network_name", "Signum".to_string())?
            .set_default("b1protocol.p2p.snr_reward_address", String::new())
    }
}
