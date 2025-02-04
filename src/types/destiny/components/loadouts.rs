use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyLoadoutsComponent {
    pub loadouts: Vec<DestinyLoadoutComponent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyLoadoutComponent {
    pub color_hash: u32,
    pub icon_hash: u32,
    pub name_hash: u32,
    pub items: Vec<DestinyLoadoutItemComponent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyLoadoutItemComponent {
    pub item_instance_id: i64,
    pub plug_item_hashes: Vec<u32>,
}
