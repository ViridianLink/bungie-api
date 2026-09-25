//! Serde helpers for the quirks of the Bungie.net JSON format.

use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serializer, de};

/// A JSON integer as Bungie sends it: `int64` and `uint64` values are encoded
/// as strings so JavaScript clients do not lose precision, but plain numbers are
/// accepted too.
#[derive(Deserialize)]
#[serde(untagged)]
enum RawInt {
    Str(String),
    Signed(i64),
    Unsigned(u64),
}

impl RawInt {
    fn parse<T, E>(self) -> Result<T, E>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
        E: de::Error,
    {
        let result = match self {
            Self::Str(s) => s.parse(),
            Self::Signed(n) => n.to_string().parse(),
            Self::Unsigned(n) => n.to_string().parse(),
        };
        result.map_err(E::custom)
    }
}

/// `#[serde(with = "int64")]` for 64-bit integers encoded as JSON strings.
pub(crate) mod int64 {
    use super::{Deserialize, Deserializer, Display, FromStr, RawInt, Serializer};

    pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        RawInt::deserialize(deserializer)?.parse()
    }

    pub(crate) fn serialize<S, T>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        serializer.collect_str(value)
    }
}

/// `#[serde(with = "int64_option", default)]` for optional 64-bit integers
/// encoded as JSON strings.
pub(crate) mod int64_option {
    use super::{Deserialize, Deserializer, Display, FromStr, RawInt, Serializer};

    pub(crate) fn deserialize<'de, D, T>(
        deserializer: D,
    ) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        Option::<RawInt>::deserialize(deserializer)?.map(RawInt::parse).transpose()
    }

    #[expect(
        clippy::ref_option,
        reason = "serde's `with` attribute passes the field by reference"
    )]
    pub(crate) fn serialize<S, T>(
        value: &Option<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        match value {
            Some(value) => serializer.collect_str(value),
            None => serializer.serialize_none(),
        }
    }
}

/// Implements `Serialize` and `Deserialize` for a `bitflags` type using its raw
/// numeric value, which is how Bungie encodes bitmask enums.
///
/// Unknown bits are dropped on deserialization so that flags added to the API
/// later do not cause errors.
macro_rules! impl_bitflags_serde {
    ($name:ident) => {
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                type Bits = <$name as ::bitflags::Flags>::Bits;
                let raw = <u64 as ::serde::Deserialize>::deserialize(deserializer)?;
                let bits = Bits::try_from(raw & u64::from(Bits::MAX))
                    .map_err(<D::Error as ::serde::de::Error>::custom)?;
                Ok(Self::from_bits_truncate(bits))
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ::serde::Serialize::serialize(&self.bits(), serializer)
            }
        }
    };
}

pub(crate) use impl_bitflags_serde;
