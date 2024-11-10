use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyAnimationReference {
    pub anim_name: String,
    pub anim_identifier: String,
    pub path: String,
}
