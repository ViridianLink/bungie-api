use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyLoadoutsComponent {
    pub loadouts: Vec<DestinyLoadoutComponent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyLoadoutComponent {
    pub color_hash: u32,
    pub icon_hash: u32,
    pub name_hash: u32,
    pub items: Vec<DestinyLoadoutItemComponent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyLoadoutItemComponent {
    #[serde(with = "crate::serde_as::int64")]
    pub item_instance_id: i64,
    pub plug_item_hashes: Vec<u32>,
}
