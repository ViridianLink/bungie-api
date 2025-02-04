use serde::{Deserialize, Serialize};

use crate::types::destiny::{DestinyEnergyType, PlugAvailabilityMode, PlugUiStyles};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyDerivedItemCategoryDefinition {
    pub category_description: String,
    pub items: Vec<DestinyDerivedItemDefinition>,
    pub category_index: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyDerivedItemDefinition {
    pub item_hash: u32,
    pub item_name: Option<String>,
    pub item_detail: Option<String>,
    pub item_description: Option<String>,
    pub icon_path: Option<String>,
    pub vendor_item_index: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemPlugDefinition {
    pub insertion_rules: Vec<DestinyPlugRuleDefinition>,
    pub plug_category_identifier: String,
    pub plug_category_hash: u32,
    pub on_action_recreate_self: bool,
    pub action_reward_site_hash: u32,
    pub action_reward_item_override_hash: u32,
    pub insertion_material_requirement_hash: u32,
    pub preview_item_override_hash: u32,
    pub enabled_material_requirement_hash: u32,
    pub enabled_rules: Vec<DestinyPlugRuleDefinition>,
    pub ui_plug_label: String,
    pub plug_style: PlugUiStyles,
    pub plug_availability: PlugAvailabilityMode,
    pub alternate_ui_plug_label: String,
    pub alternate_plug_style: PlugUiStyles,
    pub is_dummy_plug: bool,
    pub parent_item_override: DestinyParentItemOverride,
    pub apply_stats_to_socket_owner_item: bool,
    pub energy_capacity: Option<DestinyEnergyCapacityEntry>,
    pub energy_cost: Option<DestinyEnergyCostEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyPlugRuleDefinition {
    pub failure_message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyParentItemOverride {
    pub additional_equip_requirements_display_strings: Vec<String>,
    pub pip_icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyEnergyCapacityEntry {
    pub capacity_value: i32,
    pub energy_type_hash: u32,
    pub energy_type: DestinyEnergyType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyEnergyCostEntry {
    pub energy_cost: i32,
    pub energy_type_hash: u32,
    pub energy_type: DestinyEnergyType,
}
