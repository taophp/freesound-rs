# freesound-rs

A Rust client library for the [Freesound API](https://freesound.org/docs/api/).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
freesound-rs = "0.1.0"
```

## Usage

```rust
use freesound_rs::FreesoundClient;

fn main() {
    // Initialize client with your API key
    let client = FreesoundClient::new("YOUR_API_KEY".to_string(), None);

    // Use the client to interact with the Freesound API
}
```

## License

This project is licensed under the GNU LGPL License - see the LICENSE file for details.
