use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::exceptions::PlatformErrorCodes;
use crate::{BungieApiError, Result};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct BungieResponse<T> {
    pub response: Option<T>,
    pub error_code: PlatformErrorCodes,
    pub throttle_seconds: i32,
    pub error_status: String,
    pub message: String,
    #[serde(default)]
    pub message_data: HashMap<String, String>,
    pub detailed_error_trace: Option<String>,
}

impl<T> BungieResponse<T> {
    pub fn into_result(self) -> Result<T> {
        if self.error_code == PlatformErrorCodes::Success {
            self.response.ok_or(BungieApiError::MissingResponse)
        } else {
            Err(self.into_error())
        }
    }

    pub(crate) fn into_error(self) -> BungieApiError {
        BungieApiError::Bungie {
            code: self.error_code,
            status: self.error_status,
            message: self.message,
            throttle_seconds: self.throttle_seconds,
        }
    }
}
