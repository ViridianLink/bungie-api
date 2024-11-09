use std::collections::HashMap;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyManifest {
    pub version: String,
    pub mobile_asset_content_path: String,
    pub mobile_gear_asset_data_bases: Vec<GearAssetDataBaseDefinition>,
    pub mobile_world_content_paths: HashMap<String, String>,
    pub json_world_content_paths: HashMap<String, String>,
    pub json_world_component_content_paths: HashMap<String, HashMap<String, String>>,
    pub mobile_clan_banner_database_path: String,
    pub mobile_gear_c_d_n: HashMap<String, String>,
    pub icon_image_pyramid_info: Vec<ImagePyramidEntry>,
}

#[derive(Deserialize, Serialize)]
pub struct GearAssetDataBaseDefinition {
    pub version: i32,
    pub path: String,
}

#[derive(Deserialize, Serialize)]
pub struct ImagePyramidEntry {
    pub name: String,
    pub factor: f32,
}

bitflags! {
    pub struct EquippingItemBlockAttributes: u8 {
        const EQUIP_ON_ACQUIRE = 1;
    }
}

impl<'de> Deserialize<'de> for EquippingItemBlockAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(EquippingItemBlockAttributes::from_bits_truncate(value))
    }
}

impl Serialize for EquippingItemBlockAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

pub enum DestinyAmmunitionType {
    None = 0,
    Primary = 1,
    Special = 2,
    Heavy = 3,
    Unknown = 4,
}

impl<'de> Deserialize<'de> for DestinyAmmunitionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(DestinyAmmunitionType::None),
            1 => Ok(DestinyAmmunitionType::Primary),
            2 => Ok(DestinyAmmunitionType::Special),
            3 => Ok(DestinyAmmunitionType::Heavy),
            4 => Ok(DestinyAmmunitionType::Unknown),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

impl Serialize for DestinyAmmunitionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DestinyAmmunitionType::None => 0,
            DestinyAmmunitionType::Primary => 1,
            DestinyAmmunitionType::Special => 2,
            DestinyAmmunitionType::Heavy => 3,
            DestinyAmmunitionType::Unknown => 4,
        };
        serializer.serialize_u32(value)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DyeReference {
    pub channel_hash: u32,
    pub dye_hash: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemQuantity {
    pub item_hash: u32,
    pub item_instance_id: Option<i64>,
    pub quantity: i32,
    pub has_conditional_visibility: bool,
}

bitflags! {
    pub struct PlugUiStyles: u8 {
        const MASTERWORK = 1;
    }
}

impl<'de> Deserialize<'de> for PlugUiStyles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(PlugUiStyles::from_bits_truncate(value))
    }
}

impl Serialize for PlugUiStyles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

pub enum PlugAvailabilityMode {
    Normal = 0,
    UnavailableIfSocketContainsMatchingPlugCategory = 1,
    AvailableIfSocketContainsMatchingPlugCategory = 2,
}

impl<'de> Deserialize<'de> for PlugAvailabilityMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(PlugAvailabilityMode::Normal),
            1 => Ok(PlugAvailabilityMode::UnavailableIfSocketContainsMatchingPlugCategory),
            2 => Ok(PlugAvailabilityMode::AvailableIfSocketContainsMatchingPlugCategory),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

impl Serialize for PlugAvailabilityMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            PlugAvailabilityMode::Normal => 0,
            PlugAvailabilityMode::UnavailableIfSocketContainsMatchingPlugCategory => 1,
            PlugAvailabilityMode::AvailableIfSocketContainsMatchingPlugCategory => 2,
        };
        serializer.serialize_u32(value)
    }
}

pub enum DestinyEnergyType {
    Any = 0,
    Arc = 1,
    Thermal = 2,
    Void = 3,
    Ghost = 4,
    Subclass = 5,
    Stasis = 6,
}

impl<'de> Deserialize<'de> for DestinyEnergyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(DestinyEnergyType::Any),
            1 => Ok(DestinyEnergyType::Arc),
            2 => Ok(DestinyEnergyType::Thermal),
            3 => Ok(DestinyEnergyType::Void),
            4 => Ok(DestinyEnergyType::Ghost),
            5 => Ok(DestinyEnergyType::Subclass),
            6 => Ok(DestinyEnergyType::Stasis),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

