use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentResponse<T> {
    pub data: T,
    pub privacy: ComponentPrivacySetting,
    #[serde(default)]
    pub disabled: bool,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ComponentPrivacySetting {
    None = 0,
    Public = 1,
    Private = 2,
}

impl<'de> Deserialize<'de> for ComponentPrivacySetting {
    fn deserialize<D>(deserializer: D) -> Result<ComponentPrivacySetting, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(ComponentPrivacySetting::None),
            1 => Ok(ComponentPrivacySetting::Public),
            2 => Ok(ComponentPrivacySetting::Private),
            _ => Err(serde::de::Error::custom(format!(
                "unknown ComponentPrivacySetting: {}",
                s
            ))),
        }
    }
}

impl Serialize for ComponentPrivacySetting {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = *self as u8;
        s.serialize(serializer)
    }
}
