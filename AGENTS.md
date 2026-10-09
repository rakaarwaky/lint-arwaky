---
trigger: always
description: "Lint Arwaky operational guide. ARCHITECTURE.md wins on ambiguity."
---
# Lint Arwaky

## Runtime

- Language: Rust (edition 2024, pinned via rust-toolchain.toml)
- Environment: cargo workspace under crates/, built on Linux/macOS with CARGO_INCREMENTAL=0
- Artifacts: target/ and $HOME/.local/bin/ (install path). Never use /tmp for
  build output a reviewer must find
- Package manager: cargo, with Cargo.lock

```bash
rustc --version
CARGO_INCREMENTAL=0 cargo build --release
```

## Project Quick Facts

INPUT  = Rust workspace under crates/ with AES 7-layer naming
OUTPUT = 0 violations from lint-arwaky-cli check ., docs ., and scripts/gates.sh

## Pipeline

scan → check → gates → publish

## Project Structure

```text
crates/
├── shared/            # Taxonomy VOs, contracts, utilities
│   └── skills/        # Embedded skill content (source for init)
├── config-system/     # Config loading, merging, validation
├── filesystem/        # File walking, AST parsing, graph construction
├── naming-rules/      # AES101–102
├── import-rules/      # AES201–205
├── quality-rules/     # AES301–305
├── role-rules/        # AES401–406
├── orphan-rules/      # AES501–506
├── doc-rules/         # AES601–605 (document invariants)
├── structure-rules/   # AES701–705 (folder structure)
├── external-lint/     # External linter adapters
├── auto-fix/          # Mechanical fixes
├── report-formatter/  # text/JSON/SARIF/JUnit output
├── dispatcher/        # Utility Surface — business logic
├── cli-commands/      # CLI surface
├── mcp-server/        # MCP server
├── git-hooks/         # Pre-commit / git-diff
├── file-watch/        # Continuous lint
├── project-setup/     # init / install / mcp-config
├── maintenance/       # doctor / security / deps
└── tui/               # Interactive terminal UI
```

## Commands

Every command must match the exact CI gate.

```bash
# Tests
CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2   # all tests (3x faster than cargo test)
CARGO_INCREMENTAL=0 cargo nextest run -p <crate>                      # one unit
CARGO_INCREMENTAL=0 cargo nextest run -p <crate> --test <file>        # one file

# Lint / types / architecture
CARGO_INCREMENTAL=0 cargo fmt --all -- --check                         # matches ci.yml "Format" job
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings          # matches ci.yml "Clippy" job
lint-arwaky-cli check .                                                # architecture scanner (self-lint)
```

## Related Documents

- [README.md](README.md): What the tool does, every command, and how to install it
- [PRD.md](PRD.md): What the product does and why — feature tiers, exit codes, non-functional goals
- [ARCHITECTURE.md](ARCHITECTURE.md): Layer contract for every crate and the rules that keep the boundaries
- [RULES_AES.md](RULES_AES.md): The 32 rules, what each one forbids, and why
- [ROADMAP.md](ROADMAP.md): What ships next, in order
- [TEST.md](TEST.md): How to run the suite and what a green run proves
- [CONTRIBUTING.md](CONTRIBUTING.md): Branch, commit, and PR conventions
- [DEPLOY.md](DEPLOY.md): Release path and the checks it gates on
- [SECURITY.md](SECURITY.md): Reporting a vulnerability and what counts as one

---
