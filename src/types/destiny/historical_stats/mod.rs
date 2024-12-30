use std::collections::HashMap;

use chrono::{DateTime, Utc};
use definitions::DestinyActivityModeType;
use serde::{Deserialize, Serialize};

use crate::serde_as::string_to_u64;
use crate::types::user::UserInfoCard;
use crate::types::BungieMembershipType;

pub mod definitions;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPostGameCarnageReportData {
    pub period: DateTime<Utc>,
    pub starting_phase_index: i32,
    pub activity_was_started_from_beginning: bool,
    pub activity_details: DestinyHistoricalStatsActivity,
    pub entries: Vec<DestinyPostGameCarnageReportEntry>,
    pub teams: Vec<DestinyPostGameCarnageReportTeamEntry>,
}

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
pub struct DestinyPostGameCarnageReportEntry {
    pub standing: i32,
    pub score: DestinyHistoricalStatsValue,
    pub player: DestinyPlayer,
    pub character_id: String,
    pub values: HashMap<String, DestinyHistoricalStatsValue>,
    pub extended: DestinyPostGameCarnageReportExtendedData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyHistoricalStatsValue {
    pub stat_id: Option<String>,
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
pub struct DestinyPlayer {
    pub destiny_user_info: UserInfoCard,
    pub character_class: String,
    pub class_hash: u32,
    pub race_hash: u32,
    pub gender_hash: u32,
    pub character_level: i32,
    pub light_level: i32,
    pub bungie_net_user_info: Option<UserInfoCard>,
    pub clan_name: Option<String>,
    pub clan_tag: Option<String>,
    pub emblem_hash: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyPostGameCarnageReportExtendedData {
    pub weapons: Vec<DestinyHistoricalWeaponStats>,
    pub values: HashMap<String, DestinyHistoricalStatsValue>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyHistoricalWeaponStats {
    pub reference_id: u32,
    pub values: HashMap<String, DestinyHistoricalStatsValue>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPostGameCarnageReportTeamEntry {
    pub team_id: i32,
    pub standing: DestinyHistoricalStatsValue,
    pub score: DestinyHistoricalStatsValue,
    pub team_name: String,
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
