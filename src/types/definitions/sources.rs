use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::DestinyInventoryItemStatDefinition;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSourceDefinition {
    pub level: i32,
    pub min_quality: i32,
    pub max_quality: i32,
    pub min_level_required: i32,
    pub max_level_required: i32,
    pub computed_stats: HashMap<u32, DestinyInventoryItemStatDefinition>,
    pub source_hashes: Vec<u32>,
}
