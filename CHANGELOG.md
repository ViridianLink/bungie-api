# Changelog

## 2.0.0

### Fixed

- `search_destiny_player` uses `POST SearchDestinyPlayerByBungieName`; the
  `GET SearchDestinyPlayer` endpoint no longer exists.
- 64-bit IDs (`characterId`, `itemInstanceId`, `membershipId`, ...) are parsed
  from the strings Bungie sends instead of failing.
- Fields the API marks nullable are `Option`s, so non-instanced items, profiles
  without a current season and similar no longer fail to deserialize.
- `ComponentResponse::data` is optional, so private components no longer fail.
- Item component sets only require the components that were requested.
- `characterUninstancedItemComponents` is keyed by character ID.
- `platformSilver` keys (membership type names) deserialize.
- `DestinyMilestoneActivityPhase::phase_hash` reads `phaseHash`.
- Bungie error envelopes, including those returned with HTTP 4xx/5xx statuses,
  surface as `BungieApiError::Bungie` with the status and message instead of a
  JSON error.

### Added

- Enum values and fields from the current API spec, including
  `BungieMembershipType::GoliathGame`, `DestinyActivityModeType::{Relic,
  LawlessFrontier}`, `ItemState::Enhanced`, new `DestinyGameVersions`, gear
  tier, season pass hashes, skulls and difficulty tiers.
- `strict` feature to opt in to `deny_unknown_fields`.
- `BungieClientBuilder::user_agent`.
- `BungieClient::destiny_definitions::<T>()` and the `DestinyDefinition` trait.
- `BungieClient::post_bungie_response`.
- `Clone` on all models, and `PartialEq`, `Eq`, `Hash` on enums and bitflags.

### Changed

- Unknown JSON fields are ignored unless the `strict` feature is enabled.
- `BungieApiError` is `#[non_exhaustive]` and reworked: `ClientError` and
  `ServerError` became `Http { status, body }`, `Bungie` carries the error
  details, `InvalidJsonSchema` became `ManifestPathNotFound`, `NoResponse`
  became `MissingResponse`, `UrlParseError` became `UrlParse`, and `Io` was
  removed. It now has a readable `Display` and implements `source()`.
- `BungieClient::handle_bungie_response` was replaced by
  `BungieResponse::into_result`.
- `PlatformErrorCodes` is `#[non_exhaustive]`.
- The `serde_as` module is private.
- Duplicate `types::TierType` and `types::ItemLocation` were removed; they are
  re-exported from `types::destiny`.
- `DestinyManifest::mobile_gear_c_d_n` was renamed to `mobile_gear_cdn`.
- The library no longer prints to stdout.
- Date-time fields use `jiff::Timestamp` instead of `chrono::DateTime<Utc>`.
- Dependencies are pinned to explicit versions instead of `*`.
