use serde::{Deserialize, Serialize};

use crate::serde_as::serde_repr_enum;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ComponentResponse<T> {
    pub data: T,
    pub privacy: ComponentPrivacySetting,
    #[serde(default)]
    pub disabled: bool,
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ComponentPrivacySetting: u8 {
        None = 0,
        Public = 1,
        Private = 2,
    }
}
