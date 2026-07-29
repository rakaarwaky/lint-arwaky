# Lint Arwaky

> Autonomous code quality and architecture enforcement for AI agents and developers -- written in Rust.

[![Rust 2021](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![MCP Server](https://img.shields.io/badge/MCP-Server-blue.svg)](https://modelcontextprotocol.io/)
[![Architecture: AES](https://img.shields.io/badge/architecture-AES+Clean-green.svg)](ARCHITECTURE.md)

## Quick Start

Two install options are available:

### Option 1: Remote Install (fast -- no build, no clone)

```bash
curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/scripts/install.remote.sh | bash
```

The script downloads a **pre-built binary** from GitHub Releases. 

### Option 2: Local Install (build from source)

```bash
# Clone
git clone https://github.com/rakaarwaky/lint-arwaky.git
cd lint-arwaky

# Build and install to ~/.cargo/bin
bash scripts/install.local.sh

# Verify
lint-arwaky-cli version
# Expected: Lint Arwaky v1.11.0
```

Clones the repository, runs `cargo build --release`, and installs binaries to `~/.cargo/bin`. Requires Rust 1.70+ and Cargo.

## Architecture

Lint Arwaky follows its own AES (Agentic Engineering System) specification -- a strict layered architecture with seven layers, organized into feature vertical slicing.

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full specification

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
│   ├── shared/             # Taxonomy, contracts, utilities
│   └── tui/                # TUI file browser
├── PRD.md                  # Product requirements
├── ARCHITECTURE.md         # AES specification
└── README.md               # This file
```

## Available Commands

### Core Analysis


| Command                       | Description                                 |
| ------------------------------- | --------------------------------------------- |
| `lint-arwaky-cli scan [path]` | Run all linters (alias:`check`)             |
| `lint-arwaky-cli fix [path]`  | Apply safe fixes (`--dry-run` to preview)   |
| `lint-arwaky-cli ci [path]`   | CI mode with exit codes (`--threshold <n>`) |

### Individual Linter Surfaces


| Command                           | Description                                   |
| ----------------------------------- | ----------------------------------------------- |
| `lint-arwaky-cli quality [path]`  | Code-quality analysis only (AES101-AES306)    |
| `lint-arwaky-cli import [path]`   | Import-rule checks only (AES201-AES299)       |
| `lint-arwaky-cli naming [path]`   | Naming-rule checks only (AES401-AES406)       |
| `lint-arwaky-cli role [path]`     | Role-rule checks only (AES301-AES399)         |
| `lint-arwaky-cli orphan <path>`   | Check if file is dead/unreachable code        |
| `lint-arwaky-cli external [path]` | External linter checks (Clippy, Ruff, ESLint) |

### File Operations


| Command                        | Description                              |
| -------------------------------- | ------------------------------------------ |
| `lint-arwaky-cli watch [path]` | Watch directory and lint on file changes |

### Maintenance


| Command                               | Description                                |
| --------------------------------------- | -------------------------------------------- |
| `lint-arwaky-cli doctor`              | Environment diagnostics (toolchain health) |
| `lint-arwaky-cli security [path]`     | Security vulnerability scan                |
| `lint-arwaky-cli dependencies [path]` | Dependency report from manifests           |

### Git Hooks


| Command                          | Description                 |
| ---------------------------------- | ----------------------------- |
| `lint-arwaky-cli install-hook`   | Install git pre-commit hook |
| `lint-arwaky-cli uninstall-hook` | Remove git pre-commit hook  |

### Setup and Config


| Command                       | Description                                    |
| ------------------------------- | ------------------------------------------------ |
| `lint-arwaky-cli init`        | Create default lint config files               |
| `lint-arwaky-cli install`     | Install linter adapter dependencies (`--sudo`) |
| `lint-arwaky-cli mcp-config`  | Print MCP server config (`--client <type>`)    |
| `lint-arwaky-cli config-show` | Display active configuration                   |

### Info


| Command                    | Description                      |
| ---------------------------- | ---------------------------------- |
| `lint-arwaky-cli version`  | Display version                  |
| `lint-arwaky-cli adapters` | List active linters and adapters |

## Configuration

Configuration is loaded from YAML files.

```bash
# Create default config
lint-arwaky-cli init

# Show current config
lint-arwaky-cli config-show
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

See [DEPLOY.md](DEPLOY.md) for client setup and MCP tool reference.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT
