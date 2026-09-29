# Lint Arwaky

Architecture linter enforcement for Rust, Python, and TypeScript. Built in Rust, structured by the [Agentic Engineering System](ARCHITECTURE.md), and self-auditing — the project lints itself under its own rules.

Most linters catch syntax and style. Lint Arwaky catches architecture drift: forbidden cross-layer imports, dead files, role confusion, unused imports, and bypass culture. It enforces 29 AES rules across 6 groups in Rust, Python, and TypeScript in a single scan.

## Prerequisites

- Rust 1.85.0+ and Cargo (pinned via `rust-toolchain.toml`)
- Linux or macOS
- For external linter adapters: `lint-arwaky-cli install`

## Quick Start

```bash
# Pre-built binary (Linux x86_64)
curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/scripts/install.remote.sh | bash

# Or build from source
git clone https://github.com/rakaarwaky/lint-arwaky.git && cd lint-arwaky && bash scripts/install.sh
```

Verify: `lint-arwaky-cli version`. First scan:

```bash
lint-arwaky-cli scan .            # run all linters
lint-arwaky-cli ci . --threshold 0   # CI exit codes
lint-arwaky-cli fix . --dry-run      # preview auto-fixes
```

## Available Scripts/Commands

| Command | Description |
|---------|-------------|
| `scan` / `check` [path] | Run all 6 linters |
| `naming` / `import` / `quality` / `role` / `orphan` [path] | Individual rule groups (AES101–102, 201–205, 301–305, 401–406, 501–506) |
| `external` [path] | External linters (Clippy, Ruff, ESLint, tool-native codes) |
| `fix` [path] | Apply safe fixes (`--dry-run` previews) |
| `ci` [path] | CI mode with exit codes (`--threshold <n>`) |
| `docs` [path] | Audit document invariants (AES601–AES605) |
| `watch` [path] | Continuous linting on file changes |
| `doctor` / `security` / `dependencies` [path] | Toolchain diagnostics, cargo-audit scan, dependency report |
| `install-hook` / `uninstall-hook` | Git pre-commit hook |
| `init` / `install` / `mcp-config` / `config-show` | Setup and config |
| `version` / `adapters` | Info |
| `skill list` | List embedded AES skill documentation |
| `skill read <name>` | Print a skill's SKILL.md (`--with-references` adds language HOW-TOs) |

Exit codes: `0` Ok · `1` policy fail · `2` runtime error · `3` prerequisite missing.

### TUI

An interactive terminal UI with file browser, lint results preview, search, and keyboard-driven lint actions.

```bash
lint-arwaky-tui              # start TUI
NO_COLOR=1 lint-arwaky-tui   # ASCII fallback for light terminals
```

Key bindings: `c` check, `s` scan, `f` fix dry-run, `F` fix live (gated), `/` search, `?` help overlay. Press `r` to change project root. Destructive actions (`F`, `H`, `U`) require explicit confirmation.

## AES Rules (29)

Six groups: **Naming** AES101–102, **Import** AES201–205, **Quality** AES301–305, **Role** AES401–406, **Orphan** AES501–506, and **Folder Structure** AES701–705. Full definitions: [RULES_AES.md](RULES_AES.md). External linter results use tool-native codes (e.g. `clippy::needless_return`) and are reported alongside the 29 AES rules.

## Configuration

YAML with a 5-level priority chain: project root `lint_arwaky.config.yaml` → parent dirs (3 levels) → XDG user `~/.config/lint-arwaky/` → XDG system `/etc/xdg/lint-arwaky/` → embedded defaults. Generate with `lint-arwaky-cli init`; inspect with `config-show`.

## MCP Server

A [Model Context Protocol](https://modelcontextprotocol.io) server with full CLI parity and 5 tools: `execute_command`, `list_commands`, `read_skill`, `health_check`, `get_config`.

```bash
cargo run --bin lint-arwaky-mcp
lint-arwaky-cli mcp-config --client claude   # print client config
```

See [DEPLOY.md](DEPLOY.md) for client setup.

## Integrate as a CI Gate

Lint Arwaky drops into any Rust/Python/TS project. Structure code in `crates/`, `packages/`, `modules/`, then:

```bash
lint-arwaky-cli init      # creates lint_arwaky.config.yaml
lint-arwaky-cli install   # installs external linter deps
lint-arwaky-cli doctor    # verify toolchain health
```

Add a CI job running `lint-arwaky-cli check .` (exit 1 on any violation), make it a required status check, and add `lint-arwaky-cli ci . --threshold <score>` for score-based release gating. Full blueprint: [DEPLOY.md](DEPLOY.md) and [crates/skills/aes-testing-suite/SKILL.md](crates/skills/aes-testing-suite/SKILL.md).

## Architecture

7-layer Agentic Engineering System: taxonomy → contract → capabilities → utility → agent → surface → root. Every file is named `layer_concern_role` and dependencies flow down only. Full spec: [ARCHITECTURE.md](ARCHITECTURE.md).

## Performance

1,000 files < 5s; 10,000 files < 15s (full pipeline). File discovery < 500 ms; parallel AST parse < 2 s. tree-sitter for all languages, rayon-parallel, no async runtime.

## Testing

```bash
cargo nextest run --workspace --lib --tests   # 3× faster than cargo test
cargo run --bin lint-arwaky-cli -- check .    # self-lint, must report 0 violations
```

Acceptance tests follow `tests/acceptance_FR_00N.rs`. Pass/fail criteria: [TEST.md](TEST.md).

## Project Structure

```
crates/
├── shared/            # Taxonomy VOs, contracts, utilities
├── config-system/     # Config loading, merging, validation
├── filesystem/        # File walking, AST parsing, graph construction
├── naming-rules/      # AES101–102
├── import-rules/      # AES201–205
├── quality-rules/     # AES301–305
├── role-rules/        # AES401–406
├── orphan-rules/      # AES501–506
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
├── tui/               # Interactive terminal UI
└── skills/            # Embedded skill content (source for init)
```

Each crate has an `FRD.md` (spec) beside a `BACKLOG.md` (real condition). Root: [PRD.md](PRD.md), [ROADMAP.md](ROADMAP.md), [ARCHITECTURE.md](ARCHITECTURE.md).

## Contributing

All code follows the AES 7-layer architecture (enforced by this tool). Acceptance tests named `acceptance_FR_00N.rs`. No `unwrap()`, `#[allow(...)]`, `todo!()`, `FIXME`, or `HACK` in production code. Full AST parsing only — no regex. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[MIT](LICENSE)
