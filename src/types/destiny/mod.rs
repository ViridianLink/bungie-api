pub mod artifacts;
pub mod challenges;
pub mod character;
pub mod components;
pub mod config;
pub mod definitions;
pub mod entities;
pub mod historical_stats;
pub mod milestones;
pub mod perks;
pub mod progression;
pub mod quests;
pub mod responses;
pub mod sockets;
pub mod vendors;

use std::collections::HashMap;

use bitflags::bitflags;
use challenges::DestinyChallengeStatus;
use definitions::DestinyMaterialRequirement;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyProgression {
    pub progression_hash: u32,
    pub daily_progress: i32,
    pub daily_limit: i32,
    pub weekly_progress: i32,
    pub weekly_limit: i32,
    pub current_progress: i32,
    pub level: i32,
    pub level_cap: i32,
    pub step_index: i32,
    pub progress_to_next_level: i32,
    pub next_level_at: i32,
    pub current_reset_count: i32,
    pub season_resets: Vec<DestinyProgressionResetEntry>,
    pub reward_item_states: Vec<DestinyProgressionRewardItemState>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyProgressionResetEntry {
    pub season: i32,
    pub resets: i32,
}

bitflags! {
    #[derive(Debug)]
    pub struct DestinyProgressionRewardItemState: u8 {
        const Invisible = 1;
        const Earned = 2;
        const Claimed = 4;
        const ClaimAllowed = 8;
    }
}

impl<'de> Deserialize<'de> for DestinyProgressionRewardItemState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for DestinyProgressionRewardItemState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyProgressionScope {
    Account = 0,
    Character = 1,
    Clan = 2,
    Item = 3,
    ImplicitFromEquipment = 4,
    Mapped = 5,
    MappedAggregate = 6,
    MappedStat = 7,
    MappedUnlockValue = 8,
}

impl<'de> Deserialize<'de> for DestinyProgressionScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Account),
            1 => Ok(Self::Character),
            2 => Ok(Self::Clan),
            3 => Ok(Self::Item),
            4 => Ok(Self::ImplicitFromEquipment),
            5 => Ok(Self::Mapped),
            6 => Ok(Self::MappedAggregate),
            7 => Ok(Self::MappedStat),
            8 => Ok(Self::MappedUnlockValue),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyProgressionScope variant: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyProgressionScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyProgressionStepDisplayEffect {
    None = 0,
    Character = 1,
    Item = 2,
}

impl<'de> Deserialize<'de> for DestinyProgressionStepDisplayEffect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::None),
            1 => Ok(Self::Character),
            2 => Ok(Self::Item),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyProgressionStepDisplayEffect variant: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyProgressionStepDisplayEffect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyItemQuantity {
    pub item_hash: u32,
    pub item_instance_id: Option<i64>,
    pub quantity: i32,
    pub has_conditional_visibility: bool,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SocketTypeActionType {
    InsertPlug = 0,
    InfuseItem = 1,
    ReinitializeSocket = 2,
}

impl<'de> Deserialize<'de> for SocketTypeActionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::InsertPlug),
            1 => Ok(Self::InfuseItem),
            2 => Ok(Self::ReinitializeSocket),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown SocketTypeActionType variant: {s}"
            ))),
        }
    }
}

impl Serialize for SocketTypeActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinySocketVisibility {
    Visible = 0,
    Hidden = 1,
    HiddenWhenEmpty = 2,
    HiddenIfNoPlugsAvailable = 3,
}

impl<'de> Deserialize<'de> for DestinySocketVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Visible),
            1 => Ok(Self::Hidden),
            2 => Ok(Self::HiddenWhenEmpty),
            3 => Ok(Self::HiddenIfNoPlugsAvailable),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinySocketVisibility variant: {s}"
            ))),
        }
    }
}

impl Serialize for DestinySocketVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinySocketCategoryStyle {
    Unknown = 0,
    Reusable = 1,
    Consumable = 2,
    Unlockable = 3,
    Intrinsic = 4,
    EnergyMeter = 5,
    LargePerk = 6,
    Abilities = 7,
    Supers = 8,
}

