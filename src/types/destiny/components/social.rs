use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinySocialCommendationsComponent {
    pub total_score: i32,
    pub score_detail_values: Vec<i32>,
    pub commendation_node_scores_by_hash: HashMap<u32, i32>,
    pub commendation_scores_by_hash: HashMap<u32, i32>,
}
