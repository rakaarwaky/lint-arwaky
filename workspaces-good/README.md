# Calculator

A small multi-language calculator demo workspace with Rust features under `crates/`, Python features under `modules/`, and TypeScript features under `packages/`, all conforming to the AES layer model.

## Prerequisites

- Rust toolchain (edition 2024 via rust-toolchain.toml)
- Python 3.10+
- Node.js / npm

## Quick Start

```bash
# Build
CARGO_INCREMENTAL=0 cargo build --release

# Lint / check
lint-arwaky-cli check .

# Run tests
CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2
```

## Architecture

See `ARCHITECTURE.md`.

## Available Scripts / Commands

| Command                                 | Purpose                          |
| --------------------------------------- | -------------------------------- |
| `cargo build --release`                 | Build all Rust crates            |
| `lint-arwaky-cli check .`               | Run architecture scanner         |
| `cargo nextest run --workspace`         | Run all tests                    |

## Configuration

<Environment variables, config files — names only, never values>

## Testing

See `TESTING.md`.

## Contributing

See `CONTRIBUTING.md`.

## License

MIT