impl<'de> Deserialize<'de> for DestinySocketCategoryStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Reusable),
            2 => Ok(Self::Consumable),
            3 => Ok(Self::Unlockable),
            4 => Ok(Self::Intrinsic),
            5 => Ok(Self::EnergyMeter),
            6 => Ok(Self::LargePerk),
            7 => Ok(Self::Abilities),
            8 => Ok(Self::Supers),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinySocketCategoryStyle variant: {s}"
            ))),
        }
    }
}

impl Serialize for DestinySocketCategoryStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
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
            _ => Err(serde::de::Error::custom(format!(
                "Unknown TierType variant: {s}"
            ))),
        }
    }
}

impl Serialize for TierType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BucketScope {
    Character = 0,
    Account = 1,
}

impl<'de> Deserialize<'de> for BucketScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Character),
            1 => Ok(Self::Account),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown BucketScope variant: {s}"
            ))),
        }
    }
}

impl Serialize for BucketScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BucketCategory {
    Invisible = 0,
    Item = 1,
    Currency = 2,
    Equippable = 3,
    Ignored = 4,
}

impl<'de> Deserialize<'de> for BucketCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Invisible),
            1 => Ok(Self::Item),
            2 => Ok(Self::Currency),
            3 => Ok(Self::Equippable),
            4 => Ok(Self::Ignored),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown BucketCategory variant: {s}"
            ))),
        }
    }
}

impl Serialize for BucketCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
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
            _ => Err(serde::de::Error::custom(format!(
                "Unknown ItemLocation variant: {s}"
            ))),
        }
    }
}

impl Serialize for ItemLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyStatAggregationType {
    CharacterAverage = 0,
    Character = 1,
    Item = 2,
}

impl<'de> Deserialize<'de> for DestinyStatAggregationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::CharacterAverage),
            1 => Ok(Self::Character),
            2 => Ok(Self::Item),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyStatAggregationType variant: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyStatAggregationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyStatCategory {
    Gameplay = 0,
    Weapon = 1,
    Defense = 2,
    Primary = 3,
}

impl<'de> Deserialize<'de> for DestinyStatCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Gameplay),
            1 => Ok(Self::Weapon),
            2 => Ok(Self::Defense),
            3 => Ok(Self::Primary),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyStatCategory variant: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyStatCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct EquippingItemBlockAttributes: u8 {
        const EquipOnAcquire = 1;
    }
}

impl<'de> Deserialize<'de> for EquippingItemBlockAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for EquippingItemBlockAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::None),
            1 => Ok(Self::Primary),
            2 => Ok(Self::Special),
            3 => Ok(Self::Heavy),
            4 => Ok(Self::Unknown),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyAmmunitionType variant: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyAmmunitionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DyeReference {
    pub channel_hash: u32,
    pub dye_hash: u32,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
pub enum DestinyClass {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
    #[default]
    Unknown = 3,
}

impl<'de> Deserialize<'de> for DestinyClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Titan),
            1 => Ok(Self::Hunter),
            2 => Ok(Self::Warlock),
            3 => Ok(Self::Unknown),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyClass variant: {s}",
            ))),
        }
    }
}

impl Serialize for DestinyClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyGender {
    Male = 0,
    Female = 1,
    Unknown = 2,
}

impl<'de> Deserialize<'de> for DestinyGender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Male),
            1 => Ok(Self::Female),
            2 => Ok(Self::Unknown),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyGender variant: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyGender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyVendorItemRefundPolicy {
    NotRefundable = 0,
    DeletesItem = 1,
    RevokesLicense = 2,
}

impl<'de> Deserialize<'de> for DestinyVendorItemRefundPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::NotRefundable),
            1 => Ok(Self::DeletesItem),
            2 => Ok(Self::RevokesLicense),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyVendorItemRefundPolicy variant: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyVendorItemRefundPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
pub enum DamageType {
    #[default]
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
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::None),
            1 => Ok(Self::Kinetic),
            2 => Ok(Self::Arc),
            3 => Ok(Self::Thermal),
            4 => Ok(Self::Void),
            5 => Ok(Self::Raid),
            6 => Ok(Self::Stasis),
            7 => Ok(Self::Strand),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DamageType variant: {s}"
            ))),
        }
    }
}

