# Lint Arwaky

> Autonomous code quality and architecture enforcement for AI agents and developers — written in Rust.

[![Rust 2021](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![MCP Server](https://img.shields.io/badge/MCP-Server-blue.svg)](https://modelcontextprotocol.io/)
[![Architecture: AES](https://img.shields.io/badge/architecture-AES+Clean-green.svg)](ARCHITECTURE.md)

## Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo (bundled with Rust)
- Git

## Quick Start

```bash
# Clone
git clone https://github.com/rakaarwaky/lint-arwaky.git
cd lint-arwaky

# Build
cargo build --release

# Verify
./target/release/lint-arwaky-cli version
# Expected: Lint Arwaky v1.10.74 (AES Semantic Builder)

# Self-lint
./target/release/lint-arwaky-cli check .
```

## Architecture

Lint Arwaky follows its own AES (Agentic Engineering System) specification — a strict layered architecture with seven layers, organized into feature crates (vertical slicing).

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full specification, layer hierarchy, and naming conventions.

## Project Structure

```
lint-arwaky/
├── crates/
│   ├── auto-fix/           # Auto-fix processor
│   ├── cli-commands/       # CLI surface
│   ├── code-analysis/      # Code quality checks
│   ├── config-system/      # Config loading
│   ├── external-lint/      # External linter adapters
│   ├── file-watch/         # File watching
│   ├── git-hooks/          # Git hooks
│   ├── import-rules/       # Import compliance
│   ├── maintenance/        # Maintenance utilities
│   ├── mcp-server/         # MCP server
│   ├── naming-rules/       # Naming conventions
│   ├── orphan-detector/    # Orphan code detection
│   ├── project-setup/      # Setup utilities
│   ├── role-rules/         # Role violations
│   └── tui/                # TUI file browser
├── shared/                 # Taxonomy, contracts, utilities
├── PRD.md                  # Product requirements
├── ARCHITECTURE.md         # AES specification
└── README.md               # This file
```

## Available Commands

| Command                               | Description                               |
| ------------------------------------- | ----------------------------------------- |
| `lint-arwaky-cli check [path]`        | Full architecture compliance analysis     |
| `lint-arwaky-cli scan [path]`         | External project scan                     |
| `lint-arwaky-cli fix [path]`          | Apply safe fixes (`--dry-run` to preview) |
| `lint-arwaky-cli ci [path]`           | CI mode with exit codes                   |
| `lint-arwaky-cli orphan <path>`       | Check if file is dead/unreachable code    |
| `lint-arwaky-cli watch [path]`        | Watch and lint on changes                 |
| `lint-arwaky-cli security [path]`     | Scan for security vulnerabilities         |
| `lint-arwaky-cli dependencies [path]` | Scan for library vulnerabilities          |
| `lint-arwaky-cli duplicates [path]`   | Detect code duplication                   |
| `lint-arwaky-cli doctor`              | Environment diagnostics                   |
| `lint-arwaky-cli version`             | Display version                           |

## Configuration

Configuration is loaded from YAML files. See [SKILL.md](SKILL.md) for details.

```bash
# Create default config
lint-arwaky-cli init

# Show current config
lint-arwaky-cli config show
```

## Testing

```bash
# Run all tests
cargo test --workspace

# Self-lint
cargo run --bin lint-arwaky-cli -- check .

# Run MCP server
cargo run --bin lint-arwaky-mcp
```

## MCP Server

See [SKILL.md](SKILL.md) for the MCP tool reference and [DEPLOY.md](DEPLOY.md) for client setup.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT
