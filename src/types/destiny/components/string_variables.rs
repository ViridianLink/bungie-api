use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyStringVariablesComponent {
    pub integer_values_by_hash: HashMap<u32, i32>,
}
