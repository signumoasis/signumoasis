use serde::{Deserialize, Serialize};

use crate::common::models::PeerAddress;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeablePeerInfo {
    pub announced_address: Option<PeerAddress>,
    pub application: Option<String>,
    pub version: Option<String>,
    pub platform: Option<String>,
    pub share_address: Option<bool>,
    pub network_name: Option<String>,
}
