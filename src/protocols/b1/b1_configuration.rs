use std::str::FromStr;

use serde::Deserialize;

use crate::common::models::PeerAddress;

/// Settings for the signum-style API.
#[derive(Clone, Debug, Deserialize)]
pub struct SrsApiSettings {
    pub base_url: String,
    pub listen_address: String,
    pub listen_port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NodeSettings {
    pub cash_back_id: String,
    pub network: String,
}

/// Peer to Peer settings.
#[derive(Clone, Debug, Deserialize)]
pub struct PeerToPeerSettings {
    /// Peer addresses to use if none are in the database already.
    #[serde(default = "PeerToPeerSettings::default_value_bootstrap_peers")]
    pub bootstrap_peers: Vec<PeerAddress>,
    /// Address that peers should attempt to connect to.
    #[serde(default = "PeerToPeerSettings::default_value_my_address")]
    pub my_address: String,
    /// A string indicating the platform in use. Often set to a signum address for SNR rewards.
    #[serde(default = "PeerToPeerSettings::default_value_platform")]
    pub platform: String,
    /// Whether or not peers should pass along your address to their own peers.
    #[serde(default = "PeerToPeerSettings::default_value_share_address")]
    pub share_address: bool,
    /// The name of the network to which this node is connecting.
    #[serde(default = "PeerToPeerSettings::default_value_network_name")]
    pub network_name: String,
    /// The address to which SNR awards should be paid. Currently unused on the network.
    #[serde(default = "PeerToPeerSettings::default_value_snr_reward_address")]
    pub snr_reward_address: String,
}

// Defaults for PeerToPeerSettings
impl PeerToPeerSettings {
    fn default_value_bootstrap_peers() -> Vec<PeerAddress> {
        vec![
            PeerAddress::from_str("australia.signum.network:8123")
                .expect("could not parse bootstrap ip address `australia.signum.network:8123`"),
            PeerAddress::from_str("brazil.signum.network:8123")
                .expect("could not parse bootstrap ip address `brazil.signum.network:8123`"),
            PeerAddress::from_str("canada.signum.network:8123")
                .expect("could not parse bootstrap ip address `canada.signum.network:8123`"),
            PeerAddress::from_str("europe.signum.network:8123")
                .expect("could not parse bootstrap ip address `europe.signum.network:8123`"),
            PeerAddress::from_str("europe1.signum.network:8123")
                .expect("could not parse bootstrap ip address `europe1.signum.network:8123`"),
            PeerAddress::from_str("europe2.signum.network:8123")
                .expect("could not parse bootstrap ip address `europe2.signum.network:8123`"),
            PeerAddress::from_str("europe3.signum.network:8123")
                .expect("could not parse bootstrap ip address `europe3.signum.network:8123`"),
            PeerAddress::from_str("latam.signum.network:8123")
                .expect("could not parse bootstrap ip address `latam.signum.network:8123`"),
            PeerAddress::from_str("singapore.signum.network:8123")
                .expect("could not parse bootstrap ip address `singapore.signum.network:8123`"),
            PeerAddress::from_str("ru.signum.network:8123")
                .expect("could not parse bootstrap ip address `ru.signum.network:8123`"),
            PeerAddress::from_str("us-central.signum.network:8123")
                .expect("could not parse bootstrap ip address `us-central.signum.network:8123`"),
            PeerAddress::from_str("us-east.signum.network:8123")
                .expect("could not parse bootstrap ip address `us-east.signum.network:8123`"),
        ]
    }

    fn default_value_my_address() -> String {
        //TODO: Figure out a way to get external IP and populate it
        String::new()
    }

    fn default_value_platform() -> String {
        String::new()
    }

    fn default_value_share_address() -> bool {
        true
    }

    fn default_value_network_name() -> String {
        "Signum".to_string()
    }

    fn default_value_snr_reward_address() -> String {
        String::new()
    }
}
