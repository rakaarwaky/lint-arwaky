# Lint Arwaky

Architecture linter enforcement for Rust, Python, and TypeScript. Built in Rust, structured by the [Agentic Engineering System](ARCHITECTURE.md), and self-auditing — the project lints itself under its own rules.

Most linters catch syntax and style. Lint Arwaky catches architecture drift: forbidden cross-layer imports, dead files, role confusion, unused imports, and bypass culture. It enforces 32 AES rules across 7 groups (naming, import, quality, role, orphan, structure, and doc) in Rust, Python, and TypeScript in a single scan.

> **Security scope:** Lint Arwaky does not detect hardcoded secrets or
> credentials such as API keys, tokens, and passwords. Pair it with a dedicated
> secret scanner such as [gitleaks](https://github.com/gitleaks/gitleaks) or
> [TruffleHog](https://github.com/trufflesecurity/trufflehog). Secret detection
> is deliberately delegated to these specialist tools rather than added as an
> AES architecture rule.

## Prerequisites

- Rust 1.85.0+ and Cargo (pinned via `rust-toolchain.toml`)
- Linux primary, macOS secondary (Windows runtime excluded pending PRD Open Question #3 trade-off; cross-compilation target available)
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

For a guided, presenter-ready walkthrough using the bundled fixtures, see `TEST.md`
Section 5.5 "Demo Walkthrough".

## Available Scripts/Commands


| Command                                                      | Description                                                             |
| ------------------------------------------------------------ | ----------------------------------------------------------------------- |
| `scan` / `check` \[path\]                                                | Run all 7 code linters (naming, import, quality, role, orphan, structure, external) |
| `naming` / `import` / `quality` / `role` / `orphan` / `structure` \[path\] | Individual rule groups (AES101–102, 201–205, 301–305, 401–406, 501–506, 701–703) |
| `external` \[path\]                                          | External linters (Clippy, Ruff, ESLint, tool-native codes)              |
| `fix` \[path\]                                               | Apply safe fixes (`--dry-run` previews)                                 |
| `ci` \[path\]                                                | CI mode with exit codes (`--threshold <n>`)                             |
| `docs` \[path\]                                              | Audit document invariants (AES601–AES605)                               |
| `git` \[path\]                                               | Scan only files changed since a git base (`--base <ref>`)               |
| `watch` \[path\]                                             | Continuous linting on file changes                                      |
| `doctor` / `security` / `dependencies` \[path\]              | Toolchain diagnostics, cargo-audit scan, dependency report              |
| `install-hook` / `uninstall-hook`                            | Git pre-commit hook                                                     |
| `init` / `install` / `mcp-config` / `config`                 | Setup and config                                                        |
| `update` (alias `la`)                                        | Self-update to the latest release binary (`--check-only` previews)      |
| `version` / `adapters`                                       | Info                                                                    |
| `skill list`                                                 | List embedded AES skill documentation                                   |
| `skill read <name>`                                          | Print a skill's SKILL.md (`--with-references` adds language HOW-TOs)    |
| `lint-arwaky-tui`                                            | Start TUI                                                            |

Key bindings: `c` check, `s` scan, `f` fix dry-run, `F` fix live (gated), `/` search, `?` help overlay. Press `r` to change project root. Destructive actions (`F`, `H`, `U`) require explicit confirmation.

> **Known issue (#552):** confirming a gated destructive action (`F`, `H`, `U`, `install`,
> `init`) currently re-arms the confirmation gate instead of executing it. Do not rely on
> these in a live demo or production workflow until #552 closes — see `TEST.md` Section
> 5.5 for a demo script that works around this.

## Configuration

YAML with a 5-level priority chain: project root `lint_arwaky.config.yaml` → parent dirs (3 levels) → XDG user `~/.config/lint-arwaky/` → XDG system `/etc/xdg/lint-arwaky/` → embedded defaults. Generate with `lint-arwaky-cli init`; inspect with `config`.

### MCP Server

A [Model Context Protocol](https://modelcontextprotocol.io) server with full CLI parity and 5 tools: `execute_command`, `list_commands`, `read_skill`, `health_check`, `get_config`.

```bash
cargo run --bin lint-arwaky-mcp
lint-arwaky-cli mcp-config --client claude   # print client config
```

See [DEPLOY.md](DEPLOY.md) for client setup.

### Integrate as a CI Gate

Lint Arwaky drops into any Rust/Python/TS project. Structure code in `crates/`, `packages/`, `modules/`, then:

```bash
lint-arwaky-cli init      # creates lint_arwaky.config.yaml
lint-arwaky-cli install   # installs external linter deps
lint-arwaky-cli doctor    # verify toolchain health
```

Add a CI job running `lint-arwaky-cli check .` (exit 1 on any violation), make it a required status check, and add `lint-arwaky-cli ci . --threshold <score>` for score-based release gating. Full blueprint: [DEPLOY.md](DEPLOY.md) and [crates/shared/skills/aes-testing-suite/SKILL.md](crates/shared/skills/aes-testing-suite/SKILL.md).

## Architecture

7-layer Agentic Engineering System: taxonomy → contract → capabilities → utility → agent → surface → root. Every file is named `layer_concern_role` and dependencies flow down only. Full spec: [ARCHITECTURE.md](ARCHITECTURE.md).

### Performance

Two measured scopes, one number each: **filesystem indexing** (discovery + read + parse) 1,000 files &lt; 2s, 10,000 files &lt; 10s; **full pipeline** (indexing + every rule group, external adapters excluded) 1,000 files &lt; 5s, 10,000 files &lt; 15s. Within indexing: file discovery &lt; 500 ms; parallel AST parse &lt; 2 s. tree-sitter for all languages, rayon-parallel, no async runtime.

External adapters are not inside either budget — they are bounded only by their own per-tool ceilings (60–180s each, run sequentially), so a `scan` with every adapter present can take far longer than the pipeline budget. See `crates/external-lint/FRD.md`.

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
│   └── skills/        # Embedded skill content (source for init)
├── config-system/     # Config loading, merging, validation
├── filesystem/        # File walking, AST parsing, graph construction
├── naming-rules/      # AES101–102
├── import-rules/      # AES201–205
├── quality-rules/     # AES301–305
├── role-rules/        # AES401–406
├── orphan-rules/      # AES501–506
├── doc-rules/         # AES601–605 (document invariants)
├── structure-rules/   # AES701–703 (folder structure)
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

Each crate has an `FRD.md` (spec) beside a `BACKLOG.md` (real condition). Root: [PRD.md](PRD.md), [ROADMAP.md](ROADMAP.md), [ARCHITECTURE.md](ARCHITECTURE.md).

## Contributing

All code follows the AES 7-layer architecture (enforced by this tool). Acceptance tests named `acceptance_FR_00N.rs`. No `unwrap()`, `#[allow(...)]`, `todo!()`, `FIXME`, or `HACK` in production code. Full AST parsing only — no regex. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[MIT](LICENSE)
