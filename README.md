# splatoon3-rs

A Rust library for accessing Splatoon 3 stage rotation information.

## Features

- Fetch current and upcoming stage schedules for:
  - Regular Battle
  - Bankara (Anarchy) Open
  - Bankara (Anarchy) Challenge
  - X Battle

## Installation

Add this to your `Cargo.toml`

```toml
[dependencies]
splatoon3-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Usage

```rust
use splatoon3_rs::client::SplaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// Create a new client
let client = SplaClient::new()?;
// Get current Regular Battle stages
let regular_stages = client.get_now_regular_open_stages().await?;
// Get upcoming Bankara Open stages
let bankara_open_stages = client.get_next_bankara_open_stages().await?;
// Get Bankara Challenge schedule
let bankara_challenge_stages = client.get_bankara_challenge_schedule().await?;
// Get X Battle schedule
let x_battle_stages = client.get_x_schedule().await?;
Ok(())
}
```

## API Reference

### `SplaClient`

#### `new()`

Creates a new instance of the SplaClient.

#### `get_now_regular_open_stages()`

Returns the current Regular Battle stages.

#### `get_next_bankara_open_stages()`

Returns the upcoming Bankara Open stages.

#### `get_bankara_challenge_schedule()`

Returns the Bankara Challenge schedule.

#### `get_x_schedule()`

Returns the X Battle schedule.

All methods return `Result<Vec<Schedule>, Box<dyn std::error::Error>>` where `Schedule` contains:

- `stages`: Vector of stage information (id and name)
- `start_time`: Schedule start time
- `end_time`: Schedule end time

## Error Handling

The library uses Rust's standard error handling with `Result` types. All API methods will return errors in cases such as:

- Network connection failures
- Invalid API responses
- Server errors

## Development

To run tests:

```bash
cargo test
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
