//! A typed async client for the Bungie.net Destiny 2 API.
//!
//! ```no_run
//! # async fn run() -> bungie_api::Result<()> {
//! use bungie_api::BungieClientBuilder;
//!
//! let client = BungieClientBuilder::new("your-api-key").build()?;
//! let manifest = client.destiny_manifest().await?;
//! let items = client.destiny_inventory_item_definition(&manifest, "en").await?;
//! # Ok(())
//! # }
//! ```

mod bungie_client;
mod serde_as;

pub mod endpoints;
pub mod error;
pub mod types;

use std::collections::HashMap;

pub use bungie_client::{BungieClient, BungieClientBuilder};
pub use endpoints::DestinyDefinition;
pub use error::{BungieApiError, Result};
pub use types::definitions::DestinyInventoryItemDefinition;
pub use types::destiny::definitions::sockets::{
    DestinyPlugSetDefinition,
    DestinySocketCategoryDefinition,
    DestinySocketTypeDefinition,
};

pub type DestinyInventoryItemManifest =
    HashMap<String, DestinyInventoryItemDefinition>;
pub type DestinyPlugSetManifest = HashMap<String, DestinyPlugSetDefinition>;
pub type DestinySocketCategoryManifest =
    HashMap<String, DestinySocketCategoryDefinition>;
pub type DestinySocketTypeManifest = HashMap<String, DestinySocketTypeDefinition>;

#[cfg(test)]
mod tests {
    use crate::types::destiny::DestinyComponentType as C;
    use crate::{BungieClient, BungieClientBuilder};

    fn live_client() -> BungieClient {
        let api_key = std::env::var("BUNGIE_API_KEY").unwrap_or_default();
        #[expect(clippy::unwrap_used, reason = "Unwrap allowed in tests")]
        BungieClientBuilder::new(api_key).build().unwrap()
    }

    /// Downloads the live manifest and definitions. Needs network access, so it
    /// only runs when asked for: `cargo test -- --ignored`. The manifest does
    /// not require an API key; `BUNGIE_API_KEY` is used if set.
    #[expect(clippy::unwrap_used, reason = "Unwrap allowed in tests")]
    #[ignore = "requires network access"]
    #[tokio::test]
    async fn live_manifest() {
        let client = live_client();

        let manifest = client.destiny_manifest().await.unwrap();
        client.destiny_inventory_item_definition(&manifest, "en").await.unwrap();
        client.destiny_socket_type_definition(&manifest, "en").await.unwrap();
        client.destiny_socket_category_definition(&manifest, "en").await.unwrap();
        client.destiny_plug_set_definition(&manifest, "en").await.unwrap();
    }

    /// Looks up the player in `BUNGIE_TEST_PLAYER` (`Name#1234`), then loads
    /// their profile with every component and their activity history. Needs
    /// `BUNGIE_API_KEY`; skipped when either variable is unset.
    #[expect(clippy::unwrap_used, reason = "Unwrap allowed in tests")]
    #[ignore = "requires network access, BUNGIE_API_KEY and BUNGIE_TEST_PLAYER"]
    #[tokio::test]
    async fn live_player() {
        let (Ok(_), Ok(player)) =
            (std::env::var("BUNGIE_API_KEY"), std::env::var("BUNGIE_TEST_PLAYER"))
        else {
            return;
        };
        let (name, code) = player.rsplit_once('#').unwrap();
        let client = live_client();

        let cards =
            client.search_destiny_player(name, code.parse().unwrap()).await.unwrap();
        // Prefer the cross save primary account.
        let card = cards
            .iter()
            .find(|c| c.membership_type == c.cross_save_override)
            .or_else(|| cards.first())
            .unwrap();

        let components = [
            C::Profiles,
            C::VendorReceipts,
            C::ProfileInventories,
            C::ProfileCurrencies,
            C::ProfileProgression,
            C::PlatformSilver,
            C::Characters,
            C::CharacterInventories,
            C::CharacterProgressions,
            C::CharacterRenderData,
            C::CharacterActivities,
            C::CharacterEquipment,
            C::CharacterLoadouts,
            C::ItemInstances,
            C::ItemObjectives,
            C::ItemPerks,
            C::ItemRenderData,
            C::ItemStats,
            C::ItemSockets,
            C::ItemTalentGrids,
            C::ItemCommonData,
            C::ItemPlugStates,
            C::ItemPlugObjectives,
            C::ItemReusablePlugs,
            C::Kiosks,
            C::CurrencyLookups,
            C::PresentationNodes,
            C::Collectibles,
            C::Records,
            C::Transitory,
            C::Metrics,
            C::StringVariables,
            C::Craftables,
            C::SocialCommendations,
        ];
        let profile = client
            .profile(card.membership_type, card.membership_id, &components)
            .await
            .unwrap();

        let characters = profile.characters.unwrap().data.unwrap();
        for &character_id in characters.keys() {
            client
                .activity_history(
                    card.membership_type,
                    card.membership_id,
                    character_id.cast_unsigned(),
                    Some(25),
                    None,
                    0,
                )
                .await
                .unwrap();
        }
    }
}
