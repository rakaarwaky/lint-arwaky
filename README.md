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
| 4    | Browse the workspace interactively           | `lint-arwaky-tui`                                                                                          |
| 5    | Run all linters                              | `lint-arwaky-cli scan .`                                                                                   |
| 6    | Progress report against the last run         | `lint-arwaky-cli report .`                                                                                 |
| 7    | CI exit codes                                | `lint-arwaky-cli ci . --threshold 0`                                                                       |
| 8    | Preview auto-fixes                           | `lint-arwaky-cli fix . --dry-run`                                                                          |

## Performance

### Budget

| Metric | Target | Current status |
| --- | --- | --- |
| `lint-arwaky-cli scan .` p95 on a 10k-file tree (4-core CI runner) | < 60 s | **Not yet measured** — see PRD.md metric #2 |
| `lint-arwaky-cli scan .` on a 546-file fixture (`workspaces-bad`) | — | Component benches cover the pipeline (see below) |

### How it is measured

- **Nightly Criterion regression run** — `.github/workflows/benchmarks.yml`
  runs `cargo bench --workspace` on a schedule and compares against a cached
  baseline; regressions are reported on the run summary. Advisory only: it
  does not block PRs (per-PR benchmarking on shared runners is too noisy).
- **Component benches** — `crates/*/benches/` cover tree-sitter parse
  throughput (100–1000 files), graph construction, duplication analysis,
  naming and role audit throughput (10–100 files), and unused-import
  detection (10–200 imports per file).
- **Phase timings** — `lint-arwaky-cli scan -v` emits per-phase `elapsed_ms`
  tracing events (index build, discovery, per-linter-group, external
  adapters) on stderr; stdout stays clean for piping.

### Known gaps

- No end-to-end scan benchmark on a generated 10k-file fixture yet; the 60 s
  p95 target is unverified (PRD.md metric #2, DEPLOY.md "Known process gaps").
- The five file-scoped linter groups (quality, role, import, naming, orphan)
  run on parallel threads since the issue #1365 fix; external adapters,
  structure, and doc audits run after them. The TUI cancel path
  (`Esc`) checks for cancellation between groups.

## Reference


| Artifact     | Purpose                                          | Link                               |
| ------------ | ------------------------------------------------ | ---------------------------------- |
| Architecture | System boundaries, tech stack, and infra         | [ARCHITECTURE.md](ARCHITECTURE.md) |
| TUI design   | Tokens, panel geometry, and interaction contract | [crates/tui/DESIGN.md](crates/tui/DESIGN.md) |
| Testing      | How to run the suite and what a green run proves | [TEST.md](TEST.md)                 |
| Contributing | Branch, commit, and PR conventions               | [CONTRIBUTING.md](CONTRIBUTING.md) |
| License      | License                                          | [MIT](LICENSE)                     |
