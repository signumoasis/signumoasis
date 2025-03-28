use config::{builder::DefaultState, ConfigBuilder, ConfigError};
use serde::Deserialize;

use crate::common::models::PeerAddress;

/// Settings for the signum-style API.
#[derive(Clone, Debug, Deserialize)]
pub struct B1Settings {
    ///The base url for the B1 protocol - set to your domain name or to 'localhost'
    pub base_url: String,
    /// The IP address this the B1 protocol should listen on - 0.0.0.0 for everything, 127.0.0.1 for localhost
    /// or a specific IP address if you desire a specific network adapter
    pub listen_address: String,
    /// The port on which the B1 protocol should listen
    pub listen_port: u16,
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

impl B1Settings {
    /// Set defaults for the [`B1Settings`].
    pub fn set_defaults(
        builder: ConfigBuilder<DefaultState>,
    ) -> Result<ConfigBuilder<DefaultState>, ConfigError> {
        builder
            .set_default("b1protocol.listen_port", "8123")?
            .set_default("b1protocol.bootstrap_peers", {
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
            .set_default("b1protocol.my_address", String::new())?
            .set_default("b1protocol.platform", String::new())?
            .set_default("b1protocol.share_address", true)?
            .set_default("b1protocol.network_name", "Signum".to_string())?
            .set_default("b1protocol.snr_reward_address", String::new())
    }
}
