use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::destiny::quests::DestinyObjectiveProgress;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyKiosksComponent {
    pub kiosk_items: HashMap<u32, Vec<DestinyKioskItem>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyKioskItem {
    pub index: i32,
    pub can_acquire: bool,
    pub failure_indexes: Vec<i32>,
    pub flavor_objective: DestinyObjectiveProgress,
}
