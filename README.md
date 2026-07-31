# Lint Arwaky

## The Last Code Quality Tool You Will Ever Need

Your codebase is not a draft. It is the product your team ships, the system your users depend on, and the asset your company builds on. Yet every day, silent architecture decay erodes it — dead imports, broken layer boundaries, orphan files, naming inconsistencies, and role violations that no one notices until they cause an outage or block a release.

Lint Arwaky stops that decay before it starts.

Built in Rust for speed, structured by the Agentic Engineering System specification, and designed to enforce architecture the way a compiler enforces syntax — Lint Arwaky does not ask you to care about code quality. It makes it impossible to ignore.

---

## Why This Exists

Most linters catch syntax errors or style mistakes. They do not catch:

- **Architecture drift.** Files living where they do not belong, breaking layer boundaries, or importing from crates they should never touch.
- **Orphan code.** Files that no agent, module, or import can reach — dead weight silently growing in your repository.
- **Role confusion.** Agents writing business logic, surfaces depending on lower layers, capabilities implementing protocols instead of orchestrating them.
- **Import chaos.** Circular dependencies, forbidden imports, unused modules — the kind of dependency mess that makes refactoring terrifying.
- **Naming decay.** Structs named with underscores, functions without verb prefixes, enums that read like variables.

Lint Arwaky catches all of these. And it does so fast enough to run on every commit, in CI, and during development — without making developers wait.

---

## What You Get

### Architecture Enforcement

The AES specification defines seven strict layers. Lint Arwaky enforces them. If a surface module imports directly from the capabilities layer, it flags it. If an agent file contains business logic, it catches it. Your architecture is not a document no one reads — it is a live gate that blocks violations.

### Multi-Language Coverage

Rust crates, Python modules, TypeScript packages — Lint Arwaky scans all three. One tool. One configuration language. No more juggling linters per language or maintaining separate quality pipelines.

### Orphan Detection

It builds a dependency graph across your entire workspace and tells you exactly which files are unreachable. Not "maybe dead." Not "probably unused." It traces every import, every module reference, and returns the truth.

### External Lint Integration

Clippy, Ruff, ESLint, mypy, bandit — Lint Arwaky wraps them all into a single report with unified exit codes and threshold gates. You set the bar. The tool enforces it.

### Auto-Fix

For violations that have unambiguous solutions, Lint Arwaky does not just report — it fixes. With `--dry-run` you preview every change before it touches your files. Zero risk, maximum gain.

### Git Hooks and CI Gates

Install a pre-commit hook and never push architecture violations again. Configure `ci --threshold 0` and let your pipeline reject commits that break the contract. This is not a suggestion system. It is enforcement.

---

## How Fast Is It

Rust compilation means cold starts under two seconds. Incremental builds mean subsequent scans are nearly instant. The gate pipeline runs format checks, builds once, runs clippy, then executes self-lint, tests, and AES code verification in parallel. The entire quality gate suite completes in roughly two minutes — fast enough to run on every pull request without friction.

---

## Quick Start

Two install options are available:

### Option 1: Remote Install (fast — no build, no clone)

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

---

## Architecture

Lint Arwaky follows its own AES (Agentic Engineering System) specification — a strict layered architecture with seven layers, organized into feature vertical slicing.

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
