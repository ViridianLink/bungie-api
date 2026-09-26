use std::collections::HashMap;

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::challenges::DestinyChallengeStatus;
use super::quests::DestinyQuestStatus;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestone {
    pub milestone_hash: u32,
    pub available_quests: Option<Vec<DestinyMilestoneQuest>>,
    #[serde(default)]
    pub activities: Vec<DestinyMilestoneChallengeActivity>,
    #[serde(default)]
    pub values: HashMap<String, f32>,
    pub vendor_hashes: Option<Vec<u32>>,
    #[serde(default)]
    pub vendors: Vec<DestinyMilestoneVendor>,
    #[serde(default)]
    pub rewards: Vec<DestinyMilestoneRewardCategory>,
    pub start_date: Option<Timestamp>,
    pub end_date: Option<Timestamp>,
    pub order: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneQuest {
    pub quest_item_hash: u32,
    pub status: DestinyQuestStatus,
    pub activity: Option<DestinyMilestoneActivity>,
    #[serde(default)]
    pub challenges: Vec<DestinyChallengeStatus>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneActivity {
    pub activity_hash: u32,
    pub activity_mode_hash: u32,
    pub activity_mode_type: i32,
    pub modifier_hashes: Vec<u32>,
    pub variants: Vec<DestinyMilestoneActivityVariant>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneActivityVariant {
    pub activity_hash: u32,
    pub completion_status: DestinyMilestoneActivityCompletionStatus,
    pub activity_mode_hash: u32,
    pub activity_mode_type: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneActivityCompletionStatus {
    pub completed: bool,
    pub phases: Vec<DestinyMilestoneActivityPhase>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneActivityPhase {
    pub complete: bool,
    pub phase_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneChallengeActivity {
    pub activity_hash: u32,
    pub challenges: Vec<DestinyChallengeStatus>,
    pub modifier_hashes: Vec<u32>,
    #[serde(default)]
    pub boolean_activity_options: HashMap<u32, bool>,
    pub loadout_requirement_index: Option<i32>,
    #[serde(default)]
    pub phases: Vec<DestinyMilestoneActivityPhase>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneVendor {
    pub vendor_hash: u32,
    pub preview_item_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneRewardCategory {
    pub reward_category_hash: u32,
    pub entries: Vec<DestinyMilestoneRewardEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneRewardEntry {
    pub reward_entry_hash: u32,
    pub earned: bool,
    pub redeemed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneContent {
    pub about: Option<String>,
    pub status: Option<String>,
    pub tips: Option<Vec<String>>,
    pub item_categories: Vec<DestinyMilestoneContentItemCategory>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMilestoneContentItemCategory {
    pub title: Option<String>,
    pub item_hashes: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPublicMilestone {
    pub milestone_hash: u32,
    pub available_quests: Vec<DestinyPublicMilestoneQuest>,
    pub activities: Vec<DestinyPublicMilestoneChallengeActivity>,
    pub vendor_hashes: Vec<u32>,
    pub vendors: Vec<DestinyPublicMilestoneVendor>,
    pub start_date: Timestamp,
    pub end_date: Timestamp,
    pub order: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPublicMilestoneQuest {
    pub quest_item_hash: u32,
    pub activity: DestinyPublicMilestoneActivity,
    pub challenges: Vec<DestinyPublicMilestoneChallenge>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPublicMilestoneActivity {
    pub activity_hash: u32,
    pub modifier_hashes: Vec<u32>,
    pub variants: Vec<DestinyPublicMilestoneActivityVariant>,
    pub activity_mode_hash: u32,
    pub activity_mode_type: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPublicMilestoneActivityVariant {
    pub activity_hash: u32,
    pub activity_mode_hash: u32,
    pub activity_mode_type: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPublicMilestoneChallenge {
    pub objective_hash: u32,
    pub activity_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPublicMilestoneChallengeActivity {
    pub activity_hash: u32,
    pub challenge_objective_hashes: Vec<u32>,
    pub modifier_hashes: Vec<u32>,
    pub loadout_requirement_index: i32,
    pub phase_hashes: Vec<u32>,
    pub boolean_activity_options: HashMap<u32, bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPublicMilestoneVendor {
    pub vendor_hash: u32,
    pub preview_item_hash: u32,
}
