use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serializer, de};

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

macro_rules! serde_repr_enum {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident: $repr:ty {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident = $value:literal
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
        #[repr($repr)]
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                $variant = $value,
            )*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ::serde::Serialize::serialize(&(*self as $repr), serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let value = <$repr as ::serde::Deserialize>::deserialize(deserializer)?;
                match value {
                    $($value => Ok(Self::$variant),)*
                    _ => Err(<D::Error as ::serde::de::Error>::custom(format!(
                        concat!("unknown ", stringify!($name), ": {}"),
                        value
                    ))),
                }
            }
        }
    };
}

pub(crate) use serde_repr_enum;
