use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[repr(u32)]
pub enum PlatformErrorCodes {
    Success = 1,
    Unknown(u32),
}

impl<'de> Deserialize<'de> for PlatformErrorCodes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(match value {
            1 => Self::Success,
            _ => Self::Unknown(value),
        })
    }
}

impl Serialize for PlatformErrorCodes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Success => 1u32.serialize(serializer),
            Self::Unknown(value) => value.serialize(serializer),
        }
    }
}
