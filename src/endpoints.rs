use std::collections::HashMap;

use reqwest::Url;

use crate::bungie_client::BungieClient;
use crate::types::definitions::DestinyInventoryItemDefinition;
use crate::types::destiny::definitions::sockets::{
    DestinySocketCategoryDefinition, DestinySocketTypeDefinition,
};
use crate::types::destiny::DestinyManifest;
use crate::{Error, Result};

impl BungieClient {
    pub async fn destiny_manifest(&self) -> Result<DestinyManifest> {
        self.get::<DestinyManifest>(Url::parse(
            "https://www.bungie.net/Platform/Destiny2/Manifest/",
        )?)
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

        let url = format!("https://www.bungie.net{}", item_definition_path);

        let request = self.client.get(url);

        let response = request.send().await?;

        if let Some(hv) = response.headers().get("Content-Type") {
            if !hv
                .to_str()
                .map(|s| s.starts_with("application/json"))
                .unwrap_or(false)
            {
                return Err(Error::InvalidContentType(hv.to_owned()));
            }
        }

        let deserialized = response
            .json::<HashMap<String, DestinyInventoryItemDefinition>>()
            .await?;

        Ok(deserialized)
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

        let url = format!("https://www.bungie.net{}", item_definition_path);

        let request = self.client.get(url);

        let response = request.send().await?;

        if let Some(hv) = response.headers().get("Content-Type") {
            if !hv
                .to_str()
                .map(|s| s.starts_with("application/json"))
                .unwrap_or(false)
            {
                return Err(Error::InvalidContentType(hv.to_owned()));
            }
        }

        let deserialized = response
            .json::<HashMap<String, DestinySocketTypeDefinition>>()
            .await?;

        Ok(deserialized)
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

        let url = format!("https://www.bungie.net{}", item_definition_path);

        let request = self.client.get(url);

        let response = request.send().await?;

        if let Some(hv) = response.headers().get("Content-Type") {
            if !hv
                .to_str()
                .map(|s| s.starts_with("application/json"))
                .unwrap_or(false)
            {
                return Err(Error::InvalidContentType(hv.to_owned()));
            }
        }

        let deserialized = response
            .json::<HashMap<String, DestinySocketCategoryDefinition>>()
            .await?;

        Ok(deserialized)
    }
}
