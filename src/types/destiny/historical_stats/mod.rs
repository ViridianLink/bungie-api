use std::collections::HashMap;

use chrono::{DateTime, Utc};
use definitions::DestinyActivityModeType;
use serde::{Deserialize, Serialize};

use crate::serde_as::string_to_u64;
use crate::types::BungieMembershipType;

pub mod definitions;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyHistoricalStatsActivity {
    pub reference_id: u32,
    pub director_activity_hash: u32,
    #[serde(deserialize_with = "string_to_u64")]
    pub instance_id: u64,
    pub mode: DestinyActivityModeType,
    pub modes: Vec<DestinyActivityModeType>,
    pub is_private: bool,
    pub membership_type: BungieMembershipType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyHistoricalStatsValue {
    pub stat_id: String,
    pub basic: DestinyHistoricalStatsValuePair,
    pub pga: Option<DestinyHistoricalStatsValuePair>,
    pub weighted: Option<DestinyHistoricalStatsValuePair>,
    pub activity_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyHistoricalStatsValuePair {
    pub value: f64,
    pub display_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyHistoricalStatsPeriodGroup {
    pub period: DateTime<Utc>,

    pub activity_details: DestinyHistoricalStatsActivity,

    pub values: HashMap<String, DestinyHistoricalStatsValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyActivityHistoryResults {
    pub activities: Vec<DestinyHistoricalStatsPeriodGroup>,
}
