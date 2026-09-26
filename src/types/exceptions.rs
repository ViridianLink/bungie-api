use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PlatformErrorCodes {
    Success,
    Unknown(u32),
}

impl PlatformErrorCodes {
    #[must_use]
    pub const fn code(self) -> u32 {
        match self {
            Self::Success => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u32> for PlatformErrorCodes {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Success,
            _ => Self::Unknown(value),
        }
    }
}

impl<'de> Deserialize<'de> for PlatformErrorCodes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        u32::deserialize(deserializer).map(Self::from)
    }
}

impl Serialize for PlatformErrorCodes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.code().serialize(serializer)
    }
}
