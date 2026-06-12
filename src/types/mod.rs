use std::collections::HashMap;

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
    BungieNext = 254,
    All = -1,
}

impl<'de> Deserialize<'de> for BungieMembershipType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = i32::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::None),
            1 => Ok(Self::TigerXbox),
            2 => Ok(Self::TigerPsn),
            3 => Ok(Self::TigerSteam),
            4 => Ok(Self::TigerBlizzard),
            5 => Ok(Self::TigerStadia),
            6 => Ok(Self::TigerEgs),
            10 => Ok(Self::TigerDemon),
            254 => Ok(Self::BungieNext),
            -1 => Ok(Self::All),
            _ => Err(serde::de::Error::custom(format!(
                "unknown BungieMembershipType: {s}"
            ))),
        }
    }
}

impl Serialize for BungieMembershipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = *self as i16;
        s.serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyBaseItemComponentSetOfuint32 {
    pub objectives: ComponentResponse<HashMap<u32, DestinyItemObjectivesComponent>>,
    pub perks: ComponentResponse<HashMap<u32, DestinyItemPerksComponent>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemComponentSetOfint64 {
    pub instances: ComponentResponse<HashMap<i64, DestinyItemInstanceComponent>>,
    pub render_data: ComponentResponse<HashMap<i64, DestinyItemRenderComponent>>,
    pub stats: ComponentResponse<HashMap<i64, DestinyItemStatsComponent>>,
    pub sockets: ComponentResponse<HashMap<i64, DestinyItemSocketsComponent>>,
    pub reusable_plugs:
        ComponentResponse<HashMap<i64, DestinyItemReusablePlugsComponent>>,
    pub plug_objectives:
        ComponentResponse<HashMap<i64, DestinyItemPlugObjectivesComponent>>,
    pub talent_grids:
        ComponentResponse<HashMap<i64, DestinyItemTalentGridComponent>>,
    pub plug_states: ComponentResponse<HashMap<u32, DestinyItemPlugComponent>>,
    pub objectives: ComponentResponse<HashMap<i64, DestinyItemObjectivesComponent>>,
    pub perks: ComponentResponse<HashMap<i64, DestinyItemPerksComponent>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum TierType {
    #[default]
    Unknown = 0,
    Currency = 1,
    Basic = 2,
    Common = 3,
    Rare = 4,
    Superior = 5,
    Exotic = 6,
}

impl<'de> Deserialize<'de> for TierType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Currency),
            2 => Ok(Self::Basic),
            3 => Ok(Self::Common),
            4 => Ok(Self::Rare),
            5 => Ok(Self::Superior),
            6 => Ok(Self::Exotic),
            _ => Err(serde::de::Error::custom(format!("unknown TierType: {s}"))),
        }
    }
}

impl Serialize for TierType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::Unknown => 0,
            Self::Currency => 1,
            Self::Basic => 2,
            Self::Common => 3,
            Self::Rare => 4,
            Self::Superior => 5,
            Self::Exotic => 6,
        };
        s.serialize(serializer)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ItemLocation {
    Unknown = 0,
    Inventory = 1,
    Vault = 2,
    Vendor = 3,
    Postmaster = 4,
}

impl<'de> Deserialize<'de> for ItemLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Inventory),
            2 => Ok(Self::Vault),
            3 => Ok(Self::Vendor),
            4 => Ok(Self::Postmaster),
            _ => Err(serde::de::Error::custom(format!("unknown ItemLocation: {s}"))),
        }
    }
}

impl Serialize for ItemLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::Unknown => 0,
            Self::Inventory => 1,
            Self::Vault => 2,
            Self::Vendor => 3,
            Self::Postmaster => 4,
        };
        s.serialize(serializer)
    }
}
