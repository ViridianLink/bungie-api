use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{DestinyItemQuantity, DestinyVendorItemRefundPolicy};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyVendorReceipt {
    pub currency_paid: Vec<DestinyItemQuantity>,
    pub item_received: DestinyItemQuantity,
    pub license_unlock_hash: u32,
    #[serde(with = "crate::serde_as::int64")]
    pub purchased_by_character_id: i64,
    pub refund_policy: DestinyVendorItemRefundPolicy,
    pub sequence_number: i32,
    #[serde(with = "crate::serde_as::int64")]
    pub time_to_expiration: i64,
    pub expires_on: Timestamp,
}
