use serde::{Deserialize, Serialize};

use super::BungieMembershipType;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct UserInfoCard {
    pub supplemental_display_name: Option<String>,
    pub icon_path: Option<String>,
    pub cross_save_override: BungieMembershipType,
    #[serde(default)]
    pub applicable_membership_types: Vec<BungieMembershipType>,
    pub is_public: bool,
    pub membership_type: BungieMembershipType,
    #[serde(with = "crate::serde_as::int64")]
    pub membership_id: u64,
    pub display_name: Option<String>,
    pub bungie_global_display_name: Option<String>,
    pub bungie_global_display_name_code: Option<i16>,
}
