use serde::{Deserialize, Serialize};

use crate::common::models::PeerAddress;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FullPeerInfo {
    pub announced_address: Option<PeerAddress>,
    pub application: Option<String>,
    pub attempts_since_last_seen: Option<u32>,
    pub blacklist: Option<BlacklistInfo>,
    pub last_seen: Option<String>,
    pub network_name: Option<String>,
    pub platform: Option<String>,
    pub share_address: Option<bool>,
    pub version: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BlacklistInfo {
    pub count: u32,
    pub until: String,
}
