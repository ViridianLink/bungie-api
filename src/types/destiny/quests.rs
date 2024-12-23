use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyObjectiveProgress {
    pub objective_hash: u32,
    pub destination_hash: u32,
    pub activity_hash: u32,
    pub progress: i32,
    pub completion_value: i32,
    pub complete: bool,
    pub visible: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyQuestStatus {
    pub quest_hash: u32,
    pub step_hash: u32,
    pub step_objectives: Option<Vec<DestinyObjectiveProgress>>,
    pub tracked: bool,
    pub item_instance_id: i64,
    pub completed: bool,
    pub redeemed: bool,
    pub started: bool,
    pub vendor_hash: u32,
}
