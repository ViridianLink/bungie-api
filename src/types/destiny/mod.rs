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
    fn deserialize<D>(deserializer: D) -> Result<DestinyProgressionRewardItemState, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(DestinyProgressionRewardItemState::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyProgressionScope, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyProgressionScope::Account),
            1 => Ok(DestinyProgressionScope::Character),
            2 => Ok(DestinyProgressionScope::Clan),
            3 => Ok(DestinyProgressionScope::Item),
            4 => Ok(DestinyProgressionScope::ImplicitFromEquipment),
            5 => Ok(DestinyProgressionScope::Mapped),
            6 => Ok(DestinyProgressionScope::MappedAggregate),
            7 => Ok(DestinyProgressionScope::MappedStat),
            8 => Ok(DestinyProgressionScope::MappedUnlockValue),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyProgressionStepDisplayEffect, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyProgressionStepDisplayEffect::None),
            1 => Ok(DestinyProgressionStepDisplayEffect::Character),
            2 => Ok(DestinyProgressionStepDisplayEffect::Item),
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
    fn deserialize<D>(deserializer: D) -> Result<SocketTypeActionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(SocketTypeActionType::InsertPlug),
            1 => Ok(SocketTypeActionType::InfuseItem),
            2 => Ok(SocketTypeActionType::ReinitializeSocket),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinySocketVisibility, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinySocketVisibility::Visible),
            1 => Ok(DestinySocketVisibility::Hidden),
            2 => Ok(DestinySocketVisibility::HiddenWhenEmpty),
            3 => Ok(DestinySocketVisibility::HiddenIfNoPlugsAvailable),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinySocketCategoryStyle, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinySocketCategoryStyle::Unknown),
            1 => Ok(DestinySocketCategoryStyle::Reusable),
            2 => Ok(DestinySocketCategoryStyle::Consumable),
            3 => Ok(DestinySocketCategoryStyle::Unlockable),
            4 => Ok(DestinySocketCategoryStyle::Intrinsic),
            5 => Ok(DestinySocketCategoryStyle::EnergyMeter),
            6 => Ok(DestinySocketCategoryStyle::LargePerk),
            7 => Ok(DestinySocketCategoryStyle::Abilities),
            8 => Ok(DestinySocketCategoryStyle::Supers),
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
    fn deserialize<D>(deserializer: D) -> Result<BucketScope, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(BucketScope::Character),
            1 => Ok(BucketScope::Account),
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
    fn deserialize<D>(deserializer: D) -> Result<BucketCategory, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(BucketCategory::Invisible),
            1 => Ok(BucketCategory::Item),
            2 => Ok(BucketCategory::Currency),
            3 => Ok(BucketCategory::Equippable),
            4 => Ok(BucketCategory::Ignored),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyStatAggregationType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyStatAggregationType::CharacterAverage),
            1 => Ok(DestinyStatAggregationType::Character),
            2 => Ok(DestinyStatAggregationType::Item),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyStatCategory, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyStatCategory::Gameplay),
            1 => Ok(DestinyStatCategory::Weapon),
            2 => Ok(DestinyStatCategory::Defense),
            3 => Ok(DestinyStatCategory::Primary),
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
    fn deserialize<D>(deserializer: D) -> Result<EquippingItemBlockAttributes, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(EquippingItemBlockAttributes::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyAmmunitionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyAmmunitionType::None),
            1 => Ok(DestinyAmmunitionType::Primary),
            2 => Ok(DestinyAmmunitionType::Special),
            3 => Ok(DestinyAmmunitionType::Heavy),
            4 => Ok(DestinyAmmunitionType::Unknown),
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
#[derive(Debug, Clone, Copy)]
pub enum DestinyClass {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
    Unknown = 3,
}

