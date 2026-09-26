# bungie-api

A typed async client for the [Bungie.net](https://bungie-net.github.io/) Destiny 2 API.

## Usage

```toml
[dependencies]
bungie-api = { git = "https://github.com/ViridianLink/bungie-api" }
```

```rust
use bungie_api::BungieClientBuilder;
use bungie_api::types::destiny::DestinyComponentType;

async fn run() -> bungie_api::Result<()> {
    let client = BungieClientBuilder::new("your-api-key")
        .user_agent("MyApp/1.0 AppId/12345 (+https://example.com;me@example.com)")
        .build()?;

    let players = client.search_destiny_player("Guardian", 1234).await?;

    for player in players {
        let profile = client
            .profile(
                player.membership_type,
                player.membership_id,
                &[DestinyComponentType::Profiles, DestinyComponentType::Characters],
            )
            .await?;
        println!("{:?}", profile.characters);
    }

    let manifest = client.destiny_manifest().await?;
    let items = client.destiny_inventory_item_definition(&manifest, "en").await?;
    println!("{} item definitions", items.len());
    Ok(())
}
```

A runnable version is in `examples/profile.rs`.

Bungie returns errors in a JSON envelope; these surface as
`BungieApiError::Bungie { code, status, message, .. }`.

## Features

- `strict` (default): reject JSON fields the models do not know about. Build
  with `default-features = false` to ignore unknown fields instead.

## Development

The crate uses the nightly toolchain pinned in `rust-toolchain.toml` (for
`rustfmt`'s unstable options).

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`tests/fixtures` holds JSON generated from Bungie's
[OpenAPI spec](https://github.com/Bungie-net/api/blob/master/openapi.json) with
every field populated. Regenerate it after updating the models:

```sh
python3 tests/fixtures/generate.py path/to/openapi.json
```

A test that downloads the live manifest and definitions is ignored by default
(no API key needed):

```sh
cargo test -- --ignored
```
