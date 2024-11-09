mod bungie_client;

pub mod endpoints;
mod error;
pub use error::Error;
use error::Result;

pub mod types;

pub use bungie_client::{BungieClient, BungieClientBuilder};

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[tokio::test]
//     async fn run() {
//         let client = bungie_client::BungieClientBuilder::new(BUNGIE_API_KEY)
//             .build()
//             .unwrap();

//         let destiny_manifest = client.destiny_manifest().await.unwrap();

//         let item_definition = client
//             .destiny_inventory_item_definition(&destiny_manifest, "en")
//             .await
//             .unwrap();
//     }
// }
