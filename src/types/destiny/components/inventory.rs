use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::BungieMembershipType;
use crate::types::destiny::entities::items::DestinyItemComponent;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPlatformSilverComponent {
    pub platform_silver: HashMap<BungieMembershipType, DestinyItemComponent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyCurrenciesComponent {
    pub item_quantities: HashMap<u32, i32>,
    #[serde(default)]
    pub material_requirement_set_states:
        HashMap<u32, DestinyMaterialRequirementSetState>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMaterialRequirementSetState {
    pub material_requirement_set_hash: u32,
    pub material_requirement_states: Vec<DestinyMaterialRequirementState>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMaterialRequirementState {
    pub item_hash: u32,
    pub count: i32,
    pub stack_size: i32,
}
