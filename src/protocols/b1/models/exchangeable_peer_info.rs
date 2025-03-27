use serde::{Deserialize, Serialize};

use crate::common::models::PeerAddress;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeablePeerInfo {
    pub announced_address: Option<PeerAddress>,
    pub application: String,
    pub version: String,
    pub platform: Option<String>,
    pub share_address: bool,
    pub network_name: String,
}
