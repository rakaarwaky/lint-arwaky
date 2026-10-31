---
trigger: always
description: "Lint Arwaky operational guide."
---
# AGENTS — Lint Arwaky

## Agent Overview

AI agents working in this workspace must know the AES 7-layer naming convention, the pinned Rust edition 2024 toolchain, and the exact CI gates before generating code, docs, or suggestions. ARCHITECTURE.md wins on ambiguity.

## Runtime

| Aspect          | Detail                                      |
| --------------- | ------------------------------------------- |
| Language        | Rust edition 2024 via rust-toolchain.toml   |
| Environment     | cargo workspace under crates/               |
| Artifacts       | target/ and $HOME/.local/bin/               |
| Package Manager | cargo, with Cargo.lock                      |
| Version Probe   | `rustc --version`                           |
| Setup           | `CARGO_INCREMENTAL=0 cargo build --release` |
| Activation      | None                                        |

## Project Structure  

```text
.
├── crates/                            # Rust workspace crates
│   ├── auto-fix/                      # auto-fix engine (bypass, symbol rename, unused import)
│   ├── cli-commands/                  # CLI command handlers (surface)
│   ├── config-system/                 # config parsing + workspace detection
│   ├── dispatcher/                    # command dispatch + panic isolation (surface)
│   ├── doc-rules/                     # doc-shape + line-drift checks (AES601 family)
│   ├── external-lint/                 # shells out to clippy/ruff/eslint/mypy
│   ├── file-watch/                    # watch mode (notify-based)
│   ├── filesystem/                    # filesystem scanning / repo discovery
│   ├── git-hooks/                     # pre-commit / pre-push hook integration
│   ├── import-rules/                  # import graph validation (AES4xx)
│   ├── logging/                       # structured logging
│   ├── maintenance/                   # codebase maintenance checks
│   ├── mcp-server/                    # MCP protocol adapter (surface)
│   ├── naming-rules/                  # naming rule checks (AES1xx)
│   ├── orphan-rules/                  # orphan detection (AES2xx)
│   ├── project-setup/                 # scaffold / init
│   ├── quality-rules/                 # quality checks (AES5xx)
│   ├── report-formatter/              # violation report formatting
│   ├── role-rules/                    # role/layer enforcement (AES3xx)
│   ├── shared/                        # shared layer: contracts, VOs, doc-rules codegen, skill templates
│   ├── structure-rules/               # structure/doc checks (AES6xx/7xx)
│   └── tui/                           # terminal UI (surface)
├── workspaces-bad/                    # scanner fixtures: intentional AES violations
├── workspaces-good/                   # scanner fixtures: clean AES-conformant code
├── scripts/                           # release, install, gates, podman helpers
└── AGENTS.md                          # this file
```

## Commands

| Category    | Command                                                                | Scope    | CI Gate           |
| ----------- | ---------------------------------------------------------------------- | -------- | ----------------- |
| Test (all)  | `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2` | All      | ci.yml: Tests     |
| Test (unit) | `CARGO_INCREMENTAL=0 cargo nextest run -p <crate>`                     | One unit | ci.yml: Tests     |
| Test (file) | `CARGO_INCREMENTAL=0 cargo nextest run -p <crate> --test <file>`       | One file | ci.yml: Tests     |
| Lint        | `CARGO_INCREMENTAL=0 cargo fmt --all -- --check`                       | All      | ci.yml: Format    |
| Type Check  | `CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings`        | All      | ci.yml: Clippy    |
| Arch Scan   | `lint-arwaky-cli check .`                                              | All      | ci.yml: Self-Lint |

## Reference

| Artifact     | Purpose                                    | Link                               |
| ------------ | ------------------------------------------ | ---------------------------------- |
| README       | What the tool does, every command, install | [README.md](README.md)             |
| CONTRIBUTING | Branch, commit, and PR conventions         | [CONTRIBUTING.md](CONTRIBUTING.md) |
| ARCHITECTURE | Layer contract for every crate             | [ARCHITECTURE.md](ARCHITECTURE.md) |
| ROADMAP      | What ships next, in order                  | [ROADMAP.md](ROADMAP.md)           |
| PRD          | What the product does and why              | [PRD.md](PRD.md)                   |
