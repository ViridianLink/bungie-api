use serde::{Deserialize, Serialize};

use super::quests::DestinyObjectiveProgress;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemPlugBase {
    pub plug_item_hash: u32,
    pub can_insert: bool,
    pub enabled: bool,
    pub insert_fail_indexes: Vec<i32>,
    pub enable_fail_indexes: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemPlug {
    pub plug_objectives: Vec<DestinyObjectiveProgress>,
    pub plug_item_hash: u32,
    pub can_insert: bool,
    pub enabled: bool,
    pub insert_fail_indexes: Vec<i32>,
    pub enable_fail_indexes: Vec<i32>,
}
