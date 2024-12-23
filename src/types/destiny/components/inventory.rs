use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::destiny::entities::items::DestinyItemComponent;
use crate::types::BungieMembershipType;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlatformSilverComponent {
    pub platform_silver: HashMap<BungieMembershipType, DestinyItemComponent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyCurrenciesComponent {
    pub item_quantities: HashMap<u32, i32>,
}
