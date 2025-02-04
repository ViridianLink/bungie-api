use serde::{Deserialize, Serialize};

use super::DestinyProgression;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyArtifactProfileScoped {
    pub artifact_hash: u32,
    pub point_progression: DestinyProgression,
    pub points_acquired: i32,
    pub power_bonus_progression: DestinyProgression,
    pub power_bonus: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyArtifactCharacterScoped {
    pub artifact_hash: u32,
    pub points_used: i32,
    pub reset_count: i32,
    pub tiers: Vec<DestinyArtifactTier>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyArtifactTier {
    pub tier_hash: u32,
    pub is_unlocked: bool,
    pub points_to_unlock: i32,
    pub items: Vec<DestinyArtifactTierItem>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyArtifactTierItem {
    pub item_hash: u32,
    pub is_active: bool,
}
