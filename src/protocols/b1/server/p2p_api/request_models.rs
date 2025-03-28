use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};

/// Represents each of the types of request that can be made to the SRS Peer to Peer API.
/// Currently ignores the 'protocol' field, since that is always `B1` and has never changed.
/// May need to include that later if SRS changes.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
#[serde(tag = "requestType")]
pub enum RequestType {
    AddPeers { peers: Vec<String> },
    GetBlocksFromHeight(GetBlocksFromHeightPayload),
    GetCumulativeDifficulty,
    GetInfo(GetInfoPayload),
    GetMilestoneBlockIds(GetMilestoneBlockIdsPayload),
    GetNextBlocks(GetNextBlocksPayload),
    GetNextBlockIds(GetNextBlockIdsPayload),
    GetUnconfirmedTransactions,
    GetPeers,
    ProcessBlock,
    ProcessTransactions,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBlocksFromHeightPayload {
    pub height: u32,
    pub num_blocks: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoPayload {
    pub announced_address: Option<String>,
    pub application: Option<String>,
    pub version: Option<String>,
    pub platform: Option<String>,
    pub share_address: Option<bool>,
    pub network_name: String,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMilestoneBlockIdsPayload {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub last_block_id: Option<u64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub last_milestone_block_id: Option<u64>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetNextBlocksPayload {
    #[serde_as(as = "DisplayFromStr")]
    pub block_id: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetNextBlockIdsPayload {
    #[serde_as(as = "DisplayFromStr")]
    pub block_id: u64,
}
