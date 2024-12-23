use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::destiny::sockets::DestinyItemPlug;

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyPlugSetsComponent {
    pub plugs: HashMap<u32, Vec<DestinyItemPlug>>,
}
