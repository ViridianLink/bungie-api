use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPerkReference {
    pub perk_hash: u32,
    pub icon_path: String,
    pub is_active: bool,
    pub visible: bool,
}
