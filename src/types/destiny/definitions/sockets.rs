use serde::{Deserialize, Serialize};

use crate::types::common::DestinyDisplayPropertiesDefinition;
use crate::types::destiny::{
    DestinySocketCategoryStyle, DestinySocketVisibility, SocketTypeActionType,
};

use super::DestinyItemSocketEntryPlugItemRandomizedDefinition;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySocketTypeDefinition {
    pub display_properties: DestinyDisplayPropertiesDefinition,
    pub insert_action: Option<DestinyInsertPlugActionDefinition>,
    #[serde(default)]
    pub plug_whitelist: Vec<DestinyPlugWhitelistEntryDefinition>,
    pub socket_category_hash: u32,
    pub visibility: DestinySocketVisibility,
    pub always_randomize_sockets: bool,
    pub is_preview_enabled: bool,
    pub hide_duplicate_reusable_plugs: bool,
    pub overrides_ui_appearance: bool,
    pub avoid_duplicates_on_initialization: bool,
    #[serde(default)]
    pub currency_scalars: Vec<DestinySocketTypeScalarMaterialRequirementEntry>,
    pub hash: u32,
    pub index: i32,
    pub redacted: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyInsertPlugActionDefinition {
    pub action_execute_seconds: i32,
    pub action_type: SocketTypeActionType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugWhitelistEntryDefinition {
    pub category_hash: u32,
    pub category_identifier: String,
    #[serde(default)]
    pub reinitialization_possible_plug_hashes: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySocketTypeScalarMaterialRequirementEntry {
    pub currency_item_hash: u32,
    pub scalar_value: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySocketCategoryDefinition {
    pub display_properties: DestinyDisplayPropertiesDefinition,
    pub ui_category_style: u32,
    pub category_style: DestinySocketCategoryStyle,
    pub hash: u32,
    pub index: i32,
    pub redacted: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugSetDefinition {
    pub display_properties: Option<DestinyDisplayPropertiesDefinition>,
    #[serde(default)]
    pub reusable_plug_items: Vec<DestinyItemSocketEntryPlugItemRandomizedDefinition>,
    pub is_fake_plug_set: bool,
    pub hash: u32,
    pub index: i32,
    pub redacted: bool,
}
