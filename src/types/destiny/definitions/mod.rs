use serde::{Deserialize, Serialize};

use super::{DestinyActivityRewardDisplayMode, DestinyItemQuantity};

pub mod sockets;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyMaterialRequirement {
    pub item_hash: u32,
    pub delete_on_action: bool,
    pub count: i32,
    pub count_is_constant: bool,
    pub omit_from_requirements: bool,
    #[serde(default)]
    pub has_virtual_stack_size: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyItemSocketEntryPlugItemRandomizedDefinition {
    pub crafting_requirements: Option<DestinyPlugItemCraftingRequirements>,
    pub weight: f32,
    pub alternate_weight: f32,
    pub currently_can_roll: bool,
    pub plug_item_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPlugItemCraftingRequirements {
    pub unlock_requirements: Vec<DestinyPlugItemCraftingUnlockRequirement>,
    pub required_level: Option<i32>,
    pub material_requirement_hashes: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPlugItemCraftingUnlockRequirement {
    pub failure_description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyActivityRewardMapping {
    pub display_behavior: DestinyActivityRewardDisplayMode,
    pub reward_items: Vec<DestinyActivityRewardItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyActivityRewardItem {
    pub item_quantity: DestinyItemQuantity,
    pub ui_style: String,
    pub visibility_unlock_expression: DestinyUnlockExpressionDefinition,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyActivityInteractableReference {
    pub activity_interactable_hash: u32,
    pub activity_interactable_element_index: i32,
}

/// Not documented in the API spec
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyUnlockExpressionDefinition {
    pub steps: Vec<DestinyUnlockExpressionStep>,
    pub scope: i32,
}

/// Not documented in the API spec
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyUnlockExpressionStep {
    pub step_operator: i32,
    pub value: i32,
    pub unlock_hash: Option<u32>,
    pub value_hash: Option<u32>,
    pub mapping_hash: Option<u32>,
}
