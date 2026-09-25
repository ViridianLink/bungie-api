use serde::{Deserialize, Serialize};

use super::quests::DestinyObjectiveProgress;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyItemPlugBase {
    pub plug_item_hash: u32,
    pub can_insert: bool,
    pub enabled: bool,
    pub insert_fail_indexes: Vec<i32>,
    pub enable_fail_indexes: Vec<i32>,
    pub stack_size: Option<i32>,
    pub max_stack_size: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyItemPlug {
    pub plug_objectives: Vec<DestinyObjectiveProgress>,
    pub plug_item_hash: u32,
    pub can_insert: bool,
    pub enabled: bool,
    pub insert_fail_indexes: Vec<i32>,
    pub enable_fail_indexes: Vec<i32>,
    pub stack_size: Option<i32>,
    pub max_stack_size: Option<i32>,
}
