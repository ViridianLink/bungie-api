use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::destiny::quests::DestinyObjectiveProgress;
use crate::types::destiny::sockets::DestinyItemPlugBase;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyItemReusablePlugsComponent {
    pub plugs: HashMap<i32, Vec<DestinyItemPlugBase>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyItemPlugObjectivesComponent {
    pub objectives_per_plug: HashMap<u32, Vec<DestinyObjectiveProgress>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyItemPlugComponent {
    pub plug_objectives: Vec<DestinyObjectiveProgress>,
    pub plug_item_hash: u32,
    pub can_insert: bool,
    pub enabled: bool,
    pub insert_fail_indexes: Vec<i32>,
    pub enable_fail_indexes: Vec<i32>,
    pub stack_size: Option<i32>,
    pub max_stack_size: Option<i32>,
}
