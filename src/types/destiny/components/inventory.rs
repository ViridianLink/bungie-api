use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::BungieMembershipType;
use crate::types::destiny::entities::items::DestinyItemComponent;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyPlatformSilverComponent {
    pub platform_silver: HashMap<BungieMembershipType, DestinyItemComponent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyCurrenciesComponent {
    pub item_quantities: HashMap<u32, i32>,
}