impl Serialize for DamageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
pub enum DestinyItemSubType {
    #[default]
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
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::None),
            1 => Ok(Self::Crucible),
            2 => Ok(Self::Vanguard),
            5 => Ok(Self::Exotic),
            6 => Ok(Self::AutoRifle),
            7 => Ok(Self::Shotgun),
            8 => Ok(Self::Machinegun),
            9 => Ok(Self::HandCannon),
            10 => Ok(Self::RocketLauncher),
            11 => Ok(Self::FusionRifle),
            12 => Ok(Self::SniperRifle),
            13 => Ok(Self::PulseRifle),
            14 => Ok(Self::ScoutRifle),
            16 => Ok(Self::Crm),
            17 => Ok(Self::Sidearm),
            18 => Ok(Self::Sword),
            19 => Ok(Self::Mask),
            20 => Ok(Self::Shader),
            21 => Ok(Self::Ornament),
            22 => Ok(Self::FusionRifleLine),
            23 => Ok(Self::GrenadeLauncher),
            24 => Ok(Self::SubmachineGun),
            25 => Ok(Self::TraceRifle),
            26 => Ok(Self::HelmetArmor),
            27 => Ok(Self::GauntletsArmor),
            28 => Ok(Self::ChestArmor),
            29 => Ok(Self::LegArmor),
            30 => Ok(Self::ClassArmor),
            31 => Ok(Self::Bow),
            32 => Ok(Self::DummyRepeatableBounty),
            33 => Ok(Self::Glaive),
            _ => Err(serde::de::Error::custom(format!(
                "Could not deserialize u8 '{s}' to DestinyItemSubType"
            ))),
        }
    }
}

impl Serialize for DestinyItemSubType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct PlugUiStyles: u8 {
        const Masterwork = 1;
    }
}

impl<'de> Deserialize<'de> for PlugUiStyles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for PlugUiStyles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PlugAvailabilityMode {
    Normal = 0,
    UnavailableIfSocketContainsMatchingPlugCategory = 1,
    AvailableIfSocketContainsMatchingPlugCategory = 2,
}

impl<'de> Deserialize<'de> for PlugAvailabilityMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Normal),
            1 => Ok(Self::UnavailableIfSocketContainsMatchingPlugCategory),
            2 => Ok(Self::AvailableIfSocketContainsMatchingPlugCategory),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown PlugAvailabilityMode: {s}"
            ))),
        }
    }
}

impl Serialize for PlugAvailabilityMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Any),
            1 => Ok(Self::Arc),
            2 => Ok(Self::Thermal),
            3 => Ok(Self::Void),
            4 => Ok(Self::Ghost),
            5 => Ok(Self::Subclass),
            6 => Ok(Self::Stasis),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyEnergyType: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyEnergyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct SocketPlugSources: u8 {
        const InventorySourced = 1;
        const ReusablePlugItems = 2;
        const ProfilePlugSet = 4;
        const CharacterPlugSet = 8;
    }
}

impl<'de> Deserialize<'de> for SocketPlugSources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for SocketPlugSources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ItemPerkVisibility {
    Visible = 0,
    Disabled = 1,
    Hidden = 2,
}

impl<'de> Deserialize<'de> for ItemPerkVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Visible),
            1 => Ok(Self::Disabled),
            2 => Ok(Self::Hidden),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown ItemPerkVisibility: {s}"
            ))),
        }
    }
}

impl Serialize for ItemPerkVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
pub enum SpecialItemType {
    #[default]
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
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::None),
            1 => Ok(Self::SpecialCurrency),
            8 => Ok(Self::Armor),
            9 => Ok(Self::Weapon),
            23 => Ok(Self::Engram),
            24 => Ok(Self::Consumable),
            25 => Ok(Self::ExchangeMaterial),
            27 => Ok(Self::MissionReward),
            29 => Ok(Self::Currency),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown SpecialItemType: {s}"
            ))),
        }
    }
}

