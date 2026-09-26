//! Looks up a player by Bungie Name and prints their characters.
//!
//! `BUNGIE_API_KEY=... cargo run --example profile -- Name 1234`

use bungie_api::BungieClientBuilder;
use bungie_api::types::destiny::DestinyComponentType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("BUNGIE_API_KEY")?;
    let mut args = std::env::args().skip(1);
    let (Some(name), Some(code)) = (args.next(), args.next()) else {
        return Err("usage: profile <name> <code>".into());
    };

    let client = BungieClientBuilder::new(api_key).build()?;

    let players = client.search_destiny_player(&name, code.parse()?).await?;

    for player in players {
        let profile = client
            .profile(player.membership_type, player.membership_id, &[
                DestinyComponentType::Profiles,
                DestinyComponentType::Characters,
            ])
            .await?;

        let characters =
            profile.characters.map_or_else(Default::default, |c| c.data);
        for (id, character) in characters {
            println!(
                "{id}: {:?} {:?}, light {}",
                character.race_type, character.class_type, character.light
            );
        }
    }

    Ok(())
}
