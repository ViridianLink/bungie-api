use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::destiny::quests::DestinyObjectiveProgress;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMetricsComponent {
    pub metrics: HashMap<u32, DestinyMetricComponent>,
    pub metrics_root_node_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMetricComponent {
    pub invisible: bool,
    pub objective_progress: DestinyObjectiveProgress,
}
