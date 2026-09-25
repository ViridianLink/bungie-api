use serde::{Deserialize, Serialize};

use super::quests::DestinyObjectiveProgress;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DestinyChallengeStatus {
    pub objective: DestinyObjectiveProgress,
}