impl Serialize for DestinyEnergyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DestinyEnergyType::Any => 0,
            DestinyEnergyType::Arc => 1,
            DestinyEnergyType::Thermal => 2,
            DestinyEnergyType::Void => 3,
            DestinyEnergyType::Ghost => 4,
            DestinyEnergyType::Subclass => 5,
            DestinyEnergyType::Stasis => 6,
        };
        serializer.serialize_u32(value)
    }
}

bitflags! {
    pub struct SocketPlugSources : u8 {
        const INVENTORY_SOURCED = 1;
        const REUSABLE_PLUG_ITEMS = 2;
        const PROFILE_PLUG_SET = 4;
        const CHARACTER_PLUG_SET = 8;
    }
}

impl<'de> Deserialize<'de> for SocketPlugSources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(SocketPlugSources::from_bits_truncate(value))
    }
}

impl Serialize for SocketPlugSources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

pub enum DamageType {
    None = 0,
    Kinetic = 1,
    Arc = 2,
    Thermal = 3,
    Void = 4,
    Raid = 5,
    Stasis = 6,
    Strand = 7,
}

impl<'de> Deserialize<'de> for DamageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(DamageType::None),
            1 => Ok(DamageType::Kinetic),
            2 => Ok(DamageType::Arc),
            3 => Ok(DamageType::Thermal),
            4 => Ok(DamageType::Void),
            5 => Ok(DamageType::Raid),
            6 => Ok(DamageType::Stasis),
            7 => Ok(DamageType::Strand),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

impl Serialize for DamageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DamageType::None => 0,
            DamageType::Kinetic => 1,
            DamageType::Arc => 2,
            DamageType::Thermal => 3,
            DamageType::Void => 4,
            DamageType::Raid => 5,
            DamageType::Stasis => 6,
            DamageType::Strand => 7,
        };
        serializer.serialize_u32(value)
    }
}

pub enum ItemPerkVisibility {
    Visible = 0,
    Disabled = 1,
    Hidden = 2,
}

impl<'de> Deserialize<'de> for ItemPerkVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(ItemPerkVisibility::Visible),
            1 => Ok(ItemPerkVisibility::Disabled),
            2 => Ok(ItemPerkVisibility::Hidden),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

impl Serialize for ItemPerkVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            ItemPerkVisibility::Visible => 0,
            ItemPerkVisibility::Disabled => 1,
            ItemPerkVisibility::Hidden => 2,
        };
        serializer.serialize_u32(value)
    }
}

pub enum SpecialItemType {
    None = 0,
    SpecialCurrency = 1,
    Armor = 8,
    Weapon = 9,
    Engram = 23,
    Consumable = 24,
    ExchangeMaterial = 25,
    MissionReward = 27,
    Currency = 29,
}

impl<'de> Deserialize<'de> for SpecialItemType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(SpecialItemType::None),
            1 => Ok(SpecialItemType::SpecialCurrency),
            8 => Ok(SpecialItemType::Armor),
            9 => Ok(SpecialItemType::Weapon),
            23 => Ok(SpecialItemType::Engram),
            24 => Ok(SpecialItemType::Consumable),
            25 => Ok(SpecialItemType::ExchangeMaterial),
            27 => Ok(SpecialItemType::MissionReward),
            29 => Ok(SpecialItemType::Currency),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

impl Serialize for SpecialItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            SpecialItemType::None => 0,
            SpecialItemType::SpecialCurrency => 1,
            SpecialItemType::Armor => 8,
            SpecialItemType::Weapon => 9,
            SpecialItemType::Engram => 23,
            SpecialItemType::Consumable => 24,
            SpecialItemType::ExchangeMaterial => 25,
            SpecialItemType::MissionReward => 27,
            SpecialItemType::Currency => 29,
        };
        serializer.serialize_u32(value)
    }
}

pub enum DestinyItemType {
    None = 0,
    Currency = 1,
    Armor = 2,
    Weapon = 3,
    Message = 7,
    Engram = 8,
    Consumable = 9,
    ExchangeMaterial = 10,
    MissionReward = 11,
    QuestStep = 12,
    QuestStepComplete = 13,
    Emblem = 14,
    Quest = 15,
    Subclass = 16,
    ClanBanner = 17,
    Aura = 18,
    Mod = 19,
    Dummy = 20,
    Ship = 21,
    Vehicle = 22,
    Emote = 23,
    Ghost = 24,
    Package = 25,
    Bounty = 26,
    Wrapper = 27,
    SeasonalArtifact = 28,
    Finisher = 29,
    Pattern = 30,
}

