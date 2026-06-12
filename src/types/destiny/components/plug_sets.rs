use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::destiny::sockets::DestinyItemPlug;

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyPlugSetsComponent {
    pub plugs: HashMap<u32, Vec<DestinyItemPlug>>,
}