impl Serialize for SpecialItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
pub enum DestinyItemType {
    #[default]
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
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::None),
            1 => Ok(Self::Currency),
            2 => Ok(Self::Armor),
            3 => Ok(Self::Weapon),
            7 => Ok(Self::Message),
            8 => Ok(Self::Engram),
            9 => Ok(Self::Consumable),
            10 => Ok(Self::ExchangeMaterial),
            11 => Ok(Self::MissionReward),
            12 => Ok(Self::QuestStep),
            13 => Ok(Self::QuestStepComplete),
            14 => Ok(Self::Emblem),
            15 => Ok(Self::Quest),
            16 => Ok(Self::Subclass),
            17 => Ok(Self::ClanBanner),
            18 => Ok(Self::Aura),
            19 => Ok(Self::Mod),
            20 => Ok(Self::Dummy),
            21 => Ok(Self::Ship),
            22 => Ok(Self::Vehicle),
            23 => Ok(Self::Emote),
            24 => Ok(Self::Ghost),
            25 => Ok(Self::Package),
            26 => Ok(Self::Bounty),
            27 => Ok(Self::Wrapper),
            28 => Ok(Self::SeasonalArtifact),
            29 => Ok(Self::Finisher),
            30 => Ok(Self::Pattern),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyItemType: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
pub enum DestinyBreakerType {
    #[default]
    None = 0,
    ShieldPiercing = 1,
    Disruption = 2,
    Stagger = 3,
}

impl<'de> Deserialize<'de> for DestinyBreakerType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::None),
            1 => Ok(Self::ShieldPiercing),
            2 => Ok(Self::Disruption),
            3 => Ok(Self::Stagger),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyBreakerType: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyBreakerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyProgressionRewardItemAcquisitionBehavior {
    Instant = 0,
    PlayerClaimRequired = 1,
}

impl<'de> Deserialize<'de> for DestinyProgressionRewardItemAcquisitionBehavior {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Instant),
            1 => Ok(Self::PlayerClaimRequired),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyProgressionRewardItemAcquisitionBehavior: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyProgressionRewardItemAcquisitionBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ItemBindStatus {
    NotBound = 0,
    BoundToCharacter = 1,
    BoundToAccount = 2,
    BoundToGuild = 3,
}

impl<'de> Deserialize<'de> for ItemBindStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::NotBound),
            1 => Ok(Self::BoundToCharacter),
            2 => Ok(Self::BoundToAccount),
            3 => Ok(Self::BoundToGuild),
            _ => {
                Err(serde::de::Error::custom(format!("Unknown ItemBindStatus: {s}")))
            },
        }
    }
}

impl Serialize for ItemBindStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TransferStatuses: u8 {
        const ItemIsEquipped = 1;
        const NotTransferrable = 2;
        const NoRoomInDestination = 4;
    }
}

impl<'de> Deserialize<'de> for TransferStatuses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for TransferStatuses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ItemState: u8 {
        const Locked = 1;
        const Tracked = 2;
        const Masterwork = 4;
        const Crafted = 8;
        const HighlightedObjective = 16;
    }
}

impl<'de> Deserialize<'de> for ItemState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for ItemState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DestinyGameVersions: u16 {
        const Destiny2 = 1;
        const DLC1 = 2;
        const DLC2 = 4;
        const Forsaken = 8;
        const YearTwoAnnualPass = 16;
        const Shadowkeep = 32;
        const BeyondLight = 64;
        const Anniversary30th = 128;
        const TheWitchQueen = 256;
        const Lightfall = 512;
    }
}

impl<'de> Deserialize<'de> for DestinyGameVersions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u16::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for DestinyGameVersions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyComponentType {
    None = 0,
    Profiles = 100,
    VendorReceipts = 101,
    ProfileInventories = 102,
    ProfileCurrencies = 103,
    ProfileProgression = 104,
    PlatformSilver = 105,
    Characters = 200,
    CharacterInventories = 201,
    CharacterProgressions = 202,
    CharacterRenderData = 203,
    CharacterActivities = 204,
    CharacterEquipment = 205,
    CharacterLoadouts = 206,
    ItemInstances = 300,
    ItemObjectives = 301,
    ItemPerks = 302,
    ItemRenderData = 303,
    ItemStats = 304,
    ItemSockets = 305,
    ItemTalentGrids = 306,
    ItemCommonData = 307,
    ItemPlugStates = 308,
    ItemPlugObjectives = 309,
    ItemReusablePlugs = 310,
    Vendors = 400,
    VendorCategories = 401,
    VendorSales = 402,
    Kiosks = 500,
    CurrencyLookups = 600,
    PresentationNodes = 700,
    Collectibles = 800,
    Records = 900,
    Transitory = 1000,
    Metrics = 1100,
    StringVariables = 1200,
    Craftables = 1300,
    SocialCommendations = 1400,
}

