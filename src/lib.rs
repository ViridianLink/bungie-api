mod bungie_client;

pub mod endpoints;
mod error;
pub mod serde_as;
use std::collections::HashMap;

pub use error::Error;
use error::Result;

pub mod types;

pub use bungie_client::{BungieClient, BungieClientBuilder};
pub use types::definitions::DestinyInventoryItemDefinition;
pub use types::destiny::definitions::sockets::{
    DestinyPlugSetDefinition, DestinySocketCategoryDefinition, DestinySocketTypeDefinition,
};

pub type DestinyInventoryItemManifest = HashMap<String, DestinyInventoryItemDefinition>;
pub type DestinyPlugSetManifest = HashMap<String, DestinyPlugSetDefinition>;
pub type DestinySocketCategoryManifest = HashMap<String, DestinySocketCategoryDefinition>;
pub type DestinySocketTypeManifest = HashMap<String, DestinySocketTypeDefinition>;

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::types::destiny::historical_stats::definitions::DestinyActivityModeType;
    use crate::types::destiny::DestinyComponentType;
    use crate::BungieClientBuilder;

    const BUNGIE_API_KEY: &str = "";

    #[tokio::test]
    async fn run() {
        let client = BungieClientBuilder::new(BUNGIE_API_KEY).build().unwrap();

        let user_info = client
            .search_destiny_player("OscarSix", 7797)
            .await
            .unwrap()
            .pop()
            .unwrap();

        let profle_response = client
            .profile(
                user_info.membership_type,
                user_info.membership_id,
                &[DestinyComponentType::Profiles],
            )
            .await
            .unwrap();

        let character_id = profle_response.profile.unwrap().data.character_ids[0]
            .parse()
            .unwrap();

        let activities = client
            .activity_history(
                user_info.membership_type,
                user_info.membership_id,
                character_id,
                None,
                Some(DestinyActivityModeType::Raid),
                None,
            )
            .await
            .unwrap();

        let pgcr = client
            .post_game_carnage_report(activities.activities[0].activity_details.instance_id)
            .await
            .unwrap();

        fs::write("output.json", serde_json::to_string_pretty(&pgcr).unwrap()).unwrap();
    }
}
