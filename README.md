# freesound-rs

** WORK IN PROGRESS ! DO NOT USE IN PRODUCTION ! **

A Rust client library for the [Freesound API](https://freesound.org/docs/api/).

Note: this is a side project, so I should only implement search and get sound endpoints for now. But contributions are welcome!

## Installation

Add this to your `Cargo.toml`:

```bash
cargo add freesound-rs
```

## Usage

### Basic setup

```rust
use freesound_rs::{FreesoundClient, SearchQueryBuilder, SortOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client with your API key
    let client = FreesoundClient::new("YOUR_API_KEY".to_string(), None);

    Ok(())
}
```

### Searching sounds

```rust
// Simple search
let query = SearchQueryBuilder::new()
    .query("piano")
    .build();
let results = client.search(&query).await?;
println!("Found {} piano sounds", results.count);

// Advanced search with filters and sorting
let query = SearchQueryBuilder::new()
    .query("music")
    .filter("tag:guitar duration:[1 TO 10]")  // Guitar sounds between 1 and 10 seconds
    .sort(SortOption::RatingDesc)            // Best rated first
    .page(1)
    .page_size(15)
    .fields(["id", "name", "tags", "previews"])  // Only return these fields
    .build();
let results = client.search(&query).await?;

// Process search results
for sound in results.results {
    println!("Found sound: {} (#{}) by {}", sound.name, sound.id, sound.username);
}
```

### Getting sound details

```rust
// Get basic sound information
let sound = client.get_sound(1234, None, None).await?;
println!("Sound: {} by {}", sound.name, sound.username);
println!("Duration: {}s", sound.duration);
println!("License: {}", sound.license);

// Get sound with audio analysis descriptors
let sound = client
    .get_sound(
        1234,
        Some(&["lowlevel.mfcc", "rhythm.bpm"]),
        Some(true)  // normalize values
    )
    .await?;

// Access preview URLs
if let Some(previews) = sound.previews {
    println!("HQ MP3 preview: {}", previews.preview_hq_mp3);
    println!("HQ OGG preview: {}", previews.preview_hq_ogg);
}
```

## Running tests

Obtain a Freesound API key:
1. Register for an account at https://freesound.org/
2. Visit https://freesound.org/apiv2/apply/ to create an API application
3. After approval, you'll receive your API key

Create an `.env` file with your key and run the tests:
```bash
cp env.sample .env
# edit `.env` to add your FreeSound API key
cargo test
```

## License

This project is licensed under the GNU LGPL License - see the LICENSE file for details.
