# Ironforge
A rust implementation of the [Battle.NET API](https://develop.battle.net/documentation).

## Implemented APIs
### General API Features
- [X] Proper error management
- [X] US/EU/KR/TW region API endpoints
- [X] CN region API endpoints
- [X] Namespace identification
- [X] Locale identification
### BattleNet OAuth API
- [X] All regions except CN
- [X] CN region endpoint
### World of Warcraft API
<details>
<summary>52/53 APIs Implemented</summary>
Game Data API:

- [X] Achievements
- [X] Auction House
- [X] Azerite Essence
- [X] Connected Realm
- [X] Covenant
- [X] Creature
- [X] Guild Crest
- [X] Heirloom
- [X] Item
- [X] Journal
- [X] Media Search
- [X] Modified Crafting
- [X] Mount
- [X] Mythic Keystone Affix
- [X] Mythic Keystone Dungeon
- [X] Mythic Keystone Leaderboard
- [X] Mythic Raid Leaderboard
- [X] Pet
- [X] Playable Class
- [X] Playable Race
- [X] Playable Specialization
- [X] Power Type
- [X] Profession
- [X] PvP Season
- [X] PvP Tier
- [X] Quest
- [X] Realm
- [X] Region
- [X] Reputations
- [X] Spell
- [X] Talent
- [X] Tech Talent
- [X] Title
- [X] Toy
- [X] Wow Token

Profile Data API:
- [ ] Account Profile
- [X] Character Achievements
- [X] Character Appearance
- [X] Character Collections
- [X] Character Encounters
- [X] Character Equipment
- [X] Character Hunter Pets
- [X] Character Media
- [X] Character Mythic Keystone Profile
- [X] Character Professions
- [X] Character Profile
- [X] Character PvP
- [X] Character Quests
- [X] Character Reputations
- [X] Character Soulbinds
- [X] Character Specializations
- [X] Character Statistics
- [X] Character Titles
- [X] Guild
</details>

### World of Warcraft Classic API
<details>
<summary>0/23 APIs Implemented</summary>
Game Data API:
  
- [ ] Auction House
- [ ] Connected Realm
- [ ] Creature
- [ ] Guild Crest
- [ ] Item
- [ ] Media Search
- [ ] Playable Class
- [ ] Playable Race
- [ ] Power Type
- [ ] PvP Season
- [ ] Realm
- [ ] Region
- [ ] Wow Token
      
Profile Data API:
- [ ] Account Profile
- [ ] Character Achievements
- [ ] Character Appearance
- [ ] Character Equipment
- [ ] Character Hunter Pets
- [ ] Character Media
- [ ] Character Profile
- [ ] Character PvP
- [ ] Character Specializations
- [ ] Character Statistics
- [ ] Guild
</details>

### Diablo III API
<details>
<summary>0/8 APIs Implemented</summary>
</details>

### Hearthstone API
<details>
<summary>0/4 APIs Implemented</summary>
</details>

### StarCraft II API
<details>
<summary>0/5 APIs Implemented</summary>
</details>

## Tracing & Logging
Ironforge uses the [`tracing`](https://docs.rs/tracing) ecosystem for structured, async-aware logging. This provides:
- Detailed logs for all API requests, responses, and errors
- Spans for token refresh, deserialization, and request coordination
- Configurable log levels (info, debug, warn, error, trace)
- Easy integration with `tracing-subscriber` for console, file, or JSON output

### Enabling Tracing
You can enable tracing on the Blizzard API client using the builder:

```rust
use ironforge::api_client::{BlizzardAPIClientBuilder, Region, Locale};
use tracing::Level;

let client = BlizzardAPIClientBuilder::new()
    .client_id("your_client_id")
    .client_secret("your_client_secret")
    .region(Region::US)
    .locale(Locale::en_US)
    .enable_tracing(true)
    .tracing_level(Level::DEBUG)
    .build()
    .expect("Failed to build client");
```

### Initializing Tracing
Before making requests, initialize a tracing subscriber in your main function:

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "ironforge=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    // ...
}
```

You can control log verbosity with the `RUST_LOG` environment variable:
```
RUST_LOG=ironforge=debug cargo run
```

### What Gets Logged?
- All API requests (endpoint, namespace, status, errors)
- Token refresh operations
- Deserialization attempts and results
- All errors, with context

### Example Output
```
INFO  ironforge::api_client > Sending API request to endpoint: /data/wow/realm/area-52
DEBUG ironforge::api_client > Making HTTP request to: https://us.api.blizzard.com/data/wow/realm/area-52?locale=en_US
INFO  ironforge::api_client > API request completed successfully with status: 200
```

See the `api_client` and `world_of_warcraft` modules for more details on what is traced.
