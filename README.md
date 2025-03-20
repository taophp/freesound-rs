# freesound-rs

** WORK IN PROGRESS ! DO NOT USE IN PRODUCTION ! **

A Rust client library for the [Freesound API](https://freesound.org/docs/api/).

## Installation

Add this to your `Cargo.toml`:

```bash
cargo add freesound-rs
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

## Running tests

Obtain a Freesound API key:
1. Register for an account at https://freesound.org/
2. Visit https://freesound.org/apiv2/apply/ to create an API application
3. After approval, you'll receive your API key

Create an `.env` file with your keya and run the tests:
```bash
cp env.sample .env
# edit `.env` to add you FreeSound API key
cargo test
```

## License

This project is licensed under the GNU LGPL License - see the LICENSE file for details.
