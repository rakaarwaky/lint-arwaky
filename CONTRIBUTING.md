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
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)
- [Questions?](#questions)

---

## Prerequisites

- **Rust** >= 1.85.0 (edition 2024, pinned via `rust-toolchain.toml`)
- **Cargo** (bundled with Rust)
- **Git**
- Familiarity with:
  - `clap` derive macros
  - JSON-RPC 2.0 (MCP protocol)
  - `std::thread` / `rayon` concurrency (no async runtime in the core; `rmcp`/tokio only in `file-watch` and `mcp-server`)

> Optional: `rustup` for toolchain management, `cargo-watch` for development.

---

## Setup

```bash
# Clone
git clone https://github.com/rakaarwaky/lint-arwaky.git
cd lint-arwaky

# Build everything (CARGO_INCREMENTAL=0 required for reproducible builds)
CARGO_INCREMENTAL=0 cargo build --release

# Run the CLI
./target/release/lint-arwaky-cli version
# Expected: lint-arwaky 3.6.1

# Run the MCP server in a separate terminal
./target/release/lint-arwaky-mcp
# Expected: "Listening on stdin/stdout (JSON-RPC 2.0)"

# Self-lint the project (must report 0 violations)
./target/release/lint-arwaky-cli check .
# Scans `crates/`, `modules/`, `packages/` under the AES rules this project enforces.
```

For development without the release profile:

```bash
cargo run --bin lint-arwaky-cli -- scan .
cargo run --bin lint-arwaky-mcp
```

---

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full 7-layer specification and naming conventions.

## Code Style

```bash
cargo fmt --all
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
```

### Quality Gates (run before every commit)

```bash
bash scripts/gates.sh                        # fmt + clippy + self-lint + tests
cargo nextest run --workspace --lib --tests   # all tests, 3× faster than cargo test
```

## Pull Request Process

See [TEST.md](TEST.md) for verification criteria.

### Branch Management

- Allowed branch naming: `main`, `develop`. Feature/fix branches are merged into `develop` via PR, then promoted to `main`.
- **Use a git worktree** under `.worktree/` (e.g. `<repo-root>/.worktree/feature-name`) instead of switching branches in the current checkout.
- Use `--delete-branch` when merging feature/fix PRs; never delete `develop` when merging to `main`.

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

- [ ] `bash scripts/gates.sh` passes (fmt + clippy + self-lint + tests)
- [ ] `cargo nextest run --workspace --lib --tests` passes
- [ ] `lint-arwaky-cli check .` reports 0 violations
- [ ] `cargo fmt --all` clean
- [ ] `CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings` clean
- [ ] Docs updated if needed
```

## Questions?

Open an issue on GitHub or contact the maintainer.