impl<'de> Deserialize<'de> for DestinyComponentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u16::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::None),
            100 => Ok(Self::Profiles),
            101 => Ok(Self::VendorReceipts),
            102 => Ok(Self::ProfileInventories),
            103 => Ok(Self::ProfileCurrencies),
            104 => Ok(Self::ProfileProgression),
            105 => Ok(Self::PlatformSilver),
            200 => Ok(Self::Characters),
            201 => Ok(Self::CharacterInventories),
            202 => Ok(Self::CharacterProgressions),
            203 => Ok(Self::CharacterRenderData),
            204 => Ok(Self::CharacterActivities),
            205 => Ok(Self::CharacterEquipment),
            206 => Ok(Self::CharacterLoadouts),
            300 => Ok(Self::ItemInstances),
            301 => Ok(Self::ItemObjectives),
            302 => Ok(Self::ItemPerks),
            303 => Ok(Self::ItemRenderData),
            304 => Ok(Self::ItemStats),
            305 => Ok(Self::ItemSockets),
            306 => Ok(Self::ItemTalentGrids),
            307 => Ok(Self::ItemCommonData),
            308 => Ok(Self::ItemPlugStates),
            309 => Ok(Self::ItemPlugObjectives),
            310 => Ok(Self::ItemReusablePlugs),
            400 => Ok(Self::Vendors),
            401 => Ok(Self::VendorCategories),
            402 => Ok(Self::VendorSales),
            500 => Ok(Self::Kiosks),
            600 => Ok(Self::CurrencyLookups),
            700 => Ok(Self::PresentationNodes),
            800 => Ok(Self::Collectibles),
            900 => Ok(Self::Records),
            1000 => Ok(Self::Transitory),
            1100 => Ok(Self::Metrics),
            1200 => Ok(Self::StringVariables),
            1300 => Ok(Self::Craftables),
            1400 => Ok(Self::SocialCommendations),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyComponentType: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyComponentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u16).serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DestinyPresentationNodeState: u8 {
        const Invisible = 1;
        const Obscured = 2;
    }
}

impl<'de> Deserialize<'de> for DestinyPresentationNodeState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for DestinyPresentationNodeState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DestinyRecordState: u8 {
        const RecordRedeemed = 1;
        const RewardUnavailable = 2;
        const ObjectiveNotCompleted = 4;
        const Obscured = 8;
        const Invisible = 16;
        const EntitlementUnowned = 32;
        const CanEquipTitle = 64;
    }
}

impl<'de> Deserialize<'de> for DestinyRecordState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for DestinyRecordState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DestinyCollectibleState : u8 {
        const NotAcquired = 1;
        const Obscured = 2;
        const Invisible = 4;
        const CannotAffordMaterialRequirements = 8;
        const InventorySpaceUnavailable = 16;
        const UniquenessViolation = 32;
        const PurchaseDisabled = 64;
    }
}

impl<'de> Deserialize<'de> for DestinyCollectibleState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for DestinyCollectibleState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DestinyPartyMemberStates: u8 {
        const FireteamMember = 1;
        const PosseMember = 2;
        const GroupMember = 4;
        const PartyLeader = 8;
    }
}

impl<'de> Deserialize<'de> for DestinyPartyMemberStates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for DestinyPartyMemberStates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyGamePrivacySetting {
    Open = 0,
    ClanAndFriendsOnly = 1,
    FriendsOnly = 2,
    InvitationOnly = 3,
    Closed = 4,
}

impl<'de> Deserialize<'de> for DestinyGamePrivacySetting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Open),
            1 => Ok(Self::ClanAndFriendsOnly),
            2 => Ok(Self::FriendsOnly),
            3 => Ok(Self::InvitationOnly),
            4 => Ok(Self::Closed),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyGamePrivacySetting: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyGamePrivacySetting {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DestinyJoinClosedReasons: u16 {
        const InMatchmaking = 1;
        const Loading = 2;
        const SoloMode = 4;
        const InternalReasons = 8;
        const DisallowedByGameState = 16;
        const Offline = 32768;
    }
}