impl<'de> Deserialize<'de> for DestinyClass {
    fn deserialize<D>(deserializer: D) -> Result<DestinyClass, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyClass::Titan),
            1 => Ok(DestinyClass::Hunter),
            2 => Ok(DestinyClass::Warlock),
            3 => Ok(DestinyClass::Unknown),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyGender, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyGender::Male),
            1 => Ok(DestinyGender::Female),
            2 => Ok(DestinyGender::Unknown),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyVendorItemRefundPolicy, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyVendorItemRefundPolicy::NotRefundable),
            1 => Ok(DestinyVendorItemRefundPolicy::DeletesItem),
            2 => Ok(DestinyVendorItemRefundPolicy::RevokesLicense),
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
#[derive(Debug, Clone, Copy)]
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
    fn deserialize<D>(deserializer: D) -> Result<DamageType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DamageType::None),
            1 => Ok(DamageType::Kinetic),
            2 => Ok(DamageType::Arc),
            3 => Ok(DamageType::Thermal),
            4 => Ok(DamageType::Void),
            5 => Ok(DamageType::Raid),
            6 => Ok(DamageType::Stasis),
            7 => Ok(DamageType::Strand),
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
#[derive(Debug, Clone, Copy)]
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyItemSubType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
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
    fn deserialize<D>(deserializer: D) -> Result<PlugUiStyles, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(PlugUiStyles::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<PlugAvailabilityMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(PlugAvailabilityMode::Normal),
            1 => Ok(PlugAvailabilityMode::UnavailableIfSocketContainsMatchingPlugCategory),
            2 => Ok(PlugAvailabilityMode::AvailableIfSocketContainsMatchingPlugCategory),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyEnergyType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyEnergyType::Any),
            1 => Ok(DestinyEnergyType::Arc),
            2 => Ok(DestinyEnergyType::Thermal),
            3 => Ok(DestinyEnergyType::Void),
            4 => Ok(DestinyEnergyType::Ghost),
            5 => Ok(DestinyEnergyType::Subclass),
            6 => Ok(DestinyEnergyType::Stasis),
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
    fn deserialize<D>(deserializer: D) -> Result<SocketPlugSources, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(SocketPlugSources::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<ItemPerkVisibility, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(ItemPerkVisibility::Visible),
            1 => Ok(ItemPerkVisibility::Disabled),
            2 => Ok(ItemPerkVisibility::Hidden),
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
#[derive(Debug, Clone, Copy)]
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
    fn deserialize<D>(deserializer: D) -> Result<SpecialItemType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(SpecialItemType::None),
            1 => Ok(SpecialItemType::SpecialCurrency),
            8 => Ok(SpecialItemType::Armor),
            9 => Ok(SpecialItemType::Weapon),
            23 => Ok(SpecialItemType::Engram),
            24 => Ok(SpecialItemType::Consumable),
            25 => Ok(SpecialItemType::ExchangeMaterial),
            27 => Ok(SpecialItemType::MissionReward),
            29 => Ok(SpecialItemType::Currency),
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
#[derive(Debug, Clone, Copy)]
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyItemType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
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
#[derive(Debug, Clone, Copy)]
pub enum DestinyBreakerType {
    None = 0,
    ShieldPiercing = 1,
    Disruption = 2,
    Stagger = 3,
}

impl<'de> Deserialize<'de> for DestinyBreakerType {
    fn deserialize<D>(deserializer: D) -> Result<DestinyBreakerType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyBreakerType::None),
            1 => Ok(DestinyBreakerType::ShieldPiercing),
            2 => Ok(DestinyBreakerType::Disruption),
            3 => Ok(DestinyBreakerType::Stagger),
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
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<DestinyProgressionRewardItemAcquisitionBehavior, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyProgressionRewardItemAcquisitionBehavior::Instant),
            1 => Ok(DestinyProgressionRewardItemAcquisitionBehavior::PlayerClaimRequired),
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
    fn deserialize<D>(deserializer: D) -> Result<ItemBindStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(ItemBindStatus::NotBound),
            1 => Ok(ItemBindStatus::BoundToCharacter),
            2 => Ok(ItemBindStatus::BoundToAccount),
            3 => Ok(ItemBindStatus::BoundToGuild),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown ItemBindStatus: {s}"
            ))),
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
    fn deserialize<D>(deserializer: D) -> Result<TransferStatuses, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(TransferStatuses::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<ItemState, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(ItemState::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyGameVersions, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u16::deserialize(deserializer)?;
        Ok(DestinyGameVersions::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyComponentType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u16::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyComponentType::None),
            100 => Ok(DestinyComponentType::Profiles),
            101 => Ok(DestinyComponentType::VendorReceipts),
            102 => Ok(DestinyComponentType::ProfileInventories),
            103 => Ok(DestinyComponentType::ProfileCurrencies),
            104 => Ok(DestinyComponentType::ProfileProgression),
            105 => Ok(DestinyComponentType::PlatformSilver),
            200 => Ok(DestinyComponentType::Characters),
            201 => Ok(DestinyComponentType::CharacterInventories),
            202 => Ok(DestinyComponentType::CharacterProgressions),
            203 => Ok(DestinyComponentType::CharacterRenderData),
            204 => Ok(DestinyComponentType::CharacterActivities),
            205 => Ok(DestinyComponentType::CharacterEquipment),
            206 => Ok(DestinyComponentType::CharacterLoadouts),
            300 => Ok(DestinyComponentType::ItemInstances),
            301 => Ok(DestinyComponentType::ItemObjectives),
            302 => Ok(DestinyComponentType::ItemPerks),
            303 => Ok(DestinyComponentType::ItemRenderData),
            304 => Ok(DestinyComponentType::ItemStats),
            305 => Ok(DestinyComponentType::ItemSockets),
            306 => Ok(DestinyComponentType::ItemTalentGrids),
            307 => Ok(DestinyComponentType::ItemCommonData),
            308 => Ok(DestinyComponentType::ItemPlugStates),
            309 => Ok(DestinyComponentType::ItemPlugObjectives),
            310 => Ok(DestinyComponentType::ItemReusablePlugs),
            400 => Ok(DestinyComponentType::Vendors),
            401 => Ok(DestinyComponentType::VendorCategories),
            402 => Ok(DestinyComponentType::VendorSales),
            500 => Ok(DestinyComponentType::Kiosks),
            600 => Ok(DestinyComponentType::CurrencyLookups),
            700 => Ok(DestinyComponentType::PresentationNodes),
            800 => Ok(DestinyComponentType::Collectibles),
            900 => Ok(DestinyComponentType::Records),
            1000 => Ok(DestinyComponentType::Transitory),
            1100 => Ok(DestinyComponentType::Metrics),
            1200 => Ok(DestinyComponentType::StringVariables),
            1300 => Ok(DestinyComponentType::Craftables),
            1400 => Ok(DestinyComponentType::SocialCommendations),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyPresentationNodeState, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(DestinyPresentationNodeState::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyRecordState, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(DestinyRecordState::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyCollectibleState, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(DestinyCollectibleState::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyPartyMemberStates, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(DestinyPartyMemberStates::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyGamePrivacySetting, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyGamePrivacySetting::Open),
            1 => Ok(DestinyGamePrivacySetting::ClanAndFriendsOnly),
            2 => Ok(DestinyGamePrivacySetting::FriendsOnly),
            3 => Ok(DestinyGamePrivacySetting::InvitationOnly),
            4 => Ok(DestinyGamePrivacySetting::Closed),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyJoinClosedReasons, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u16::deserialize(deserializer)?;
        Ok(DestinyJoinClosedReasons::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyRace, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyRace::Human),
            1 => Ok(DestinyRace::Awoken),
            2 => Ok(DestinyRace::Exo),
            3 => Ok(DestinyRace::Unknown),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown DestinyRace: {s}"
            ))),
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyActivityDifficultyTier, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        match s {
            0 => Ok(DestinyActivityDifficultyTier::Trivial),
            1 => Ok(DestinyActivityDifficultyTier::Easy),
            2 => Ok(DestinyActivityDifficultyTier::Normal),
            3 => Ok(DestinyActivityDifficultyTier::Challenging),
            4 => Ok(DestinyActivityDifficultyTier::Hard),
            5 => Ok(DestinyActivityDifficultyTier::Brave),
            6 => Ok(DestinyActivityDifficultyTier::AlmostImpossible),
            7 => Ok(DestinyActivityDifficultyTier::Impossible),
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
    fn deserialize<D>(deserializer: D) -> Result<EquipFailureReason, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(EquipFailureReason::from_bits_truncate(s))
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
    fn deserialize<D>(deserializer: D) -> Result<DestinyTalentNodeState, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = u8::deserialize(deserializer)?;
        Ok(match s {
            0 => DestinyTalentNodeState::Invalid,
            1 => DestinyTalentNodeState::CanUpgrade,
            2 => DestinyTalentNodeState::NoPoints,
            3 => DestinyTalentNodeState::NoPrerequisites,
            4 => DestinyTalentNodeState::NoSteps,
            5 => DestinyTalentNodeState::NoUnlock,
            6 => DestinyTalentNodeState::NoMaterial,
            7 => DestinyTalentNodeState::NoGridLevel,
            8 => DestinyTalentNodeState::SwappingLocked,
            9 => DestinyTalentNodeState::MustSwap,
            10 => DestinyTalentNodeState::Complete,
            11 => DestinyTalentNodeState::Unknown,
            12 => DestinyTalentNodeState::CreationOnly,
            13 => DestinyTalentNodeState::Hidden,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Unknown DestinyTalentNodeState: {s}"
                )));
            }
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
