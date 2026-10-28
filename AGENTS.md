---
trigger: always
description: "Lint Arwaky operational guide."
---

# AGENTS — Lint Arwaky

## Agent Overview

AI agents working in this workspace must know the AES 7-layer naming convention, the pinned Rust edition 2024 toolchain, and the exact CI gates before generating code, docs, or suggestions. ARCHITECTURE.md wins on ambiguity.

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

| Path       | Purpose              |
|------------|----------------------|
| crates/    | Rust workspace crates|
| shared/    | Shared kernel code   |

## Commands

| Category    | Command                              | Scope      | CI Gate              |
|-------------|--------------------------------------|------------|----------------------|
| Test (all)  | `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2`     | All        | ci.yml: Test         |
| Test (unit) | `CARGO_INCREMENTAL=0 cargo nextest run -p <crate>`                      | One unit   | ci.yml: Test         |
| Test (file) | `CARGO_INCREMENTAL=0 cargo nextest run -p <crate> --test <file>`        | One file   | ci.yml: Test         |
| Lint        | `CARGO_INCREMENTAL=0 cargo fmt --all -- --check`                         | All        | ci.yml: Format       |
| Type Check  | `CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings`          | All        | ci.yml: Clippy       |
| Arch Scan   | `lint-arwaky-cli check .`                                                | All        | ci.yml: Self-Lint    |

## Documentation Awareness

| Document | Agent Must                              |
|----------|-----------------------------------------|
| PRD      | Read before implementing any `PR-` item |
| FRD      | Follow `FR-` spec exactly               |
| DATA     | Respect `DT-` shapes and types          |
| DESIGN   | Use `DS-` tokens and variants           |
| ROADMAP  | Check `State` before starting work      |
| BACKLOG  | Update status after each task           |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | Business goals, user stories, and scope    | [PRD.md](PRD.md) |
| FRD          | Functional requirements specification      | [FRD.md](FRD.md) |
| DATA         | Data shapes and kernel definitions         | [DATA.md](DATA.md) |
| DESIGN       | UI/UX and visual system                    | [DESIGN.md](DESIGN.md) |
| Architecture | System boundaries, tech stack, and infra   | [ARCHITECTURE.md](ARCHITECTURE.md) |
