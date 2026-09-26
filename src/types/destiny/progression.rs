use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    DestinyProgressionResetEntry,
    DestinyProgressionRewardItemSocketOverrideState,
    DestinyProgressionRewardItemState,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyFactionProgression {
    pub faction_hash: u32,
    pub faction_vendor_index: i32,
    pub progression_hash: u32,
    pub daily_progress: i32,
    pub daily_limit: i32,
    pub weekly_progress: i32,
    pub weekly_limit: i32,
    pub current_progress: i32,
    pub level: i32,
    pub level_cap: i32,
    pub step_index: i32,
    pub progress_to_next_level: i32,
    pub next_level_at: i32,
    pub current_reset_count: i32,
    pub season_resets: Vec<DestinyProgressionResetEntry>,
    pub reward_item_states: Vec<DestinyProgressionRewardItemState>,
    #[serde(default)]
    pub reward_item_socket_override_states:
        HashMap<i32, DestinyProgressionRewardItemSocketOverrideState>,
}
