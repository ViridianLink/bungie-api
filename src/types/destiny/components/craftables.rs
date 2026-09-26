use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyCraftablesComponent {
    pub craftables: HashMap<u32, DestinyCraftableComponent>,
    pub crafting_root_node_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyCraftableComponent {
    pub visible: bool,
    pub failed_requirement_indexes: Vec<i32>,
    pub sockets: Vec<DestinyCraftableSocketComponent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyCraftableSocketComponent {
    pub plug_set_hash: u32,
    #[serde(default)]
    pub plugs: Vec<DestinyCraftableSocketPlugComponent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyCraftableSocketPlugComponent {
    pub plug_item_hash: u32,
    pub failed_requirement_indexes: Vec<i32>,
}
