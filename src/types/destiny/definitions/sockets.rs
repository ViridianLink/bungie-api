use serde::{Deserialize, Serialize};

use crate::types::common::DestinyDisplayPropertiesDefinition;
use crate::types::destiny::DestinySocketCategoryStyle;

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
