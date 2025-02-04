use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyCraftablesComponent {
    pub craftables: HashMap<u32, DestinyCraftableComponent>,
    pub crafting_root_node_hash: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyCraftableComponent {
    pub visible: bool,
    pub failed_requirement_indexes: Vec<i32>,
    pub sockets: Vec<DestinyCraftableSocketComponent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyCraftableSocketComponent {
    pub plug_set_hash: u32,
    pub plugs: Vec<DestinyCraftableSocketPlugComponent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyCraftableSocketPlugComponent {
    pub plug_item_hash: u32,
    pub failed_requirement_indexes: Vec<i32>,
}
