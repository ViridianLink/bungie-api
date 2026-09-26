use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::types::destiny::DestinyGameVersions;
use crate::types::destiny::vendors::DestinyVendorReceipt;
use crate::types::user::UserInfoCard;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyVendorReceiptsComponent {
    pub receipts: Vec<DestinyVendorReceipt>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyProfileComponent {
    pub user_info: UserInfoCard,
    pub date_last_played: Timestamp,
    pub versions_owned: DestinyGameVersions,
    pub character_ids: Vec<String>,
    pub season_hashes: Vec<u32>,
    pub event_card_hashes_owned: Vec<u32>,
    pub current_season_hash: Option<u32>,
    pub current_season_reward_power_cap: Option<i32>,
    pub active_event_card_hash: Option<u32>,
    pub current_guardian_rank: i32,
    pub lifetime_highest_guardian_rank: i32,
    pub current_season_pass_hash: Option<u32>,
    #[serde(default)]
    pub renewed_guardian_rank: i32,
    #[serde(default)]
    pub season_pass_hashes: Vec<u32>,
}
