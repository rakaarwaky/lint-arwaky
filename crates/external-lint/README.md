# Lint Arwaky

Architecture linter enforcement for Rust, Python, and TypeScript. Built in Rust, structured by the [Agentic Engineering System](ARCHITECTURE.md), and self-auditing: the project lints itself under its own rules.

Most linters catch syntax and style. Lint Arwaky catches architecture drift: forbidden cross-layer imports, dead files, role confusion, unused imports, and bypass culture. It enforces 41 AES rules across 7 groups (naming, import, quality, role, orphan, structure, and doc) in Rust, Python, and TypeScript in a single scan.

> **Security scope:** Secret detection is delegated to specialist tools. Lint Arwaky ships a Gitleaks adapter that scans source files and git history for hardcoded secrets and credentials such as API keys, tokens, and passwords. Gitleaks findings surface as tool-native `gitleaks::<rule-id>` codes alongside `ruff::`, `clippy::`, and `eslint::` codes; they are not AES architecture rules.

## Project Overview

Lint Arwaky is a single-tool, multi-language architecture linter for monorepos. One scan reports 43 AES rule codes across 8 groups, drives CI exit codes, auto-fixes mechanical violations, and self-audits the project itself. It serves developers, AI agents, and DevOps/CI engineers on Rust/Python/TypeScript monorepos with 10k+ files.

## Supported Adapters

For the full supported adapter list and AES rule taxonomy, see [ARCHITECTURE.md](../../ARCHITECTURE.md).

Enable the gitleaks adapter with `- name: "gitleaks"` in `adapters:` of `lint_arwaky.config.yaml`. For full-history scanning, set `options: { history: "true" }`. Suppress individual findings with `ignored_rules: ["gitleaks::<rule-id>"]`; per-line `# gitleaks:allow` is not supported (deferred).

## Prerequisites

- Rust 1.85.0+ and Cargo (pinned via `rust-toolchain.toml`)
- Linux primary, macOS secondary
- For external linter adapters: `lint-arwaky-cli install`
- For MCP: a JSON-RPC 2.0 client

## Quick Start

| Step | Action                                                    | Command                                                                 |
|------|----------------------------------------------------------|------------------------------------------------------------------------|
| 1    | Install the pre-built binary (Linux x86_64)              | `curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/scripts/install.remote.sh \| bash` |
| 2    | Or build from source                                      | `git clone https://github.com/rakaarwaky/lint-arwaky.git && cd lint-arwaky && bash scripts/install.sh` |
| 3    | Verify the installation                                   | `lint-arwaky-cli version`                                              |
| 4    | Run all linters                                           | `lint-arwaky-cli scan .`                                              |
| 5    | Progress report against the last run                      | `lint-arwaky-cli report .`                                            |
| 6    | CI exit codes                                             | `lint-arwaky-cli ci . --threshold 0`                                   |
| 7    | Preview auto-fixes                                        | `lint-arwaky-cli fix . --dry-run`                                      |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](ARCHITECTURE.md) |
| Testing | How to run the suite and what a green run proves | [TEST.md](TEST.md) |
| Contributing | Branch, commit, and PR conventions | [CONTRIBUTING.md](CONTRIBUTING.md) |
| License | License | [MIT](LICENSE) |
