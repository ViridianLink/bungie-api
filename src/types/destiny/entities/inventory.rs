use serde::{Deserialize, Serialize};

use super::items::DestinyItemComponent;

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyInventoryComponent {
    pub items: Vec<DestinyItemComponent>,
}