impl<'de> Deserialize<'de> for DestinyJoinClosedReasons {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u16::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for DestinyJoinClosedReasons {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyRace {
    Human = 0,
    Awoken = 1,
    Exo = 2,
    Unknown = 3,
}

impl<'de> Deserialize<'de> for DestinyRace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Human),
            1 => Ok(Self::Awoken),
            2 => Ok(Self::Exo),
            3 => Ok(Self::Unknown),
            _ => Err(serde::de::Error::custom(format!("Unknown DestinyRace: {s}"))),
        }
    }
}

impl Serialize for DestinyRace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyActivity {
    pub activity_hash: u32,
    pub is_new: bool,
    pub can_lead: bool,
    pub can_join: bool,
    pub is_completed: bool,
    pub is_visible: bool,
    pub display_level: Option<i32>,
    pub recommended_light: Option<i32>,
    pub difficulty_tier: DestinyActivityDifficultyTier,
    pub challenges: Vec<DestinyChallengeStatus>,
    pub modifier_hashes: Vec<u32>,
    pub boolean_activity_options: HashMap<u32, bool>,
    pub loadout_requirement_index: i32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyActivityDifficultyTier {
    Trivial = 0,
    Easy = 1,
    Normal = 2,
    Challenging = 3,
    Hard = 4,
    Brave = 5,
    AlmostImpossible = 6,
    Impossible = 7,
}

impl<'de> Deserialize<'de> for DestinyActivityDifficultyTier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(Self::Trivial),
            1 => Ok(Self::Easy),
            2 => Ok(Self::Normal),
            3 => Ok(Self::Challenging),
            4 => Ok(Self::Hard),
            5 => Ok(Self::Brave),
            6 => Ok(Self::AlmostImpossible),
            7 => Ok(Self::Impossible),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyActivityDifficultyTier: {s}"
            ))),
        }
    }
}

impl Serialize for DestinyActivityDifficultyTier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyStat {
    pub stat_hash: u32,
    pub value: i32,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct EquipFailureReason: u8 {
        const ItemUnequippable = 1;
        const ItemUniqueEquipRestricted = 2;
        const ItemFailedUnlockCheck = 4;
        const ItemFailedLevelCheck = 8;
        const ItemWrapped = 16;
        const ItemNotLoaded = 32;
        const ItemEquipBlocklisted = 64;
        const ItemLoadoutRequirementNotMet = 128;
    }
}

impl<'de> Deserialize<'de> for EquipFailureReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(s))
    }
}

impl Serialize for EquipFailureReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bits().serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyTalentNode {
    pub node_index: i32,
    pub node_hash: u32,
    pub state: DestinyTalentNodeState,
    pub is_activated: bool,
    pub step_index: i32,
    pub materials_to_upgrade: Vec<DestinyMaterialRequirement>,
    pub activation_grid_level: i32,
    pub progress_percent: f32,
    pub hidden: bool,
    pub node_stats_block: DestinyTalentNodeStatBlock,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DestinyTalentNodeState {
    Invalid = 0,
    CanUpgrade = 1,
    NoPoints = 2,
    NoPrerequisites = 3,
    NoSteps = 4,
    NoUnlock = 5,
    NoMaterial = 6,
    NoGridLevel = 7,
    SwappingLocked = 8,
    MustSwap = 9,
    Complete = 10,
    Unknown = 11,
    CreationOnly = 12,
    Hidden = 13,
}

impl<'de> Deserialize<'de> for DestinyTalentNodeState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(match s {
            0 => Self::Invalid,
            1 => Self::CanUpgrade,
            2 => Self::NoPoints,
            3 => Self::NoPrerequisites,
            4 => Self::NoSteps,
            5 => Self::NoUnlock,
            6 => Self::NoMaterial,
            7 => Self::NoGridLevel,
            8 => Self::SwappingLocked,
            9 => Self::MustSwap,
            10 => Self::Complete,
            11 => Self::Unknown,
            12 => Self::CreationOnly,
            13 => Self::Hidden,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Unknown DestinyTalentNodeState: {s}"
                )));
            },
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyTalentNodeStatBlock {
    pub current_step_stats: Vec<DestinyStat>,
    pub next_step_stats: Vec<DestinyStat>,
}

impl Serialize for DestinyTalentNodeState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyUnlockStatus {
    pub unlock_hash: u32,
    pub is_set: bool,
}
