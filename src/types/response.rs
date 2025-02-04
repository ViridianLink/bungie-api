use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::exceptions::PlatformErrorCodes;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct BungieResponse<T> {
    pub response: T,
    pub error_code: PlatformErrorCodes,
    pub throttle_seconds: i32,
    pub error_status: String,
    pub message: String,
    pub message_data: HashMap<String, String>,
    pub detailed_error_trace: Option<String>,
}
