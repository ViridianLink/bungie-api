use serde::{Deserialize, Serialize};

use super::DyeReference;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyCharacterCustomization {
    pub personality: u32,
    pub face: u32,
    pub skin_color: u32,
    pub lip_color: u32,
    pub eye_color: u32,
    pub hair_colors: Vec<u32>,
    pub feature_colors: Vec<u32>,
    pub decal_color: u32,
    pub wear_helmet: bool,
    pub hair_index: i32,
    pub feature_index: i32,
    pub decal_index: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyCharacterPeerView {
    pub equipment: Vec<DestinyItemPeerView>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemPeerView {
    pub item_hash: u32,
    pub dyes: Vec<DyeReference>,
}
