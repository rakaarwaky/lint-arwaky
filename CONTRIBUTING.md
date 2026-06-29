# Contributing to Lint Arwaky

> This guide covers everything you need to start contributing

## Why Contribute

| Aspect                     | Benefit                                                        |
| -------------------------- | -------------------------------------------------------------- |
| **Real-world impact**      | Your code powers the same rule engine that audits this project |
| **Skill development**      | Practice Rust, async/tokio, MCP, and 7-layer architecture      |
| **Open-source experience** | Build portfolio with a self-auditing codebase                  |
| **Community**              | Join a project where every PR is checked by the rules it adds  |
| **Learning opportunity**   | Study a codebase that passes its own architecture linter       |

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Architecture](#architecture)
- [How to Add an Adapter](#how-to-add-an-adapter)
- [How to Add a CLI Command](#how-to-add-a-cli-command)
- [How to Add an MCP Tool](#how-to-add-an-mcp-tool)
- [Testing](#testing)
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)

---

## Prerequisites

- **Rust** >= 1.70 (edition 2021)
- **Cargo** (bundled with Rust)
- **Git**
- Familiarity with:
  - Rust async/await and `tokio`
  - `clap` derive macros
  - `mcp-sdk-rs` (or willingness to read the JSON-RPC 2.0 spec)

> Optional: `rustup` for toolchain management, `cargo-watch` for development.

---

## Setup

```bash
# Clone
git clone https://github.com/rakaarwaky/lint-arwaky.git
cd lint-arwaky

# Build everything
cargo build --release

# Run the CLI
./target/release/lint-arwaky-cli version
# Expected: Lint Arwaky v1.10.14 (AES Semantic Builder)

# Run the MCP server in a separate terminal
./target/release/lint-arwaky-mcp
# Expected: "Listening on stdin/stdout (JSON-RPC 2.0)"

# Self-lint the project
./target/release/lint-arwaky-cli check .
# Scans `crates/` under the same AES rules the project enforces.
```

For development without the release profile:

```bash
cargo run --bin lint-arwaky-cli -- check .
cargo run --bin lint-arwaky-mcp
```

---

## Architecture

### 7-Layer Model

The codebase is organized into **feature crates** (vertical slicing) in the `crates/` directory. Within each crate, files from different layers coexist and are identified strictly by their **file prefix**:

- `taxonomy_` — Value Objects (VOs), entities, constants, errors, events, utilities, helpers
- `contract_` — Interface layer: ports (I\*), protocols, aggregates
- `capabilities_` — Thinking layer: checkers, analyzers, processors, evaluators, resolvers, etc.
- `infrastructure_` — Toolbox layer: linter adapters, providers, scanners
- `agent_` — Wiring layer: orchestrators
- `surface_` — Interface layer: CLI commands, MCP handlers
- `root_` — Composition containers, main entries (root layer)

---

## Code Style

### Formatting

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

## Pull Request Process

### Before Submitting

1. **Run tests**: `cargo test --workspace`
2. **Run self-lint**: `cargo run --bin lint-arwaky-cli -- check .` — no CRITICAL findings.
3. **Format & lint**: `cargo fmt --all && cargo clippy --all-targets -- -D warnings`
4. **Update docs**: Ensure `README.md`, `SKILL.md`, `PRD.md`, and `CHANGELOG.md` reflect your changes.

### PR Description Template

```markdown
## What

Brief description of what this PR does.

## Why

Why is this change needed?

## How

How does it work? Any design decisions?

## Testing

How was it tested? What test cases were added?

## Checklist

- [ ] `cargo test --workspace` passes
- [ ] `cargo run --bin lint-arwaky-cli -- check .` reports 0 CRITICAL findings
- [ ] `cargo fmt --all` clean
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] Docs updated if needed
```

## Questions?

Open an issue on GitHub or contact the maintainer.
