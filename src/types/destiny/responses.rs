use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::components::ComponentResponse;
use crate::types::{DestinyBaseItemComponentSetOfuint32, DestinyItemComponentSetOfint64};

use super::components::collectibles::{
    DestinyCollectiblesComponent, DestinyProfileCollectiblesComponent,
};
use super::components::craftables::DestinyCraftablesComponent;
use super::components::inventory::{DestinyCurrenciesComponent, DestinyPlatformSilverComponent};
use super::components::kiosks::DestinyKiosksComponent;
use super::components::loadouts::DestinyLoadoutsComponent;
use super::components::metrics::DestinyMetricsComponent;
use super::components::plug_sets::DestinyPlugSetsComponent;
use super::components::presentation::DestinyPresentationNodesComponent;
use super::components::profiles::{
    DestinyProfileProgressionComponent, DestinyProfileTransitoryComponent,
};
use super::components::records::{
    DestinyCharacterRecordsComponent, DestinyProfileRecordsComponent,
};
use super::components::social::DestinySocialCommendationsComponent;
use super::components::string_variables::DestinyStringVariablesComponent;
use super::entities::characters::{
    DestinyCharacterActivitiesComponent, DestinyCharacterComponent,
    DestinyCharacterProgressionComponent, DestinyCharacterRenderComponent,
};
use super::entities::inventory::DestinyInventoryComponent;
use super::entities::profiles::{DestinyProfileComponent, DestinyVendorReceiptsComponent};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProfileResponse {
    pub response_minted_timestamp: DateTime<Utc>,
    pub secondary_components_minted_timestamp: DateTime<Utc>,
    pub vendor_receipts: Option<ComponentResponse<DestinyVendorReceiptsComponent>>,
    pub profile_inventory: Option<ComponentResponse<DestinyInventoryComponent>>,
    pub profile_currencies: Option<ComponentResponse<DestinyInventoryComponent>>,
    pub profile: Option<ComponentResponse<DestinyProfileComponent>>,
    pub platform_silver: Option<ComponentResponse<DestinyPlatformSilverComponent>>,
    pub profile_kiosks: Option<ComponentResponse<DestinyKiosksComponent>>,
    pub profile_plug_sets: Option<ComponentResponse<DestinyPlugSetsComponent>>,
    pub profile_progression: Option<ComponentResponse<DestinyProfileProgressionComponent>>,
    pub profile_presentation_nodes: Option<ComponentResponse<DestinyPresentationNodesComponent>>,
    pub profile_records: Option<ComponentResponse<DestinyProfileRecordsComponent>>,
    pub profile_collectibles: Option<ComponentResponse<DestinyProfileCollectiblesComponent>>,
    pub profile_transitory_data: Option<ComponentResponse<DestinyProfileTransitoryComponent>>,
    pub metrics: Option<ComponentResponse<DestinyMetricsComponent>>,
    pub profile_string_variables: Option<ComponentResponse<DestinyStringVariablesComponent>>,
    pub profile_commendations: Option<ComponentResponse<DestinySocialCommendationsComponent>>,
    pub characters: Option<ComponentResponse<HashMap<i64, DestinyCharacterComponent>>>,
    pub character_inventories: Option<ComponentResponse<HashMap<i64, DestinyInventoryComponent>>>,
    pub character_loadouts: Option<ComponentResponse<HashMap<i64, DestinyLoadoutsComponent>>>,
    pub character_progressions:
        Option<ComponentResponse<HashMap<i64, DestinyCharacterProgressionComponent>>>,
    pub character_render_data:
        Option<ComponentResponse<HashMap<i64, DestinyCharacterRenderComponent>>>,
    pub character_activities:
        Option<ComponentResponse<HashMap<i64, DestinyCharacterActivitiesComponent>>>,
    pub character_equipment: Option<ComponentResponse<HashMap<i64, DestinyInventoryComponent>>>,
    pub character_kiosks: Option<ComponentResponse<HashMap<i64, DestinyKiosksComponent>>>,
    pub character_plug_sets: Option<ComponentResponse<HashMap<i64, DestinyPlugSetsComponent>>>,
    pub character_uninstanced_item_components: Option<DestinyBaseItemComponentSetOfuint32>,
    pub character_presentation_nodes:
        Option<ComponentResponse<HashMap<i64, DestinyPresentationNodesComponent>>>,
    pub character_records:
        Option<ComponentResponse<HashMap<i64, DestinyCharacterRecordsComponent>>>,
    pub character_collectibles:
        Option<ComponentResponse<HashMap<i64, DestinyCollectiblesComponent>>>,
    pub character_string_variables:
        Option<ComponentResponse<HashMap<i64, DestinyStringVariablesComponent>>>,
    pub character_craftables: Option<ComponentResponse<HashMap<i64, DestinyCraftablesComponent>>>,
    pub item_components: Option<DestinyItemComponentSetOfint64>,
    pub character_currency_lookups:
        Option<ComponentResponse<HashMap<i64, DestinyCurrenciesComponent>>>,
}
