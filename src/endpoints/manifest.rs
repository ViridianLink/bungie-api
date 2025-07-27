use std::collections::HashMap;

use crate::Result;
use crate::bungie_client::BungieClient;
use crate::types::definitions::DestinyInventoryItemDefinition;
use crate::types::destiny::config::DestinyManifest;
use crate::types::destiny::definitions::sockets::{
    DestinyPlugSetDefinition, DestinySocketCategoryDefinition, DestinySocketTypeDefinition,
};

impl BungieClient {
    pub async fn destiny_manifest(&self) -> Result<DestinyManifest> {
        self.get_bungie_response::<DestinyManifest>(
            "https://www.bungie.net/Platform/Destiny2/Manifest/",
        )
        .await
    }

    pub async fn destiny_inventory_item_definition(
        &self,
        manifest: &DestinyManifest,
        local: &str,
    ) -> Result<HashMap<String, DestinyInventoryItemDefinition>> {
        let item_definition_path = manifest
            .json_world_component_content_paths
            .get(local)
            .unwrap()
            .get("DestinyInventoryItemDefinition")
            .unwrap();

        let url = format!("https://www.bungie.net{item_definition_path}");

        self.get::<HashMap<String, DestinyInventoryItemDefinition>>(url)
            .await
    }

    pub async fn destiny_socket_type_definition(
        &self,
        manifest: &DestinyManifest,
        local: &str,
    ) -> Result<HashMap<String, DestinySocketTypeDefinition>> {
        let item_definition_path = manifest
            .json_world_component_content_paths
            .get(local)
            .unwrap()
            .get("DestinySocketTypeDefinition")
            .unwrap();

        let url = format!("https://www.bungie.net{item_definition_path}");

        self.get::<HashMap<String, DestinySocketTypeDefinition>>(url)
            .await
    }

    pub async fn destiny_socket_category_definition(
        &self,
        manifest: &DestinyManifest,
        local: &str,
    ) -> Result<HashMap<String, DestinySocketCategoryDefinition>> {
        let item_definition_path = manifest
            .json_world_component_content_paths
            .get(local)
            .unwrap()
            .get("DestinySocketCategoryDefinition")
            .unwrap();

        let url = format!("https://www.bungie.net{item_definition_path}");

        self.get::<HashMap<String, DestinySocketCategoryDefinition>>(url)
            .await
    }

    pub async fn destiny_plug_set_definition(
        &self,
        manifest: &DestinyManifest,
        local: &str,
    ) -> Result<HashMap<String, DestinyPlugSetDefinition>> {
        let item_definition_path = manifest
            .json_world_component_content_paths
            .get(local)
            .unwrap()
            .get("DestinyPlugSetDefinition")
            .unwrap();

        let url = format!("https://www.bungie.net{item_definition_path}");

        self.get::<HashMap<String, DestinyPlugSetDefinition>>(url)
            .await
    }
}
