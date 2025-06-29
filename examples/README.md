# Ironforge Examples

This directory contains examples demonstrating how to use the Ironforge library.

## Examples

### `basic_usage.rs`
A simple example showing how to:
- Initialize the Blizzard API client
- Make basic API calls (achievements, realm info, character profile)

### Running Examples

To run an example:

```bash
# Set your Blizzard API credentials
export BLIZZARD_CLIENT_ID="your_client_id"
export BLIZZARD_CLIENT_SECRET="your_client_secret"

# Run the basic usage example
cargo run --example basic_usage
```

### Prerequisites

1. A Blizzard Developer Account with API credentials
2. The `BLIZZARD_CLIENT_ID` and `BLIZZARD_CLIENT_SECRET` environment variables set
3. Rust and Cargo installed
