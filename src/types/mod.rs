use std::collections::HashMap;

use components::ComponentResponse;
use destiny::{
    components::items::{
        DestinyItemPlugComponent, DestinyItemPlugObjectivesComponent,
        DestinyItemReusablePlugsComponent,
    },
    entities::items::{
        DestinyItemInstanceComponent, DestinyItemObjectivesComponent, DestinyItemPerksComponent,
        DestinyItemRenderComponent, DestinyItemSocketsComponent, DestinyItemStatsComponent,
        DestinyItemTalentGridComponent,
    },
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
    fn deserialize<D>(deserializer: D) -> Result<BungieMembershipType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = i32::deserialize(deserializer)?;
        match s {
            0 => Ok(BungieMembershipType::None),
            1 => Ok(BungieMembershipType::TigerXbox),
            2 => Ok(BungieMembershipType::TigerPsn),
            3 => Ok(BungieMembershipType::TigerSteam),
            4 => Ok(BungieMembershipType::TigerBlizzard),
            5 => Ok(BungieMembershipType::TigerStadia),
            6 => Ok(BungieMembershipType::TigerEgs),
            10 => Ok(BungieMembershipType::TigerDemon),
            254 => Ok(BungieMembershipType::BungieNext),
            -1 => Ok(BungieMembershipType::All),
            _ => Err(serde::de::Error::custom(format!(
                "unknown BungieMembershipType: {}",
                s
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
pub struct DestinyItemComponentSetOfint64 {
    pub instances: ComponentResponse<HashMap<i64, DestinyItemInstanceComponent>>,
    pub render_data: ComponentResponse<HashMap<i64, DestinyItemRenderComponent>>,
    pub stats: ComponentResponse<HashMap<i64, DestinyItemStatsComponent>>,
    pub sockets: ComponentResponse<HashMap<i64, DestinyItemSocketsComponent>>,
    pub reusable_plugs: ComponentResponse<HashMap<i64, DestinyItemReusablePlugsComponent>>,
    pub plug_objectives: ComponentResponse<HashMap<i64, DestinyItemPlugObjectivesComponent>>,
    pub talent_grids: ComponentResponse<HashMap<i64, DestinyItemTalentGridComponent>>,
    pub plug_states: ComponentResponse<HashMap<u32, DestinyItemPlugComponent>>,
    pub objectives: ComponentResponse<HashMap<i64, DestinyItemObjectivesComponent>>,
    pub perks: ComponentResponse<HashMap<i64, DestinyItemPerksComponent>>,
}

#[derive(Debug, Clone, Copy)]
pub enum TierType {
    Unknown = 0,
    Currency = 1,
    Basic = 2,
    Common = 3,
    Rare = 4,
    Superior = 5,
    Exotic = 6,
}

impl<'de> Deserialize<'de> for TierType {
    fn deserialize<D>(deserializer: D) -> Result<TierType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(TierType::Unknown),
            1 => Ok(TierType::Currency),
            2 => Ok(TierType::Basic),
            3 => Ok(TierType::Common),
            4 => Ok(TierType::Rare),
            5 => Ok(TierType::Superior),
            6 => Ok(TierType::Exotic),
            _ => Err(serde::de::Error::custom(format!("unknown TierType: {}", s))),
        }
    }
}

impl Serialize for TierType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            TierType::Unknown => 0,
            TierType::Currency => 1,
            TierType::Basic => 2,
            TierType::Common => 3,
            TierType::Rare => 4,
            TierType::Superior => 5,
            TierType::Exotic => 6,
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
    fn deserialize<D>(deserializer: D) -> Result<ItemLocation, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(ItemLocation::Unknown),
            1 => Ok(ItemLocation::Inventory),
            2 => Ok(ItemLocation::Vault),
            3 => Ok(ItemLocation::Vendor),
            4 => Ok(ItemLocation::Postmaster),
            _ => Err(serde::de::Error::custom(format!(
                "unknown ItemLocation: {}",
                s
            ))),
        }
    }
}

impl Serialize for ItemLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            ItemLocation::Unknown => 0,
            ItemLocation::Inventory => 1,
            ItemLocation::Vault => 2,
            ItemLocation::Vendor => 3,
            ItemLocation::Postmaster => 4,
        };
        s.serialize(serializer)
    }
}
