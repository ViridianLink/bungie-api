use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{challenges::DestinyChallengeStatus, quests::DestinyQuestStatus};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestone {
    pub milestone_hash: u32,
    pub available_quests: Option<Vec<DestinyMilestoneQuest>>,
    pub activities: Vec<DestinyMilestoneChallengeActivity>,
    pub values: HashMap<String, f32>,
    pub vendor_hashes: Option<Vec<u32>>,
    pub vendors: Vec<DestinyMilestoneVendor>,
    pub rewards: Vec<DestinyMilestoneRewardCategory>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub order: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneQuest {
    pub quest_item_hash: u32,
    pub status: DestinyQuestStatus,
    pub activity: DestinyMilestoneActivity,
    pub challenges: Vec<DestinyChallengeStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneActivity {
    pub activity_hash: u32,
    pub activity_mode_hash: u32,
    pub activity_mode_type: i32,
    pub modifier_hashes: Vec<u32>,
    pub variants: Vec<DestinyMilestoneActivityVariant>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneActivityVariant {
    pub activity_hash: u32,
    pub completion_status: DestinyMilestoneActivityCompletionStatus,
    pub activity_mode_hash: u32,
    pub activity_mode_type: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyMilestoneActivityCompletionStatus {
    pub completed: bool,
    pub phases: Vec<DestinyMilestoneActivityPhase>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyMilestoneActivityPhase {
    pub complete: bool,
    pub phase_hash: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneChallengeActivity {
    pub activity_hash: u32,
    pub challenges: Vec<DestinyChallengeStatus>,
    pub modifier_hashes: Vec<u32>,
    pub boolean_activity_options: HashMap<u32, bool>,
    pub loadout_requirement_index: i32,
    pub phases: Vec<DestinyMilestoneActivityPhase>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneVendor {
    pub vendor_hash: u32,
    pub preview_item_hash: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneRewardCategory {
    pub reward_category_hash: u32,
    pub entries: Vec<DestinyMilestoneRewardEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneRewardEntry {
    pub reward_entry_hash: u32,
    pub earned: bool,
    pub redeemed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneContent {
    pub about: Option<String>,
    pub status: Option<String>,
    pub tips: Option<Vec<String>>,
    pub item_categories: Vec<DestinyMilestoneContentItemCategory>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneContentItemCategory {
    pub title: Option<String>,
    pub item_hashes: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPublicMilestone {
    pub milestone_hash: u32,
    pub available_quests: Vec<DestinyPublicMilestoneQuest>,
    pub activities: Vec<DestinyPublicMilestoneChallengeActivity>,
    pub vendor_hashes: Vec<u32>,
    pub vendors: Vec<DestinyPublicMilestoneVendor>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub order: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPublicMilestoneQuest {
    pub quest_item_hash: u32,
    pub activity: DestinyPublicMilestoneActivity,
    pub challenges: Vec<DestinyPublicMilestoneChallenge>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPublicMilestoneActivity {
    pub activity_hash: u32,
    pub modifier_hashes: Vec<u32>,
    pub variants: Vec<DestinyPublicMilestoneActivityVariant>,
    pub activity_mode_hash: u32,
    pub activity_mode_type: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPublicMilestoneActivityVariant {
    pub activity_hash: u32,
    pub activity_mode_hash: u32,
    pub activity_mode_type: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPublicMilestoneChallenge {
    pub objective_hash: u32,
    pub activity_hash: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPublicMilestoneChallengeActivity {
    pub activity_hash: u32,
    pub challenge_objective_hashes: Vec<u32>,
    pub modifier_hashes: Vec<u32>,
    pub loadout_requirement_index: i32,
    pub phase_hashes: Vec<u32>,
    pub boolean_activity_options: HashMap<u32, bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPublicMilestoneVendor {
    pub vendor_hash: u32,
    pub preview_item_hash: u32,
}
