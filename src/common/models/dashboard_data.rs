use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DashboardData {
    pub b1_total_peers: u32,
    pub b1_allowed_peers: u32,
    pub b1_blacklisted_peers: u32,
}
