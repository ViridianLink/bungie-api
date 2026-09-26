use std::collections::HashMap;

use serde::de::DeserializeOwned;
use url::Url;

use crate::bungie_client::{BUNGIE_URL, BungieClient};
use crate::types::definitions::DestinyInventoryItemDefinition;
use crate::types::destiny::config::DestinyManifest;
use crate::types::destiny::definitions::sockets::{
    DestinyPlugSetDefinition,
    DestinySocketCategoryDefinition,
    DestinySocketTypeDefinition,
};
use crate::{BungieApiError, Result};

pub trait DestinyDefinition: DeserializeOwned {
    const NAME: &'static str;
}

impl DestinyDefinition for DestinyInventoryItemDefinition {
    const NAME: &'static str = "DestinyInventoryItemDefinition";
}

impl DestinyDefinition for DestinySocketTypeDefinition {
    const NAME: &'static str = "DestinySocketTypeDefinition";
}

impl DestinyDefinition for DestinySocketCategoryDefinition {
    const NAME: &'static str = "DestinySocketCategoryDefinition";
}

impl DestinyDefinition for DestinyPlugSetDefinition {
    const NAME: &'static str = "DestinyPlugSetDefinition";
}

impl BungieClient {
    pub async fn destiny_manifest(&self) -> Result<DestinyManifest> {
        let url = self.platform_url(["Destiny2", "Manifest"])?;
        self.get_bungie_response(url).await
    }
    
    pub async fn destiny_definitions<T: DestinyDefinition>(
        &self,
        manifest: &DestinyManifest,
        locale: &str,
    ) -> Result<HashMap<String, T>> {
        let path = manifest
            .json_world_component_content_paths
            .get(locale)
            .and_then(|paths| paths.get(T::NAME))
            .ok_or_else(|| BungieApiError::ManifestPathNotFound {
                locale: locale.to_owned(),
                definition: T::NAME,
            })?;

        let url = Url::parse(BUNGIE_URL)?.join(path)?;

        self.get(url).await
    }

    pub async fn destiny_inventory_item_definition(
        &self,
        manifest: &DestinyManifest,
        locale: &str,
    ) -> Result<HashMap<String, DestinyInventoryItemDefinition>> {
        self.destiny_definitions(manifest, locale).await
    }

    pub async fn destiny_socket_type_definition(
        &self,
        manifest: &DestinyManifest,
        locale: &str,
    ) -> Result<HashMap<String, DestinySocketTypeDefinition>> {
        self.destiny_definitions(manifest, locale).await
    }

    pub async fn destiny_socket_category_definition(
        &self,
        manifest: &DestinyManifest,
        locale: &str,
    ) -> Result<HashMap<String, DestinySocketCategoryDefinition>> {
        self.destiny_definitions(manifest, locale).await
    }

    pub async fn destiny_plug_set_definition(
        &self,
        manifest: &DestinyManifest,
        locale: &str,
    ) -> Result<HashMap<String, DestinyPlugSetDefinition>> {
        self.destiny_definitions(manifest, locale).await
    }
}
