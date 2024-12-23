use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::destiny::quests::DestinyObjectiveProgress;
use crate::types::destiny::DestinyRecordState;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordComponent {
    pub state: DestinyRecordState,
    pub objectives: Vec<DestinyObjectiveProgress>,
    pub interval_objectives: Vec<DestinyObjectiveProgress>,
    pub intervals_redeemed_count: i32,
    pub completed_count: i32,
    pub reward_visibilty: Vec<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProfileRecordsComponent {
    pub score: i32,
    pub active_score: i32,
    pub legacy_score: i32,
    pub lifetime_score: i32,
    pub tracked_record_hash: Option<u32>,
    pub records: HashMap<u32, DestinyRecordComponent>,
    pub record_categories_root_node_hash: u32,
    pub record_seals_root_node_hash: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyCharacterRecordsComponent {
    pub featured_record_hashes: Vec<u32>,
    pub records: HashMap<u32, DestinyRecordComponent>,
    pub record_categories_root_node_hash: u32,
    pub record_seals_root_node_hash: u32,
}
