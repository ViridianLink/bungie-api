use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::destiny::artifacts::DestinyArtifactProfileScoped;
use crate::types::destiny::{
    DestinyGamePrivacySetting, DestinyJoinClosedReasons, DestinyPartyMemberStates,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyProfileProgressionComponent {
    pub checklists: HashMap<u32, HashMap<u32, bool>>,
    pub seasonal_artifact: DestinyArtifactProfileScoped,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyProfileTransitoryComponent {
    pub party_members: Vec<DestinyProfileTransitoryPartyMember>,
    pub current_activity: DestinyProfileTransitoryCurrentActivity,
    pub joinability: DestinyProfileTransitoryJoinability,
    pub tracking: Vec<DestinyProfileTransitoryTrackingEntry>,
    pub last_orbited_destination_hash: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyProfileTransitoryPartyMember {
    pub membership_id: i64,
    pub emblem_hash: u32,
    pub display_name: String,
    pub status: DestinyPartyMemberStates,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyProfileTransitoryCurrentActivity {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub score: f32,
    pub highest_opposing_faction_score: f32,
    pub number_of_opponents: i32,
    pub number_of_players: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyProfileTransitoryJoinability {
    pub open_slots: i32,
    pub privacy_setting: DestinyGamePrivacySetting,
    pub closed_reasons: DestinyJoinClosedReasons,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyProfileTransitoryTrackingEntry {
    pub location_hash: u32,
    pub item_hash: u32,
    pub objective_hash: u32,
    pub activity_hash: u32,
    pub questline_item_hash: u32,
    pub tracked_date: DateTime<Utc>,
}
