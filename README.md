# Snowflake ID Generator (Rust)

A compact, easy-to-use Rust implementation of a Snowflake-style 64-bit ID generator. This crate provides both single-threaded and multi-threaded generators, and supports synchronous (blocking) and asynchronous (non-blocking) usage patterns.

Key goals:
- Small and focused API
- Correct bit layout and overflow handling
- Usable from sync and async contexts

## Features

- 64-bit unique IDs composed of timestamp, worker ID and a per-millisecond sequence.
- Configurable custom epoch.
- Sync and async generators.
- Single-threaded and multi-threaded implementations.

## Examples

Example (async, multi-threaded):

```rust
use snowflake_id_generator::multi_thread::async_generator::SnowflakeGenerator;

#[tokio::main]
async fn main() {
    // worker_id, custom_epoch_ms
    let generator = SnowflakeGenerator::new(0, 1).expect("valid generator");
    let id = generator.generate_id().await;
    println!("Generated ID: {}", id);
}
```

Example (sync, single-threaded):

```rust
use snowflake_id_generator::single_thread::sync_generator::SnowflakeGenerator;

fn main() {
    let mut generator = SnowflakeGenerator::new(0, 1).expect("valid generator");
    let id = generator.generate_id().expect("generate id");
    println!("Generated ID: {}", id);
}
```

## Project Structure

See the `src/` modules for more generators and utilities:

- `src/common.rs` — core `SnowflakeState`, bit layout and helpers
- `src/single_thread.rs` — single-threaded sync/async generators
- `src/multi_thread.rs` — multi-threaded sync/async generators

## ID Layout

The 64-bit ID layout used by this generator (from most-significant bit to least):

- 1 bit: unused/sign (always 0)
- 41 bits: timestamp in milliseconds since the custom epoch
- 10 bits: worker ID (0..1023)
- 12 bits: per-millisecond sequence (0..4095)

This layout allows unique IDs across workers and within the same millisecond.

## Errors

Common error conditions provided by the crate include:

- Worker ID out of allowed range (0..1023).
- Epoch set in the future.
- System clock issues that would cause timestamps earlier than the epoch.

Refer to the crate's error types in `src/common.rs` for exact variants and `Display` messages.


## Contributing

Contributions are welcome. Please:

1. Open an issue to discuss large changes.
2. Submit small, focused pull requests.
3. Add tests for bug fixes and new functionality.

## License

This project is licensed under the terms in the repository's `LICENSE` file.