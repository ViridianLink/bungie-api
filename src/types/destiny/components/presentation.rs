use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::destiny::DestinyPresentationNodeState;
use crate::types::destiny::quests::DestinyObjectiveProgress;

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyPresentationNodesComponent {
    pub nodes: HashMap<u32, DestinyPresentationNodeComponent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyPresentationNodeComponent {
    pub state: DestinyPresentationNodeState,
    pub objective: DestinyObjectiveProgress,
    pub progress_value: i32,
    pub completion_value: i32,
    pub record_category_score: i32,
}
