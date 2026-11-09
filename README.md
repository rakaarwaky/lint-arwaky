# Lint Arwaky

## Project Overview

Lint Arwaky is a single-tool, multi-language architecture linter for monorepos. One scan reports 41 AES rule codes across 7 groups (naming, import, quality, role, orphan, structure, and doc), drives CI exit codes, auto-fixes mechanical violations, and self-audits the project itself. It serves developers, AI agents, and DevOps/CI engineers on Rust/Python/TypeScript monorepos with 10k+ files.

## Prerequisites

- Rust 1.85.0+ and Cargo (pinned via `rust-toolchain.toml`)
- Linux primary, macOS secondary
- For external linter adapters: `lint-arwaky-cli install`
- For MCP: a JSON-RPC 2.0 client

## Quick Start


| Step | Action                                       | Command                                                                                                    |
| ---- | -------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| 1    | Install the pre-built binary (Linux x86\_64) | `curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/scripts/install.remote.sh | bash` |
| 2    | Or build from source                         | `git clone https://github.com/rakaarwaky/lint-arwaky.git && cd lint-arwaky && bash scripts/install.sh`     |
| 3    | Verify the installation                      | `lint-arwaky-cli version`                                                                                  |
| 4    | Browse the workspace interactively           | `lint-arwaky-tui` (three panels: tree, files, lint output)                                                 |
| 5    | Run all linters                              | `lint-arwaky-cli scan .`                                                                                   |
| 6    | Progress report against the last run         | `lint-arwaky-cli report .`                                                                                 |
| 7    | CI exit codes                                | `lint-arwaky-cli ci . --threshold 0`                                                                       |
| 8    | Preview auto-fixes                           | `lint-arwaky-cli fix . --dry-run`                                                                          |


## Reference


| Artifact     | Purpose                                          | Link                               |
| ------------ | ------------------------------------------------ | ---------------------------------- |
| Architecture | System boundaries, tech stack, and infra         | [ARCHITECTURE.md](ARCHITECTURE.md) |
| TUI design   | Tokens, panel geometry, and interaction contract | [crates/tui/DESIGN.md](crates/tui/DESIGN.md) |
| Testing      | How to run the suite and what a green run proves | [TEST.md](TEST.md)                 |
| Contributing | Branch, commit, and PR conventions               | [CONTRIBUTING.md](CONTRIBUTING.md) |
| License      | License                                          | [MIT](LICENSE)                     |
