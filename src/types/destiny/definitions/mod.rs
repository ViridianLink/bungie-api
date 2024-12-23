use serde::{Deserialize, Serialize};

pub mod sockets;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMaterialRequirement {
    pub item_hash: u32,
    pub delete_on_action: bool,
    pub count: i32,
    pub count_is_constant: bool,
    pub omit_from_requirements: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketEntryPlugItemRandomizedDefinition {
    pub crafting_requirements: Option<DestinyPlugItemCraftingRequirements>,
    pub currently_can_roll: bool,
    pub plug_item_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugItemCraftingRequirements {
    pub unlock_requirements: Vec<DestinyPlugItemCraftingUnlockRequirement>,
    pub required_level: Option<i32>,
    pub material_requirement_hashes: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugItemCraftingUnlockRequirement {
    pub failure_description: String,
}
