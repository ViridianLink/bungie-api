use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::ItemLocation;
use crate::types::destiny::perks::DestinyPerkReference;
use crate::types::destiny::quests::DestinyObjectiveProgress;
use crate::types::destiny::{
    DamageType, DestinyEnergyType, DestinyProgression, DestinyStat, DestinyTalentNode,
    EquipFailureReason, ItemBindStatus, ItemState, TransferStatuses,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemComponent {
    pub item_hash: u32,
    pub item_instance_id: i64,
    pub quantity: i32,
    pub bind_status: ItemBindStatus,
    pub location: ItemLocation,
    pub bucket_hash: u32,
    pub transfer_status: TransferStatuses,
    pub lockable: bool,
    pub state: ItemState,
    pub override_style_item_hash: u32,
    pub expiration_date: DateTime<Utc>,
    pub is_wrapper: bool,
    pub tooltip_notification_indexes: Vec<i32>,
    pub metric_hash: u32,
    pub metric_objective: DestinyObjectiveProgress,
    pub version_number: i32,
    pub item_value_visibility: Vec<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyItemPerksComponent {
    pub perks: Option<Vec<DestinyPerkReference>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemObjectivesComponent {
    pub objectives: Vec<DestinyObjectiveProgress>,
    pub flavor_objective: DestinyObjectiveProgress,
    pub date_completed: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemInstanceComponent {
    pub damage_type: DamageType,
    pub damage_type_hash: u32,
    pub primary_stat: DestinyStat,
    pub item_level: i32,
    pub quality: i32,
    pub is_equipped: bool,
    pub can_equip: bool,
    pub equip_required_level: i32,
    pub unlock_hashes_required_to_equip: Vec<u32>,
    pub cannot_equip_reason: EquipFailureReason,
    pub breaker_type: i32,
    pub breaker_type_hash: u32,
    pub energy: DestinyItemInstanceEnergy,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemInstanceEnergy {
    pub energy_type_hash: u32,
    pub energy_type: DestinyEnergyType,
    pub energy_capacity: i32,
    pub energy_used: i32,
    pub energy_unused: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemRenderComponent {
    pub use_custom_dyes: bool,
    pub art_regions: HashMap<i32, i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyItemStatsComponent {
    pub stats: HashMap<u32, DestinyStat>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyItemSocketsComponent {
    pub sockets: Vec<DestinyItemSocketState>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemSocketState {
    pub plug_hash: u32,
    pub is_enabled: bool,
    pub is_visible: bool,
    pub enable_fail_indexes: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemTalentGridComponent {
    pub talent_grid_hash: u32,
    pub nodes: Vec<DestinyTalentNode>,
    pub is_grid_complete: bool,
    pub grid_progression: DestinyProgression,
}
