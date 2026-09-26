use std::collections::HashMap;
use std::fmt;

use components::ComponentResponse;
use destiny::components::items::{
    DestinyItemPlugComponent,
    DestinyItemPlugObjectivesComponent,
    DestinyItemReusablePlugsComponent,
};
use destiny::entities::items::{
    DestinyItemInstanceComponent,
    DestinyItemObjectivesComponent,
    DestinyItemPerksComponent,
    DestinyItemRenderComponent,
    DestinyItemSocketsComponent,
    DestinyItemStatsComponent,
    DestinyItemTalentGridComponent,
};
pub use destiny::{ItemLocation, TierType};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod common;
pub mod components;
pub mod definitions;
pub mod destiny;
pub mod exceptions;
pub mod links;
pub mod misc;
pub mod response;
pub mod user;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum BungieMembershipType {
    None = 0,
    TigerXbox = 1,
    TigerPsn = 2,
    TigerSteam = 3,
    TigerBlizzard = 4,
    TigerStadia = 5,
    TigerEgs = 6,
    TigerDemon = 10,
    GoliathGame = 20,
    BungieNext = 254,
    All = -1,
}

impl BungieMembershipType {
    #[must_use]
    pub const fn from_i64(value: i64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::TigerXbox,
            2 => Self::TigerPsn,
            3 => Self::TigerSteam,
            4 => Self::TigerBlizzard,
            5 => Self::TigerStadia,
            6 => Self::TigerEgs,
            10 => Self::TigerDemon,
            20 => Self::GoliathGame,
            254 => Self::BungieNext,
            -1 => Self::All,
            _ => return None,
        })
    }

    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "None" => Self::None,
            "TigerXbox" => Self::TigerXbox,
            "TigerPsn" => Self::TigerPsn,
            "TigerSteam" => Self::TigerSteam,
            "TigerBlizzard" => Self::TigerBlizzard,
            "TigerStadia" => Self::TigerStadia,
            "TigerEgs" => Self::TigerEgs,
            "TigerDemon" => Self::TigerDemon,
            "GoliathGame" => Self::GoliathGame,
            "BungieNext" => Self::BungieNext,
            "All" => Self::All,
            _ => return None,
        })
    }
}

impl<'de> Deserialize<'de> for BungieMembershipType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MembershipTypeVisitor;

        impl Visitor<'_> for MembershipTypeVisitor {
            type Value = BungieMembershipType;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a BungieMembershipType number or name")
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                BungieMembershipType::from_i64(v).ok_or_else(|| {
                    E::custom(format!("unknown BungieMembershipType: {v}"))
                })
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                i64::try_from(v)
                    .ok()
                    .and_then(BungieMembershipType::from_i64)
                    .ok_or_else(|| {
                        E::custom(format!("unknown BungieMembershipType: {v}"))
                    })
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<i64>()
                    .ok()
                    .and_then(BungieMembershipType::from_i64)
                    .or_else(|| BungieMembershipType::from_name(v))
                    .ok_or_else(|| {
                        E::custom(format!("unknown BungieMembershipType: {v}"))
                    })
            }
        }

        deserializer.deserialize_any(MembershipTypeVisitor)
    }
}

impl Serialize for BungieMembershipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as i16).serialize(serializer)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyBaseItemComponentSetOfuint32 {
    pub objectives:
        Option<ComponentResponse<HashMap<u32, DestinyItemObjectivesComponent>>>,
    pub perks: Option<ComponentResponse<HashMap<u32, DestinyItemPerksComponent>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyItemComponentSetOfint64 {
    pub instances:
        Option<ComponentResponse<HashMap<i64, DestinyItemInstanceComponent>>>,
    pub render_data:
        Option<ComponentResponse<HashMap<i64, DestinyItemRenderComponent>>>,
    pub stats: Option<ComponentResponse<HashMap<i64, DestinyItemStatsComponent>>>,
    pub sockets:
        Option<ComponentResponse<HashMap<i64, DestinyItemSocketsComponent>>>,
    pub reusable_plugs:
        Option<ComponentResponse<HashMap<i64, DestinyItemReusablePlugsComponent>>>,
    pub plug_objectives:
        Option<ComponentResponse<HashMap<i64, DestinyItemPlugObjectivesComponent>>>,
    pub talent_grids:
        Option<ComponentResponse<HashMap<i64, DestinyItemTalentGridComponent>>>,
    pub plug_states:
        Option<ComponentResponse<HashMap<u32, DestinyItemPlugComponent>>>,
    pub objectives:
        Option<ComponentResponse<HashMap<i64, DestinyItemObjectivesComponent>>>,
    pub perks: Option<ComponentResponse<HashMap<i64, DestinyItemPerksComponent>>>,
}
