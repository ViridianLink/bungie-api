use serde::{Deserialize, Serialize};

use super::items::DestinyItemComponent;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyInventoryComponent {
    pub items: Vec<DestinyItemComponent>,
}
