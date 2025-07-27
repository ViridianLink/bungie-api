use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::destiny::DestinyGameVersions;
use crate::types::destiny::vendors::DestinyVendorReceipt;
use crate::types::user::UserInfoCard;

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyVendorReceiptsComponent {
    pub receipts: Vec<DestinyVendorReceipt>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DestinyProfileComponent {
    pub user_info: UserInfoCard,
    pub date_last_played: DateTime<Utc>,
    pub versions_owned: DestinyGameVersions,
    pub character_ids: Vec<String>,
    pub season_hashes: Vec<u32>,
    pub event_card_hashes_owned: Vec<u32>,
    pub current_season_hash: u32,
    pub current_season_reward_power_cap: i32,
    pub active_event_card_hash: u32,
    pub current_guardian_rank: i32,
    pub lifetime_highest_guardian_rank: i32,
}
