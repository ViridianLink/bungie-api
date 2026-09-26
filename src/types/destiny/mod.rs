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
use definitions::{DestinyActivityRewardMapping, DestinyMaterialRequirement};
use serde::{Deserialize, Serialize};

use crate::serde_as::{impl_bitflags_serde, serde_repr_enum};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
    pub current_reset_count: Option<i32>,
    #[serde(default)]
    pub season_resets: Vec<DestinyProgressionResetEntry>,
    #[serde(default)]
    pub reward_item_states: Vec<DestinyProgressionRewardItemState>,
    #[serde(default)]
    pub reward_item_socket_override_states:
        HashMap<i32, DestinyProgressionRewardItemSocketOverrideState>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyProgressionResetEntry {
    pub season: i32,
    pub resets: i32,
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DestinyProgressionRewardItemState: u8 {
        const Invisible = 1;
        const Earned = 2;
        const Claimed = 4;
        const ClaimAllowed = 8;
    }
}

impl_bitflags_serde!(DestinyProgressionRewardItemState);

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyProgressionScope: u8 {
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
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyProgressionStepDisplayEffect: u8 {
        None = 0,
        Character = 1,
        Item = 2,
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyItemQuantity {
    pub item_hash: u32,
    #[serde(with = "crate::serde_as::int64_option", default)]
    pub item_instance_id: Option<i64>,
    pub quantity: i32,
    pub has_conditional_visibility: bool,
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SocketTypeActionType: u8 {
        InsertPlug = 0,
        InfuseItem = 1,
        ReinitializeSocket = 2,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinySocketVisibility: u8 {
        Visible = 0,
        Hidden = 1,
        HiddenWhenEmpty = 2,
        HiddenIfNoPlugsAvailable = 3,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinySocketCategoryStyle: u8 {
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
}

serde_repr_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TierType: u8 {
        #[default]
        Unknown = 0,
        Currency = 1,
        Basic = 2,
        Common = 3,
        Rare = 4,
        Superior = 5,
        Exotic = 6,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BucketScope: u8 {
        Character = 0,
        Account = 1,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BucketCategory: u8 {
        Invisible = 0,
        Item = 1,
        Currency = 2,
        Equippable = 3,
        Ignored = 4,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ItemLocation: u8 {
        Unknown = 0,
        Inventory = 1,
        Vault = 2,
        Vendor = 3,
        Postmaster = 4,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyStatAggregationType: u8 {
        CharacterAverage = 0,
        Character = 1,
        Item = 2,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyStatCategory: u8 {
        Gameplay = 0,
        Weapon = 1,
        Defense = 2,
        Primary = 3,
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EquippingItemBlockAttributes: u8 {
        const EquipOnAcquire = 1;
    }
}

impl_bitflags_serde!(EquippingItemBlockAttributes);

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyAmmunitionType: u8 {
        None = 0,
        Primary = 1,
        Special = 2,
        Heavy = 3,
        Unknown = 4,
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DyeReference {
    pub channel_hash: u32,
    pub dye_hash: u32,
}

serde_repr_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyClass: u8 {
        Titan = 0,
        Hunter = 1,
        Warlock = 2,
        #[default]
        Unknown = 3,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyGender: u8 {
        Male = 0,
        Female = 1,
        Unknown = 2,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyVendorItemRefundPolicy: u8 {
        NotRefundable = 0,
        DeletesItem = 1,
        RevokesLicense = 2,
    }
}

serde_repr_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DamageType: u8 {
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
}

serde_repr_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyItemSubType: u8 {
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
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PlugUiStyles: u8 {
        const Masterwork = 1;
    }
}

impl_bitflags_serde!(PlugUiStyles);

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PlugAvailabilityMode: u8 {
        Normal = 0,
        UnavailableIfSocketContainsMatchingPlugCategory = 1,
        AvailableIfSocketContainsMatchingPlugCategory = 2,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyEnergyType: u8 {
        Any = 0,
        Arc = 1,
        Thermal = 2,
        Void = 3,
        Ghost = 4,
        Subclass = 5,
        Stasis = 6,
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SocketPlugSources: u8 {
        const InventorySourced = 1;
        const ReusablePlugItems = 2;
        const ProfilePlugSet = 4;
        const CharacterPlugSet = 8;
    }
}

impl_bitflags_serde!(SocketPlugSources);

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ItemPerkVisibility: u8 {
        Visible = 0,
        Disabled = 1,
        Hidden = 2,
    }
}

serde_repr_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SpecialItemType: u8 {
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
}

serde_repr_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyItemType: u8 {
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
}

serde_repr_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyBreakerType: u8 {
        #[default]
        None = 0,
        ShieldPiercing = 1,
        Disruption = 2,
        Stagger = 3,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyProgressionRewardItemAcquisitionBehavior: u8 {
        Instant = 0,
        PlayerClaimRequired = 1,
    }
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ItemBindStatus: u8 {
        NotBound = 0,
        BoundToCharacter = 1,
        BoundToAccount = 2,
        BoundToGuild = 3,
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TransferStatuses: u8 {
        const ItemIsEquipped = 1;
        const NotTransferrable = 2;
        const NoRoomInDestination = 4;
    }
}

impl_bitflags_serde!(TransferStatuses);

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ItemState: u8 {
        const Locked = 1;
        const Tracked = 2;
        const Masterwork = 4;
        const Crafted = 8;
        const HighlightedObjective = 16;
        const Enhanced = 32;
    }
}

impl_bitflags_serde!(ItemState);

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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
        const TheFinalShape = 1024;
        const EdgeOfFate = 2048;
        const Renegades = 4096;
    }
}

impl_bitflags_serde!(DestinyGameVersions);

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyComponentType: u16 {
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
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DestinyPresentationNodeState: u8 {
        const Invisible = 1;
        const Obscured = 2;
    }
}

impl_bitflags_serde!(DestinyPresentationNodeState);

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

impl_bitflags_serde!(DestinyRecordState);

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DestinyCollectibleState: u8 {
        const NotAcquired = 1;
        const Obscured = 2;
        const Invisible = 4;
        const CannotAffordMaterialRequirements = 8;
        const InventorySpaceUnavailable = 16;
        const UniquenessViolation = 32;
        const PurchaseDisabled = 64;
    }
}

impl_bitflags_serde!(DestinyCollectibleState);

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DestinyPartyMemberStates: u8 {
        const FireteamMember = 1;
        const PosseMember = 2;
        const GroupMember = 4;
        const PartyLeader = 8;
    }
}

impl_bitflags_serde!(DestinyPartyMemberStates);

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyGamePrivacySetting: u8 {
        Open = 0,
        ClanAndFriendsOnly = 1,
        FriendsOnly = 2,
        InvitationOnly = 3,
        Closed = 4,
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DestinyJoinClosedReasons: u16 {
        const InMatchmaking = 1;
        const Loading = 2;
        const SoloMode = 4;
        const InternalReasons = 8;
        const DisallowedByGameState = 16;
        const Offline = 32768;
    }
}

impl_bitflags_serde!(DestinyJoinClosedReasons);

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyRace: u8 {
        Human = 0,
        Awoken = 1,
        Exo = 2,
        Unknown = 3,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
    #[serde(default)]
    pub challenges: Vec<DestinyChallengeStatus>,
    #[serde(default)]
    pub modifier_hashes: Vec<u32>,
    #[serde(default)]
    pub boolean_activity_options: HashMap<u32, bool>,
    pub loadout_requirement_index: Option<i32>,
    #[serde(default)]
    pub fireteam_requirement_failure_indices: Vec<i32>,
    #[serde(default)]
    pub is_focused_activity: bool,
    #[serde(default)]
    pub leader_requirement_failure_indices: Vec<i32>,
    #[serde(default)]
    pub visible_rewards: Vec<DestinyActivityRewardMapping>,
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyActivityDifficultyTier: u8 {
        Trivial = 0,
        Easy = 1,
        Normal = 2,
        Challenging = 3,
        Hard = 4,
        Brave = 5,
        AlmostImpossible = 6,
        Impossible = 7,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyStat {
    pub stat_hash: u32,
    pub value: i32,
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

impl_bitflags_serde!(EquipFailureReason);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyTalentNodeState: u8 {
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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyTalentNodeStatBlock {
    pub current_step_stats: Vec<DestinyStat>,
    pub next_step_stats: Vec<DestinyStat>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyUnlockStatus {
    pub unlock_hash: u32,
    pub is_set: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyProgressionRewardItemSocketOverrideState {
    pub reward_item_stats: HashMap<u32, DestinyStat>,
    pub item_state: ItemState,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyActivityDifficultyTierCollectionComponent {
    pub difficulty_tier_collection_hash: u32,
    pub difficulty_tiers: Vec<DestinyActivityDifficultyTierComponent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyActivityDifficultyTierComponent {
    pub difficulty_tier_index: i32,
    pub fixed_activity_skulls: Vec<DestinyActivitySkullComponent>,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyActivitySelectableSkullCollectionComponent {
    pub selectable_skull_collection_hash: u32,
    pub selectable_skulls: Vec<DestinyActivitySkullComponent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyActivitySkullComponent {
    pub hash: u32,
    pub skull_identifier_hash: u32,
    pub is_enabled: bool,
}

serde_repr_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DestinyActivityRewardDisplayMode: u8 {
        Aggregate = 0,
        PickFirst = 1,
        Count = 2,
    }
}
