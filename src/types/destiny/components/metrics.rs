use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::destiny::quests::DestinyObjectiveProgress;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyMetricsComponent {
    pub metrics: HashMap<u32, DestinyMetricComponent>,
    pub metrics_root_node_hash: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyMetricComponent {
    pub invisible: bool,
    pub objective_progress: DestinyObjectiveProgress,
}
