use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::destiny::DestinyCollectibleState;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyCollectiblesComponent {
    pub collectibles: HashMap<u32, DestinyCollectibleComponent>,
    pub collection_categories_root_node_hash: u32,
    pub collection_badges_root_node_hash: u32,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct DestinyCollectibleComponent {
    pub state: DestinyCollectibleState,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProfileCollectiblesComponent {
    pub recent_collectible_hashes: Vec<u32>,
    pub newness_flagged_collectible_hashes: Vec<u32>,
    pub collectibles: HashMap<u32, DestinyCollectibleComponent>,
    pub collection_categories_root_node_hash: u32,
    pub collection_badges_root_node_hash: u32,
}
