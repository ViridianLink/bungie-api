#![expect(clippy::unwrap_used, reason = "Unwrap allowed in tests")]

use bungie_api::BungieApiError;
use bungie_api::types::BungieMembershipType;
use bungie_api::types::components::{ComponentPrivacySetting, ComponentResponse};
use bungie_api::types::destiny::config::DestinyManifest;
use bungie_api::types::destiny::entities::profiles::DestinyProfileComponent;
use bungie_api::types::destiny::historical_stats::{
    DestinyActivityHistoryResults,
    DestinyPostGameCarnageReportData,
};
use bungie_api::types::destiny::responses::DestinyProfileResponse;
use bungie_api::types::destiny::{ItemState, TierType};
use bungie_api::types::exceptions::PlatformErrorCodes;
use bungie_api::types::response::BungieResponse;
use bungie_api::types::user::UserInfoCard;
use serde::de::DeserializeOwned;

/// Fixtures are generated from Bungie's `OpenAPI` spec by
/// `tests/fixtures/generate.py`, with every field populated. With the default
/// `strict` feature, fields the models do not know about are rejected.
fn fixture<T: DeserializeOwned>(name: &str) -> T {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    let json = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&json).unwrap()
}

#[test]
fn profile_response_fixture() {
    let profile: DestinyProfileResponse = fixture("profile_response.json");

    let characters = profile.characters.unwrap().data.unwrap();
    let character = characters.get(&1).unwrap();
    assert_eq!(character.membership_id, 1);

    let silver = profile.platform_silver.unwrap().data.unwrap();
    assert!(silver.platform_silver.contains_key(&BungieMembershipType::All));
}

#[test]
fn post_game_carnage_report_fixture() {
    let _: DestinyPostGameCarnageReportData =
        fixture("post_game_carnage_report.json");
}

#[test]
fn activity_history_fixture() {
    let _: DestinyActivityHistoryResults = fixture("activity_history.json");
}

#[test]
fn user_info_card_fixture() {
    let card: UserInfoCard = fixture("user_info_card.json");
    assert_eq!(card.membership_id, 1);
}

#[test]
fn manifest_fixture() {
    let _: DestinyManifest = fixture("manifest.json");
}

#[test]
fn membership_type_accepts_numbers_and_names() {
    for json in ["3", "\"3\"", "\"TigerSteam\""] {
        let value: BungieMembershipType = serde_json::from_str(json).unwrap();
        assert_eq!(value, BungieMembershipType::TigerSteam);
    }
    assert_eq!(serde_json::to_string(&BungieMembershipType::All).unwrap(), "-1");
    assert!(serde_json::from_str::<BungieMembershipType>("7").is_err());
}

#[test]
fn int64_fields_round_trip_as_strings() {
    let json = r#"{
        "isPublic": true,
        "crossSaveOverride": 0,
        "membershipType": 3,
        "membershipId": "4611686018467284386"
    }"#;
    let card: UserInfoCard = serde_json::from_str(json).unwrap();
    assert_eq!(card.membership_id, 4_611_686_018_467_284_386);

    let value = serde_json::to_value(&card).unwrap();
    assert_eq!(value.get("membershipId").unwrap(), "4611686018467284386");
}

#[test]
fn private_component_has_no_data() {
    let json = r#"{ "privacy": 2 }"#;
    let component: ComponentResponse<DestinyProfileComponent> =
        serde_json::from_str(json).unwrap();
    assert!(component.data.is_none());
    assert_eq!(component.privacy, ComponentPrivacySetting::Private);
}

#[test]
fn bitflags_ignore_unknown_bits() {
    let state: ItemState = serde_json::from_str("65571").unwrap();
    assert_eq!(state, ItemState::Locked | ItemState::Tracked | ItemState::Enhanced);
    assert_eq!(serde_json::to_string(&state).unwrap(), "35");
}

#[test]
fn enums_use_numeric_values() {
    let tier: TierType = serde_json::from_str("6").unwrap();
    assert_eq!(tier, TierType::Exotic);
    assert_eq!(serde_json::to_string(&tier).unwrap(), "6");
    assert!(serde_json::from_str::<TierType>("99").is_err());
}

#[test]
fn error_envelope_becomes_bungie_error() {
    let json = r#"{
        "ErrorCode": 1601,
        "ThrottleSeconds": 0,
        "ErrorStatus": "DestinyAccountNotFound",
        "Message": "We were unable to find your Destiny account information.",
        "MessageData": {}
    }"#;
    let response: BungieResponse<UserInfoCard> = serde_json::from_str(json).unwrap();

    let result = response.into_result();
    assert!(
        matches!(
            &result,
            Err(BungieApiError::Bungie {
                code: PlatformErrorCodes::Unknown(1601),
                status,
                ..
            }) if status == "DestinyAccountNotFound"
        ),
        "unexpected result: {result:?}"
    );
}

#[test]
fn success_envelope_returns_payload() {
    let json = r#"{
        "Response": [],
        "ErrorCode": 1,
        "ThrottleSeconds": 0,
        "ErrorStatus": "Success",
        "Message": "Ok",
        "MessageData": {}
    }"#;
    let response: BungieResponse<Vec<UserInfoCard>> =
        serde_json::from_str(json).unwrap();
    assert!(response.into_result().unwrap().is_empty());
}
