mod manifest;

use url::Url;

use crate::types::destiny::historical_stats::definitions::DestinyActivityModeType;
use crate::types::destiny::historical_stats::DestinyActivityHistoryResults;
use crate::types::destiny::responses::DestinyProfileResponse;
use crate::types::destiny::DestinyComponentType;
use crate::types::user::UserInfoCard;
use crate::types::BungieMembershipType;
use crate::{BungieClient, Result};

impl BungieClient {
    pub async fn search_destiny_player(
        &self,
        username: &str,
        discriminator: u16,
    ) -> Result<Vec<UserInfoCard>> {
        let mut url =
            Url::parse("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/-1/")?;

        url.path_segments_mut()
            .expect("Cannot set path segments")
            .push(format!("{}#{}", username, discriminator).as_str());

        self.get_bungie_response::<Vec<UserInfoCard>>(url).await
    }

    pub async fn get_profile(
        &self,
        membership_type: BungieMembershipType,
        membership_id: u64,
        components: &[DestinyComponentType],
    ) -> Result<DestinyProfileResponse> {
        let mut url = Url::parse("https://www.bungie.net/Platform/Destiny2/")?;

        url.path_segments_mut()
            .expect("Cannot set path segments")
            .push(&(membership_type as i16).to_string())
            .push("Profile")
            .push(&membership_id.to_string());

        let components = components
            .iter()
            .copied()
            .map(|c| (c as u16).to_string())
            .collect::<Vec<_>>()
            .join(",");

        url.query_pairs_mut().append_pair("components", &components);

        self.get_bungie_response::<DestinyProfileResponse>(url)
            .await
    }

    pub async fn get_activity_history(
        &self,
        membership_type: BungieMembershipType,
        membership_id: u64,
        character_id: u64,
        count: Option<i32>,
        mode: Option<DestinyActivityModeType>,
        page: Option<i32>,
    ) -> Result<DestinyActivityHistoryResults> {
        let mut url = Url::parse("https://www.bungie.net/Platform/Destiny2/")?;

        url.path_segments_mut()
            .expect("Cannot set path segments")
            .push(&(membership_type as i16).to_string())
            .push("Account")
            .push(&membership_id.to_string())
            .push("Character")
            .push(&character_id.to_string())
            .push("Stats")
            .push("Activities");

        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(count) = count {
                query_pairs.append_pair("count", &count.to_string());
            }
            if let Some(mode) = mode {
                query_pairs.append_pair("mode", &(mode as i32).to_string());
            }
            query_pairs.append_pair("page", &page.unwrap_or(0).to_string());
        }

        self.get_bungie_response::<DestinyActivityHistoryResults>(url)
            .await
    }
}
