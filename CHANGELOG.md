# Changelog

## 2.0.0

### Fixed

- `search_destiny_player` uses `POST SearchDestinyPlayerByBungieName`; the
  `GET SearchDestinyPlayer` endpoint no longer exists.
- 64-bit IDs (`characterId`, `itemInstanceId`, `membershipId`, ...) are parsed
  from the strings Bungie sends instead of failing.
- `characterUninstancedItemComponents` is keyed by character ID.
- `platformSilver` keys (membership type names) deserialize.
- `DestinyMilestoneActivityPhase::phase_hash` reads `phaseHash`.
- Fields that live profile and activity history responses omit are optional
  (or default to empty for lists and maps), including
  `ComponentResponse::data` (absent for private components), the item
  component sets (only requested components are sent) and several item,
  record, objective, milestone and progression fields.
- `DestinyItemComponent::dismantle_permission`,
  `DestinyActivityDifficultyTierComponent::is_enabled` and
  `DestinyActivityRewardItem::visibility_unlock_expression`, which Bungie sends
  but does not document.
- `DestinyActivityModeType::SparrowRacing` (94).
- `DestinyColor` accepts the `colorHash` field present in manifest
  definitions.
- Bungie error envelopes, including those returned with HTTP 4xx/5xx statuses,
  surface as `BungieApiError::Bungie` with the status and message instead of a
  JSON error.

### Added

- Enum values and fields from the current API spec, including
  `BungieMembershipType::GoliathGame`, `DestinyActivityModeType::{Relic,
  LawlessFrontier}`, `ItemState::Enhanced`, new `DestinyGameVersions`, gear
  tier, season pass hashes, skulls and difficulty tiers.
- `strict` feature, on by default, controlling `deny_unknown_fields`.
- `BungieClientBuilder::user_agent`.
- `BungieClient::destiny_definitions::<T>()` and the `DestinyDefinition` trait.
- `BungieClient::post_bungie_response`.
- `Clone` on all models, and `PartialEq`, `Eq`, `Hash` on enums and bitflags.

### Changed

- `DestinySocketTypeDefinition::insert_action`,
  `DestinyActivityRewardItem::ui_style` and
  `DestinyHistoricalStatsValue::stat_id` are no longer optional; live data
  always includes them.
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
