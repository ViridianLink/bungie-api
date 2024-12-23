use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{DestinyItemQuantity, DestinyVendorItemRefundPolicy};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorReceipt {
    pub currency_paid: Vec<DestinyItemQuantity>,
    pub item_received: DestinyItemQuantity,
    pub license_unlock_hash: u32,
    pub purchased_by_character_id: i64,
    pub refund_policy: DestinyVendorItemRefundPolicy,
    pub sequence_number: i32,
    pub time_to_expiration: i64,
    pub expires_on: DateTime<Utc>,
}
