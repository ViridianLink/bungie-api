use serde::{Deserialize, Serialize};

use super::quests::DestinyObjectiveProgress;

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyChallengeStatus {
    pub objective: DestinyObjectiveProgress,
}
