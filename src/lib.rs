mod bungie_client;

pub mod endpoints;
mod error;
pub use error::Error;
use error::Result;

pub mod types;

pub use bungie_client::{BungieClient, BungieClientBuilder};
pub use types::definitions::DestinyInventoryItemDefinition;
pub use types::destiny::definitions::sockets::{
    DestinyPlugSetDefinition, DestinySocketCategoryDefinition, DestinySocketTypeDefinition,
};

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
