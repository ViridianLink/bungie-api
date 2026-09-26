use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyDisplayPropertiesDefinition {
    pub icon_hash: u32,
    pub description: String,
    pub name: String,
    pub icon: Option<String>,
    #[serde(default)]
    pub icon_sequences: Vec<DestinyIconSequenceDefinition>,
    pub high_res_icon: Option<String>,
    pub has_icon: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyIconSequenceDefinition {
    pub frames: Vec<String>,
}
