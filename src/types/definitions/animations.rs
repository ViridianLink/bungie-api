use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyAnimationReference {
    pub anim_name: String,
    pub anim_identifier: String,
    pub path: String,
}
