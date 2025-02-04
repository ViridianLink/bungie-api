use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyDisplayPropertiesDefinition {
    pub description: String,
    pub name: String,
    pub icon: Option<String>,
    #[serde(default)]
    pub icon_sequences: Vec<DestinyIconSequenceDefinition>,
    pub high_res_icon: Option<String>,
    pub has_icon: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DestinyIconSequenceDefinition {
    pub frames: Vec<String>,
}
