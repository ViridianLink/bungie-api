use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySocialCommendationsComponent {
    pub total_score: i32,
    pub score_detail_values: Vec<i32>,
    pub commendation_node_scores_by_hash: HashMap<u32, i32>,
    pub commendation_scores_by_hash: HashMap<u32, i32>,
}
