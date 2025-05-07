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

    use crate::BungieClientBuilder;

    const BUNGIE_API_KEY: &str = "";

    #[tokio::test]
    async fn run() {
        let client = BungieClientBuilder::new(BUNGIE_API_KEY).build().unwrap();

        let manifest = client.destiny_manifest().await.unwrap();
        client
            .destiny_inventory_item_definition(&manifest, "en")
            .await
            .unwrap();
        client
            .destiny_socket_type_definition(&manifest, "en")
            .await
            .unwrap();
        client
            .destiny_socket_category_definition(&manifest, "en")
            .await
            .unwrap();
        client.destiny_plug_set_definition(&manifest, "en").await
        //     .unwrap();
    }
}