impl<'de> Deserialize<'de> for DestinyItemType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(DestinyItemType::None),
            1 => Ok(DestinyItemType::Currency),
            2 => Ok(DestinyItemType::Armor),
            3 => Ok(DestinyItemType::Weapon),
            7 => Ok(DestinyItemType::Message),
            8 => Ok(DestinyItemType::Engram),
            9 => Ok(DestinyItemType::Consumable),
            10 => Ok(DestinyItemType::ExchangeMaterial),
            11 => Ok(DestinyItemType::MissionReward),
            12 => Ok(DestinyItemType::QuestStep),
            13 => Ok(DestinyItemType::QuestStepComplete),
            14 => Ok(DestinyItemType::Emblem),
            15 => Ok(DestinyItemType::Quest),
            16 => Ok(DestinyItemType::Subclass),
            17 => Ok(DestinyItemType::ClanBanner),
            18 => Ok(DestinyItemType::Aura),
            19 => Ok(DestinyItemType::Mod),
            20 => Ok(DestinyItemType::Dummy),
            21 => Ok(DestinyItemType::Ship),
            22 => Ok(DestinyItemType::Vehicle),
            23 => Ok(DestinyItemType::Emote),
            24 => Ok(DestinyItemType::Ghost),
            25 => Ok(DestinyItemType::Package),
            26 => Ok(DestinyItemType::Bounty),
            27 => Ok(DestinyItemType::Wrapper),
            28 => Ok(DestinyItemType::SeasonalArtifact),
            29 => Ok(DestinyItemType::Finisher),
            30 => Ok(DestinyItemType::Pattern),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

impl Serialize for DestinyItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DestinyItemType::None => 0,
            DestinyItemType::Currency => 1,
            DestinyItemType::Armor => 2,
            DestinyItemType::Weapon => 3,
            DestinyItemType::Message => 7,
            DestinyItemType::Engram => 8,
            DestinyItemType::Consumable => 9,
            DestinyItemType::ExchangeMaterial => 10,
            DestinyItemType::MissionReward => 11,
            DestinyItemType::QuestStep => 12,
            DestinyItemType::QuestStepComplete => 13,
            DestinyItemType::Emblem => 14,
            DestinyItemType::Quest => 15,
            DestinyItemType::Subclass => 16,
            DestinyItemType::ClanBanner => 17,
            DestinyItemType::Aura => 18,
            DestinyItemType::Mod => 19,
            DestinyItemType::Dummy => 20,
            DestinyItemType::Ship => 21,
            DestinyItemType::Vehicle => 22,
            DestinyItemType::Emote => 23,
            DestinyItemType::Ghost => 24,
            DestinyItemType::Package => 25,
            DestinyItemType::Bounty => 26,
            DestinyItemType::Wrapper => 27,
            DestinyItemType::SeasonalArtifact => 28,
            DestinyItemType::Finisher => 29,
            DestinyItemType::Pattern => 30,
        };
        serializer.serialize_u32(value)
    }
}

pub enum DestinyItemSubType {
    None = 0,
    Crucible = 1,
    Vanguard = 2,
    Exotic = 5,
    AutoRifle = 6,
    Shotgun = 7,
    Machinegun = 8,
    HandCannon = 9,
    RocketLauncher = 10,
    FusionRifle = 11,
    SniperRifle = 12,
    PulseRifle = 13,
    ScoutRifle = 14,
    Crm = 16,
    Sidearm = 17,
    Sword = 18,
    Mask = 19,
    Shader = 20,
    Ornament = 21,
    FusionRifleLine = 22,
    GrenadeLauncher = 23,
    SubmachineGun = 24,
    TraceRifle = 25,
    HelmetArmor = 26,
    GauntletsArmor = 27,
    ChestArmor = 28,
    LegArmor = 29,
    ClassArmor = 30,
    Bow = 31,
    DummyRepeatableBounty = 32,
    Glaive = 33,
}

