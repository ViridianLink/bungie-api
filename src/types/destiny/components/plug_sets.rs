use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::destiny::sockets::DestinyItemPlug;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyPlugSetsComponent {
    pub plugs: HashMap<u32, Vec<DestinyItemPlug>>,
}
