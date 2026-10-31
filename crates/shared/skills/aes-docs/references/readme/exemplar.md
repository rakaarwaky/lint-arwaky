# README Exemplar

```markdown
# README — Ecommerce Application

## Project Overview

Ecommerce Application is a cross-functional project that lets customers browse products, add items to a cart, and place orders online, while merchants list products and manage orders through a seller dashboard.

## Prerequisites

- Rust toolchain (edition 2024, pinned by rust-toolchain.toml)
- cargo with Cargo.lock

## Quick Start

| Step | Action | Command  |
|------|--------|----------|
| 1 | Build the workspace | `CARGO_INCREMENTAL=0 cargo build --release` |
| 2 | Run all tests | `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2` |
| 3 | Format check | `CARGO_INCREMENTAL=0 cargo fmt --all -- --check` |
| 4 | Clippy check | `CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings` |
| 5 | Architecture scan | `lint-arwaky-cli check .` |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| README   | What the tool does, every command, install | [README.md](README.md) |
| CONTRIBUTING | Branch, commit, and PR conventions | [CONTRIBUTING.md](CONTRIBUTING.md) |
| ARCHITECTURE | Layer contract for every crate | [ARCHITECTURE.md](ARCHITECTURE.md) |
| ROADMAP | What ships next, in order | [ROADMAP.md](ROADMAP.md) |
| PRD | What the product does and why | [PRD.md](PRD.md) |

```