impl<'de> Deserialize<'de> for DestinyItemSubType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(DestinyItemSubType::None),
            1 => Ok(DestinyItemSubType::Crucible),
            2 => Ok(DestinyItemSubType::Vanguard),
            5 => Ok(DestinyItemSubType::Exotic),
            6 => Ok(DestinyItemSubType::AutoRifle),
            7 => Ok(DestinyItemSubType::Shotgun),
            8 => Ok(DestinyItemSubType::Machinegun),
            9 => Ok(DestinyItemSubType::HandCannon),
            10 => Ok(DestinyItemSubType::RocketLauncher),
            11 => Ok(DestinyItemSubType::FusionRifle),
            12 => Ok(DestinyItemSubType::SniperRifle),
            13 => Ok(DestinyItemSubType::PulseRifle),
            14 => Ok(DestinyItemSubType::ScoutRifle),
            16 => Ok(DestinyItemSubType::Crm),
            17 => Ok(DestinyItemSubType::Sidearm),
            18 => Ok(DestinyItemSubType::Sword),
            19 => Ok(DestinyItemSubType::Mask),
            20 => Ok(DestinyItemSubType::Shader),
            21 => Ok(DestinyItemSubType::Ornament),
            22 => Ok(DestinyItemSubType::FusionRifleLine),
            23 => Ok(DestinyItemSubType::GrenadeLauncher),
            24 => Ok(DestinyItemSubType::SubmachineGun),
            25 => Ok(DestinyItemSubType::TraceRifle),
            26 => Ok(DestinyItemSubType::HelmetArmor),
            27 => Ok(DestinyItemSubType::GauntletsArmor),
            28 => Ok(DestinyItemSubType::ChestArmor),
            29 => Ok(DestinyItemSubType::LegArmor),
            30 => Ok(DestinyItemSubType::ClassArmor),
            31 => Ok(DestinyItemSubType::Bow),
            32 => Ok(DestinyItemSubType::DummyRepeatableBounty),
            33 => Ok(DestinyItemSubType::Glaive),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

impl Serialize for DestinyItemSubType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DestinyItemSubType::None => 0,
            DestinyItemSubType::Crucible => 1,
            DestinyItemSubType::Vanguard => 2,
            DestinyItemSubType::Exotic => 5,
            DestinyItemSubType::AutoRifle => 6,
            DestinyItemSubType::Shotgun => 7,
            DestinyItemSubType::Machinegun => 8,
            DestinyItemSubType::HandCannon => 9,
            DestinyItemSubType::RocketLauncher => 10,
            DestinyItemSubType::FusionRifle => 11,
            DestinyItemSubType::SniperRifle => 12,
            DestinyItemSubType::PulseRifle => 13,
            DestinyItemSubType::ScoutRifle => 14,
            DestinyItemSubType::Crm => 16,
            DestinyItemSubType::Sidearm => 17,
            DestinyItemSubType::Sword => 18,
            DestinyItemSubType::Mask => 19,
            DestinyItemSubType::Shader => 20,
            DestinyItemSubType::Ornament => 21,
            DestinyItemSubType::FusionRifleLine => 22,
            DestinyItemSubType::GrenadeLauncher => 23,
            DestinyItemSubType::SubmachineGun => 24,
            DestinyItemSubType::TraceRifle => 25,
            DestinyItemSubType::HelmetArmor => 26,
            DestinyItemSubType::GauntletsArmor => 27,
            DestinyItemSubType::ChestArmor => 28,
            DestinyItemSubType::LegArmor => 29,
            DestinyItemSubType::ClassArmor => 30,
            DestinyItemSubType::Bow => 31,
            DestinyItemSubType::DummyRepeatableBounty => 32,
            DestinyItemSubType::Glaive => 33,
        };
        serializer.serialize_u32(value)
    }
}

pub enum DestinyClass {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
    Unknown = 3,
}

impl<'de> Deserialize<'de> for DestinyClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(DestinyClass::Titan),
            1 => Ok(DestinyClass::Hunter),
            2 => Ok(DestinyClass::Warlock),
            3 => Ok(DestinyClass::Unknown),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

impl Serialize for DestinyClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DestinyClass::Titan => 0,
            DestinyClass::Hunter => 1,
            DestinyClass::Warlock => 2,
            DestinyClass::Unknown => 3,
        };
        serializer.serialize_u32(value)
    }
}

pub enum DestinyBreakerType {
    None = 0,
    ShieldPiercing = 1,
    Disruption = 2,
    Stagger = 3,
}

impl<'de> Deserialize<'de> for DestinyBreakerType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(DestinyBreakerType::None),
            1 => Ok(DestinyBreakerType::ShieldPiercing),
            2 => Ok(DestinyBreakerType::Disruption),
            3 => Ok(DestinyBreakerType::Stagger),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

impl Serialize for DestinyBreakerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DestinyBreakerType::None => 0,
            DestinyBreakerType::ShieldPiercing => 1,
            DestinyBreakerType::Disruption => 2,
            DestinyBreakerType::Stagger => 3,
        };
        serializer.serialize_u32(value)
    }
}
