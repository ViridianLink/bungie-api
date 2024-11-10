mod bungie_client;

pub mod endpoints;
mod error;
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

    use super::*;

    const BUNGIE_API_KEY: &str = "";

    #[tokio::test]
    async fn run() {
        let client = bungie_client::BungieClientBuilder::new(BUNGIE_API_KEY)
            .build()
            .unwrap();

        let destiny_manifest = client.destiny_manifest().await.unwrap();

        let definitions = client
            .destiny_plug_set_definition(&destiny_manifest, "en")
            .await
            .unwrap();
    }
}
