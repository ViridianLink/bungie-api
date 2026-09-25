mod manifest;

pub use manifest::DestinyDefinition;
use serde::Serialize;

use crate::types::BungieMembershipType;
use crate::types::destiny::DestinyComponentType;
use crate::types::destiny::historical_stats::definitions::DestinyActivityModeType;
use crate::types::destiny::historical_stats::{
    DestinyActivityHistoryResults,
    DestinyPostGameCarnageReportData,
};
use crate::types::destiny::responses::DestinyProfileResponse;
use crate::types::user::UserInfoCard;
use crate::{BungieClient, Result};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExactSearchRequest<'a> {
    display_name: &'a str,
    display_name_code: u16,
}

impl BungieClient {
    /// Finds the Destiny memberships for a Bungie Name such as `Name#1234`,
    /// across all platforms.
    pub async fn search_destiny_player(
        &self,
        username: &str,
        discriminator: u16,
    ) -> Result<Vec<UserInfoCard>> {
        let url = self.platform_url([
            "Destiny2",
            "SearchDestinyPlayerByBungieName",
            &membership_type_segment(BungieMembershipType::All),
        ])?;

        let body = ExactSearchRequest {
            display_name: username,
            display_name_code: discriminator,
        };

        self.post_bungie_response(url, &body).await
    }

    pub async fn profile(
        &self,
        membership_type: BungieMembershipType,
        membership_id: u64,
        components: &[DestinyComponentType],
    ) -> Result<DestinyProfileResponse> {
        let mut url = self.platform_url([
            "Destiny2",
            &membership_type_segment(membership_type),
            "Profile",
            &membership_id.to_string(),
        ])?;

        let components = components
            .iter()
            .map(|&c| (c as u16).to_string())
            .collect::<Vec<_>>()
            .join(",");

        url.query_pairs_mut().append_pair("components", &components);

        self.get_bungie_response(url).await
    }

    pub async fn activity_history(
        &self,
        membership_type: BungieMembershipType,
        membership_id: u64,
        character_id: u64,
        count: Option<i32>,
        mode: Option<DestinyActivityModeType>,
        page: u32,
    ) -> Result<DestinyActivityHistoryResults> {
        let mut url = self.platform_url([
            "Destiny2",
            &membership_type_segment(membership_type),
            "Account",
            &membership_id.to_string(),
            "Character",
            &character_id.to_string(),
            "Stats",
            "Activities",
        ])?;

        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(count) = count {
                query_pairs.append_pair("count", &count.to_string());
            }
            if let Some(mode) = mode {
                query_pairs.append_pair("mode", &(mode as u8).to_string());
            }
            query_pairs.append_pair("page", &page.to_string());
        }

        self.get_bungie_response(url).await
    }

    pub async fn post_game_carnage_report(
        &self,
        activity_id: u64,
    ) -> Result<DestinyPostGameCarnageReportData> {
        let url = self.platform_url([
            "Destiny2",
            "Stats",
            "PostGameCarnageReport",
            &activity_id.to_string(),
        ])?;

        self.get_bungie_response(url).await
    }
}

fn membership_type_segment(membership_type: BungieMembershipType) -> String {
    (membership_type as i16).to_string()
}

#[cfg(test)]
mod tests {
    use crate::BungieClient;

    #[expect(clippy::unwrap_used, reason = "Unwrap allowed in tests")]
    #[test]
    fn platform_url_has_trailing_slash() {
        let client = BungieClient::new("key").unwrap();
        let url = client.platform_url(["Destiny2", "-1", "Profile", "123"]).unwrap();
        assert_eq!(
            url.as_str(),
            "https://www.bungie.net/Platform/Destiny2/-1/Profile/123/"
        );
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap allowed in tests")]
    #[test]
    fn platform_url_escapes_segments() {
        let client = BungieClient::new("key").unwrap();
        let url = client.platform_url(["a b", "c#d/e"]).unwrap();
        assert_eq!(url.as_str(), "https://www.bungie.net/Platform/a%20b/c%23d%2Fe/");
    }
}
