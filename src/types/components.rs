use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// A single profile component. `data` is absent when the component is private
/// to the caller or disabled.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ComponentResponse<T> {
    pub data: Option<T>,
    pub privacy: ComponentPrivacySetting,
    #[serde(default)]
    pub disabled: bool,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize_repr, Serialize_repr,
)]
pub enum ComponentPrivacySetting {
    None = 0,
    Public = 1,
    Private = 2,
}
