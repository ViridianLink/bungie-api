use std::collections::HashMap;

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::items::DestinyItemPerksComponent;
use crate::types::BungieMembershipType;
use crate::types::destiny::artifacts::DestinyArtifactCharacterScoped;
use crate::types::destiny::character::{
    DestinyCharacterCustomization,
    DestinyCharacterPeerView,
};
use crate::types::destiny::definitions::DestinyActivityInteractableReference;
use crate::types::destiny::historical_stats::definitions::DestinyActivityModeType;
use crate::types::destiny::milestones::DestinyMilestone;
use crate::types::destiny::progression::DestinyFactionProgression;
use crate::types::destiny::quests::{DestinyObjectiveProgress, DestinyQuestStatus};
use crate::types::destiny::{
    DestinyActivity,
    DestinyActivityDifficultyTierCollectionComponent,
    DestinyActivitySelectableSkullCollectionComponent,
    DestinyClass,
    DestinyGender,
    DestinyProgression,
    DestinyRace,
    DyeReference,
};
use crate::types::misc::DestinyColor;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyCharacterComponent {
    #[serde(with = "crate::serde_as::int64")]
    pub membership_id: i64,
    pub membership_type: BungieMembershipType,
    #[serde(with = "crate::serde_as::int64")]
    pub character_id: i64,
    pub date_last_played: Timestamp,
    #[serde(with = "crate::serde_as::int64")]
    pub minutes_played_this_session: i64,
    #[serde(with = "crate::serde_as::int64")]
    pub minutes_played_total: i64,
    pub light: i32,
    pub stats: HashMap<u32, i32>,
    pub race_hash: u32,
    pub gender_hash: u32,
    pub class_hash: u32,
    pub race_type: DestinyRace,
    pub class_type: DestinyClass,
    pub gender_type: DestinyGender,
    pub emblem_path: String,
    pub emblem_background_path: String,
    pub emblem_hash: u32,
    pub emblem_color: DestinyColor,
    pub level_progression: DestinyProgression,
    pub base_character_level: i32,
    pub percent_to_next_level: f32,
    pub title_record_hash: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyCharacterProgressionComponent {
    pub progressions: HashMap<u32, DestinyProgression>,
    pub factions: HashMap<u32, DestinyFactionProgression>,
    pub milestones: HashMap<u32, DestinyMilestone>,
    pub quests: Vec<DestinyQuestStatus>,
    pub uninstanced_item_objectives: HashMap<u32, Vec<DestinyObjectiveProgress>>,
    pub uninstanced_item_perks: HashMap<u32, DestinyItemPerksComponent>,
    pub checklists: HashMap<u32, HashMap<u32, bool>>,
    pub seasonal_artifact: DestinyArtifactCharacterScoped,
    #[serde(default)]
    pub unclaimed_order_rewards: HashMap<u32, i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyCharacterRenderComponent {
    pub custom_dyes: Vec<DyeReference>,
    pub customization: DestinyCharacterCustomization,
    pub peer_view: DestinyCharacterPeerView,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyCharacterActivitiesComponent {
    pub date_activity_started: Timestamp,
    pub available_activities: Vec<DestinyActivity>,
    pub current_activity_hash: u32,
    pub current_activity_mode_hash: u32,
    pub current_activity_mode_type: i32,
    pub current_activity_mode_hashes: Vec<u32>,
    pub current_activity_mode_types: Vec<DestinyActivityModeType>,
    pub current_playlist_activity_hash: u32,
    pub last_completed_story_hash: u32,
    #[serde(default)]
    pub available_activity_interactables: Vec<DestinyActivityInteractableReference>,
    #[serde(default)]
    pub difficulty_tier_collections:
        HashMap<u32, DestinyActivityDifficultyTierCollectionComponent>,
    #[serde(default)]
    pub selectable_skull_collections:
        HashMap<u32, DestinyActivitySelectableSkullCollectionComponent>,
}
