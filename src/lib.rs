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
    use crate::BungieClientBuilder;
    
    #[expect(clippy::unwrap_used, reason = "Unwrap allowed in tests")]
    #[ignore = "requires network access"]
    #[tokio::test]
    async fn live_manifest() {
        let api_key = std::env::var("BUNGIE_API_KEY").unwrap_or_default();
        let client = BungieClientBuilder::new(api_key).build().unwrap();

        let manifest = client.destiny_manifest().await.unwrap();
        client.destiny_inventory_item_definition(&manifest, "en").await.unwrap();
        client.destiny_socket_type_definition(&manifest, "en").await.unwrap();
        client.destiny_socket_category_definition(&manifest, "en").await.unwrap();
        client.destiny_plug_set_definition(&manifest, "en").await.unwrap();
    }
}
