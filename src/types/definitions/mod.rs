pub mod animations;
pub mod items;
pub mod sources;

use std::collections::HashMap;

use animations::DestinyAnimationReference;
use items::{DestinyDerivedItemCategoryDefinition, DestinyItemPlugDefinition};
use serde::{Deserialize, Serialize};
use sources::DestinyItemSourceDefinition;

use super::common::DestinyDisplayPropertiesDefinition;
use super::destiny::{
    DamageType, DestinyAmmunitionType, DestinyBreakerType, DestinyClass, DestinyItemQuantity,
    DestinyItemSubType, DestinyItemType, DyeReference, EquippingItemBlockAttributes,
    ItemPerkVisibility, SocketPlugSources, SpecialItemType,
};
use super::links::HyperlinkReference;
use super::misc::DestinyColor;
use super::{BungieMembershipType, TierType};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyInventoryItemDefinition {
    pub display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(default)]
    pub tooltip_notifications: Vec<DestinyItemTooltipNotification>,
    pub collectible_hash: Option<u32>,
    pub icon_watermark: Option<String>,
    pub icon_watermark_shelved: Option<String>,
    pub icon_watermark_featured: Option<String>,
    pub secondary_icon: Option<String>,
    pub secondary_overlay: Option<String>,
    pub secondary_special: Option<String>,
    pub background_color: Option<DestinyColor>,
    pub is_featured_item: bool,
    pub is_holofoil: bool,
    pub is_adept: bool,
    pub screenshot: Option<String>,
    pub item_type_display_name: Option<String>,
    pub flavor_text: Option<String>,
    pub ui_item_display_style: Option<String>,
    pub item_type_and_tier_display_name: Option<String>,
    pub display_source: Option<String>,
    pub tooltip_style: Option<String>,
    pub action: Option<DestinyItemActionBlockDefinition>,
    pub crafting: Option<DestinyItemCraftingBlockDefinition>,
    pub inventory: DestinyItemInventoryBlockDefinition,
    pub set_data: Option<DestinyItemSetBlockDefinition>,
    pub stats: Option<DestinyItemStatBlockDefinition>,
    pub emblem_objective_hash: Option<u32>,
    pub equipping_block: Option<DestinyEquippingBlockDefinition>,
    pub translation_block: Option<DestinyItemTranslationBlockDefinition>,
    pub preview: Option<DestinyItemPreviewBlockDefinition>,
    pub quality: Option<DestinyItemQualityBlockDefinition>,
    pub value: Option<DestinyItemValueBlockDefinition>,
    pub source_data: Option<DestinyItemSourceBlockDefinition>,
    pub objectives: Option<DestinyItemObjectiveBlockDefinition>,
    pub metrics: Option<DestinyItemMetricBlockDefinition>,
    pub plug: Option<DestinyItemPlugDefinition>,
    pub gearset: Option<DestinyItemGearsetBlockDefinition>,
    pub sack: Option<DestinyItemSackBlockDefinition>,
    pub sockets: Option<DestinyItemSocketBlockDefinition>,
    pub summary: Option<DestinyItemSummaryBlockDefinition>,
    pub talent_grid: Option<DestinyItemTalentGridBlockDefinition>,
    pub acquire_reward_site_hash: u32,
    pub acquire_unlock_hash: u32,
    #[serde(default)]
    pub investment_stats: Vec<DestinyItemInvestmentStatDefinition>,
    #[serde(default)]
    pub perks: Vec<DestinyItemPerkEntryDefinition>,
    pub lore_hash: Option<u32>,
    pub summary_item_hash: Option<u32>,
    #[serde(default)]
    pub animations: Vec<DestinyAnimationReference>,
    pub allow_actions: bool,
    #[serde(default)]
    pub links: Vec<HyperlinkReference>,
    pub does_postmaster_pull_have_side_effects: bool,
    pub non_transferrable: bool,
    #[serde(default)]
    pub item_category_hashes: Vec<u32>,
    pub special_item_type: SpecialItemType,
    pub item_type: DestinyItemType,
    pub item_sub_type: DestinyItemSubType,
    pub class_type: DestinyClass,
    pub breaker_type: DestinyBreakerType,
    pub breaker_type_hash: Option<u32>,
    pub equippable: bool,
    #[serde(default)]
    pub damage_type_hashes: Vec<u32>,
    #[serde(default)]
    pub damage_types: Vec<DamageType>,
    pub default_damage_type: DamageType,
    pub default_damage_type_hash: Option<u32>,
    pub season_hash: Option<u32>,
    pub is_wrapper: bool,
    #[serde(default)]
    pub trait_ids: Vec<String>,
    #[serde(default)]
    pub trait_hashes: Vec<u32>,
    pub hash: u32,
    pub index: i32,
    pub redacted: bool,
    pub blacklisted: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemTooltipNotification {
    pub display_string: String,
    pub display_style: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemActionBlockDefinition {
    pub verb_name: String,
    pub verb_description: String,
    pub is_positive: bool,
    pub overlay_screen_name: Option<String>,
    pub overlay_icon: Option<String>,
    pub required_cooldown_seconds: i32,
    pub required_items: Vec<DestinyItemActionRequiredItemDefinition>,
    pub progression_rewards: Vec<DestinyProgressionRewardDefinition>,
    pub reward_sheet_hash: u32,
    pub reward_item_hash: u32,
    pub reward_site_hash: u32,
    pub action_type_label: Option<String>,
    pub required_location: Option<String>,
    pub required_cooldown_hash: u32,

    pub delete_on_action: bool,
    pub consume_entire_stack: bool,
    pub use_on_acquire: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemActionRequiredItemDefinition {
    pub count: i32,
    pub item_hash: u32,
    pub delete_on_action: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyProgressionRewardDefinition {
    pub progression_mapping_hash: u32,
    pub amount: i32,
    pub apply_throttles: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemCraftingBlockDefinition {
    pub output_item_hash: u32,
    pub required_socket_type_hashes: Vec<u32>,
    pub failed_requirement_strings: Vec<String>,
    pub base_material_requirements: Option<u32>,
    pub bonus_plugs: Vec<DestinyItemCraftingBlockBonusPlugDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemCraftingBlockBonusPlugDefinition {
    pub socket_type_hash: u32,
    pub plug_item_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemInventoryBlockDefinition {
    pub stack_unique_label: Option<String>,
    pub max_stack_size: i32,
    pub bucket_type_hash: u32,
    pub recovery_bucket_type_hash: u32,
    pub tier_type_hash: u32,
    pub is_instance_item: bool,
    pub non_transferrable_original: bool,
    pub tier_type_name: Option<String>,
    pub tier_type: TierType,
    pub expiration_tooltip: Option<String>,
    pub expired_in_activity_message: Option<String>,
    pub expired_in_orbit_message: Option<String>,
    pub suppress_expiration_when_objectives_complete: bool,
    pub recipe_item_hash: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSetBlockDefinition {
    pub item_list: Vec<DestinyItemSetBlockEntryDefinition>,
    pub tracking_unlock_value_hash: u32,
    pub abandonment_unlock_hash: u32,
    pub require_ordered_set_item_add: bool,
    pub set_is_featured: bool,
    pub set_type: String,
    pub quest_line_name: String,
    pub quest_line_description: String,
    pub quest_step_summary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSetBlockEntryDefinition {
    pub tracking_value: i32,
    pub item_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemStatBlockDefinition {
    pub disable_primary_stat_display: bool,
    pub stat_group_hash: Option<u32>,
    pub stats: HashMap<u32, DestinyInventoryItemStatDefinition>,
    pub has_displayable_stats: bool,
    pub primary_base_stat_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyInventoryItemStatDefinition {
    pub stat_hash: u32,
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub display_maximum: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyEquippingBlockDefinition {
    pub gearset_item_hash: Option<u32>,
    pub unique_label: Option<String>,
    pub unique_label_hash: u32,
    pub equipment_slot_type_hash: u32,
    pub attributes: EquippingItemBlockAttributes,
    pub equipping_sound_hash: u32,
    pub horn_sound_hash: u32,
    pub ammo_type: DestinyAmmunitionType,
    pub display_strings: Vec<String>,
    pub equipable_item_set_hash: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemTranslationBlockDefinition {
    pub weapon_pattern_identifier: Option<String>,
    pub weapon_pattern_hash: u32,
    pub default_dyes: Vec<DyeReference>,
    pub locked_dyes: Vec<DyeReference>,
    pub custom_dyes: Vec<DyeReference>,
    pub arrangements: Vec<DestinyGearArtArrangementReference>,
    pub has_geometry: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyGearArtArrangementReference {
    pub class_hash: u32,
    pub art_arrangement_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemPreviewBlockDefinition {
    pub screen_style: String,
    pub preview_vendor_hash: u32,
    pub artifact_hash: Option<u32>,
    pub preview_action_string: String,
    #[serde(default)]
    pub derived_item_categories: Vec<DestinyDerivedItemCategoryDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemQualityBlockDefinition {
    pub item_levels: Vec<i32>,
    pub quality_level: i32,
    pub infusion_category_name: String,
    pub infusion_category_hash: u32,
    pub infusion_category_hashes: Vec<u32>,
    pub progression_level_requirement_hash: u32,
    pub current_version: u32,
    pub versions: Vec<DestinyItemVersionDefinition>,
    pub display_version_watermark_icons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemVersionDefinition {
    pub power_cap_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemValueBlockDefinition {
    pub item_value: Vec<DestinyItemQuantity>,
    pub value_description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSourceBlockDefinition {
    pub source_hashes: Vec<u32>,
    pub sources: Vec<DestinyItemSourceDefinition>,
    pub exclusive: BungieMembershipType,
    pub vendor_sources: Vec<DestinyItemVendorSourceReference>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemVendorSourceReference {
    pub vendor_hash: u32,
    pub vendor_item_indexes: Vec<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemObjectiveBlockDefinition {
    pub objective_hashes: Vec<u32>,
    pub display_activity_hashes: Vec<u32>,
    pub require_full_objective_completion: bool,
    pub questline_item_hash: u32,
    pub narrative: String,
    pub objective_verb_name: String,
    pub quest_type_identifier: String,
    pub quest_type_hash: u32,
    pub completion_reward_site_hash: u32,
    pub next_quest_step_reward_site_hash: u32,
    pub timestamp_unlock_value_hash: u32,
    pub is_global_objective_item: bool,
    pub use_on_objective_completion: bool,
    pub inhibit_completion_unlock_value_hash: u32,
    pub per_objective_display_properties: Vec<DestinyObjectiveDisplayProperties>,
    pub display_as_stat_tracker: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyObjectiveDisplayProperties {
    pub activity_hash: Option<u32>,
    pub display_on_item_preview_screen: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemMetricBlockDefinition {
    pub available_metric_category_node_hashes: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemGearsetBlockDefinition {
    pub tracking_value_max: i32,
    pub item_list: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSackBlockDefinition {
    pub detail_action: String,
    pub open_action: String,
    pub seed_unlock_value_hash: u32,
    pub resolved_bit_vector_unlock_value_hash: u32,
    pub resolved_item_count_unlock_value_hash: u32,
    pub select_item_count: i32,
    pub roll_state_unlock_value_hash: u32,
    pub reward_item_list_hash: u32,
    pub vendor_sack_type: Option<String>,
    pub open_on_acquire: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSocketBlockDefinition {
    pub detail: String,
    pub socket_entries: Vec<DestinyItemSocketEntryDefinition>,
    pub intrinsic_sockets: Vec<DestinyItemIntrinsicSocketEntryDefinition>,
    pub socket_categories: Vec<DestinyItemSocketCategoryDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSocketEntryDefinition {
    pub socket_type_hash: u32,
    pub single_initial_item_hash: u32,
    pub reusable_plug_items: Vec<DestinyItemSocketEntryPlugItemDefinition>,
    pub prevent_initialization_on_vendor_purchase: bool,
    pub prevent_initialization_when_versioning: bool,
    pub hide_perks_in_item_tooltip: bool,
    pub plug_sources: SocketPlugSources,
    pub reusable_plug_set_hash: Option<u32>,
    pub randomized_plug_set_hash: Option<u32>,
    pub overrides_ui_appearance: bool,
    pub default_visible: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSocketEntryPlugItemDefinition {
    pub plug_item_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemIntrinsicSocketEntryDefinition {
    pub plug_item_hash: u32,
    pub socket_type_hash: u32,
    pub default_visible: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSocketCategoryDefinition {
    pub socket_category_hash: u32,
    pub socket_indexes: Vec<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSummaryBlockDefinition {
    pub sort_priority: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemTalentGridBlockDefinition {
    pub talent_grid_hash: u32,
    pub item_detail_string: Option<String>,
    pub build_name: Option<String>,
    pub hud_damage_type: DamageType,
    pub hud_icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemInvestmentStatDefinition {
    pub stat_type_hash: u32,
    pub value: i32,
    pub is_conditionally_active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemPerkEntryDefinition {
    pub requirement_display_string: String,
    pub perk_hash: u32,
    pub perk_visibility: ItemPerkVisibility,
}
