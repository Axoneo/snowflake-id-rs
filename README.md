# Snowflake ID Generator

A Rust implementation of a **Snowflake ID Generator**, designed to generate unique 64-bit IDs in both **single-threaded** and **multi-threaded** environments. The generator supports both **synchronous** and **asynchronous** modes.

## Features

- **64-bit unique IDs**:
  - Combines timestamp, worker ID, and sequence number to ensure uniqueness.
- **Custom epoch**:
  - Allows you to define a custom epoch for timestamp calculations.
- **Single-threaded and multi-threaded support**:
  - Optimized for both single-threaded and multi-threaded environments.
- **Synchronous and asynchronous modes**:
  - Supports both blocking and non-blocking ID generation.

## Project Structure

- `src/common.rs`: Contains the core `SnowflakeState` structure and logic for generating IDs.
- `src/multi_thread.rs`: Implements multi-threaded generators for both synchronous and asynchronous environments.
- `src/single_thread.rs`: Implements single-threaded generators for synchronous and asynchronous environments.

## Usage

### Multi-threaded Asynchronous Generator

```rust
use snowflake_id_generator::multi_thread::MultiThreadedAsyncGenerator;

#[tokio::main]
async fn main() {
    let generator = MultiThreadedAsyncGenerator::new(0, 1).unwrap();
    let id = generator.generate_id().await;
    println!("Generated ID: {}", id);
}
```

### Multi-threaded Synchronous Generator

```rust
use snowflake_id_generator::multi_thread::MultiThreadedSyncGenerator;

fn main() {
    let generator = MultiThreadedSyncGenerator::new(0, 1).unwrap();
    let id = generator.generate_id().unwrap();
    println!("Generated ID: {}", id);
}
```

## ID Format

The generated 64-bit ID is composed of the following components:

| Bits  | Component         | Description                                   |
|-------|-------------------|-----------------------------------------------|
| 1     | Sign Bit          | Always 0 for positive IDs.                    |
| 41    | Timestamp         | Milliseconds since the custom epoch.         |
| 10    | Worker ID         | Unique identifier for the worker (0-1023).   |
| 12    | Sequence Number   | Counter for IDs generated in the same millisecond. |

## Error Handling

The generator may return the following errors:
- **`SnowflakeError::WorkerIdOutOfRange`**: If the worker ID is not in the range `0-1023`.
- **`SnowflakeError::TimeBeforeUnixEpoch`**: If the system time is before the UNIX epoch.
- **`SnowflakeError::EpochInFuture`**: If the custom epoch is set in the future.

## Testing

Run the tests using the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
