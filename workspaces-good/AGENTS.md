---
trigger: always
description: "Calculator workspace operational guide."
---

# AGENTS — Calculator Workspace

## Agent Overview

AI agents working in this workspace must know the AES 7-layer naming convention, the pinned Rust edition 2024 toolchain, and the exact CI gates before generating code, docs, or suggestions.

## Runtime

| Aspect          | Detail                                          |
|-----------------|-------------------------------------------------|
| Language        | Rust (edition 2024 via rust-toolchain.toml)     |
| Environment     | cargo workspace under crates/                   |
| Artifacts       | target/ and $HOME/.local/bin/                   |
| Package Manager | cargo, with Cargo.lock                          |
| Version Probe   | `rustc --version`                               |
| Setup           | `CARGO_INCREMENTAL=0 cargo build --release`     |
| Activation      | None                                            |

## Project Structure

```text
.
├── crates/            # Rust workspace members
│   ├── calculator/    # feature: calculator (agent + capabilities + root)
│   ├── cli/           # surface: CLI entry
│   ├── shared/        # shared kernel: contracts + VOs
│   └── single_orchestrator/
├── modules/           # Python feature layers
│   ├── addition/
│   ├── calculator/
│   ├── division/
│   ├── multiplication/
│   ├── subtraction/
│   └── shared/
├── packages/          # TypeScript feature layers
│   ├── addition/
│   ├── calculator/
│   ├── division/
│   ├── multiplication/
│   ├── subtraction/
│   └── shared/
└── AGENTS.md          # this file
```

## Commands

| Category    | Command                              | Scope      | CI Gate              |
|-------------|--------------------------------------|------------|----------------------|
| Test (all)  | `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2`     | All        | ci.yml: Test         |
| Test (unit) | `CARGO_INCREMENTAL=0 cargo nextest run -p <crate>`                      | One unit   | ci.yml: Test         |
| Test (file) | `CARGO_INCREMENTAL=0 cargo nextest run -p <crate> --test <file>`        | One file   | ci.yml: Test         |
| Lint        | `CARGO_INCREMENTAL=0 cargo fmt --all -- --check`                         | All        | ci.yml: Format       |
| Type Check  | `CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings`          | All        | ci.yml: Clippy       |
| Arch Scan   | `lint-arwaky-cli check .`                                                | All        | ci.yml: Self-Lint    |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| README       | What the tool does, every command, install | [README.md](README.md) |
| CONTRIBUTING | Branch, commit, and PR conventions         | [CONTRIBUTING.md](CONTRIBUTING.md) |
| ARCHITECTURE | Layer contract for every crate             | [ARCHITECTURE.md](ARCHITECTURE.md) |
| ROADMAP      | What ships next, in order                  | [ROADMAP.md](ROADMAP.md) |
| PRD          | What the product does and why              | [PRD.md](PRD.md) |
