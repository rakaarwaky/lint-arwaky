# PRD — Lint Arwaky

> Product Requirements. Describes WHAT this project does and WHY. Real condition lives in [BACKLOG.md](BACKLOG.md).

## Problem Statement

Software projects accumulate quality debt silently. Developers lack a single tool that audits Rust, Python, and TypeScript together, enforces architectural rules, and works for both human developers and AI agents.

## Goals & Success Metrics

| # | Goal | Measurement | Target |
|---|------|-------------|--------|
| 1 | Multi-language linting in a single pass | `scan` on mixed-language workspace | violations from all 3 languages |
| 2 | 24 AES rules enforced across 5 groups | `workspaces-bad` scan | all 24 rule codes produce violations |
| 3 | MCP server with 5 tools, full CLI parity | `execute_command` on every CLI command | all commands reachable |
| 4 | Self-auditing | `lint-arwaky-cli check .` on this repo | 0 violations |

## User Personas

- **AI Agent**: Autonomous linting, self-healing via MCP tools.
- **Developer**: Lint codebases locally; enforce architecture during development.
- **DevOps / CI Engineer**: Quality gates, trend reports, dependency scans with stable exit codes.

## Scope

- **In scope**: CLI binary, MCP server, TUI, 24 AES rules, external linter adapters, SARIF/JUnit/JSON reports, git hooks, auto-fix (remove + replace + rename).
- **Out of scope**: IDE plugins, web dashboard, cloud SaaS, non-Rust implementation, structural/multi-file semantic refactors in auto-fix.

## Product Decisions (locked)

| Topic | Decision |
|-------|----------|
| Auto-fix safety | Remove + replace + rename only; no structural or multi-file edits |
| Surface parity | MCP, CLI, and TUI expose the same commands |
| MCP tools | 5: `execute_command`, `list_commands`, `read_skill`, `health_check`, `get_config` |
| Acceptance tests | Named `acceptance_FR_00N.rs` |
| Doctor exit code | 0 when diagnostic completes; 2 only on internal failure |
| Auto-fix outcomes | Reason-coded: `Applied` / `Skipped(reason)` / `Failed(reason)` |
| Concurrency | `std::thread` / `rayon`; async only in file-watch and mcp-server |
| Parsing | Full AST via tree-sitter; no regex fallback |
| Filesystem I/O | Centralized in filesystem crate |

## Exit Code Contract

| Code | Name | When |
|------|------|------|
| 0 | Ok | Success; clean scan; doctor finished; dry-run completed |
| 1 | Policy fail | Violations found; CI threshold failed; vulnerabilities found |
| 2 | Runtime error | Path missing; pipeline crash; invalid args; I/O failure |
| 3 | Prerequisite missing | Required external tool not installed |

MCP JSON responses SHOULD include `exit_code` aligned with this contract.

## AES Rule Summary (24 Rules)

Five groups: **Naming** (AES101–102, 2), **Import** (AES201–205, 5), **Quality** (AES301–305, 5), **Role** (AES401–406, 6), **Orphan** (AES501–506, 6). Full rule definitions: [RULES_AES.md](RULES_AES.md).

## Feature Requirements (Prioritized)

> Real condition for each feature lives in [BACKLOG.md](BACKLOG.md). This section is specification only.

### P0 — Must Have

- Multi-language scanning (Rust, Python, JS/TS). Acceptance: `scan` on mixed-language workspace returns violations from all three.
- 24 AES rules enforcement. Acceptance: `workspaces-bad` produces violations for all 24 rule codes.
- CLI with `check`, `scan`, `fix`, `ci` commands. Acceptance: each exits with the correct code from the contract above.
- MCP server with 5 tools, full execute parity. Acceptance: every CLI command is reachable via `execute_command`.
- Self-auditing capability. Acceptance: `lint-arwaky-cli check .` on this repo reports 0 violations.

### P1 — Should Have

- External linter adapters (Clippy, Ruff, ESLint, etc.). Acceptance: `adapters` lists all installed tools.
- SARIF 2.1.0, JUnit XML, JSON reports. Acceptance: `--format sarif|junit|json` produces valid output.
- Git hooks integration. Acceptance: `install-hook` creates a working pre-commit hook.
- Auto-fix capabilities (remove + replace + rename). Acceptance: `fix` applies mechanical fixes; `--dry-run` previews.
- Watch mode for continuous linting. Acceptance: `watch` re-lints on file save.
- TUI file browser. Acceptance: TUI renders file tree, runs lint actions, exits cleanly.
- Workspace exit-code contract enforced everywhere. Acceptance: all commands return codes 0/1/2/3 per contract.
- Acceptance tests standardized to `acceptance_FR_00N.rs`.

### P2 — Nice to Have

- Windows support. Acceptance: build + test suite passes on a Windows CI runner.
- Deeper monorepo performance optimizations. Acceptance: 10k-file scan completes in < 10s on CI hardware.

## Open Questions / Risks

| # | Question / Risk | Owner | Deadline | Status |
|---|-----------------|-------|----------|--------|
| 1 | Is 10k-file scan < 10s achievable on the CI runner tier? | @raka | v3.7.0 | open |
| 2 | Auto-fix: semantic rewrites allowed, or mechanical-only? | @raka | v3.7.0 | locked: mechanical-only |
| 3 | Windows: dedicated CI runner or cross-compile only? | @raka | post-v3.x | open |

## Non-functional Requirements (High-level)

| Category | Commitment | Detail lives in |
|----------|------------|-----------------|
| Performance | 1,000 files < 5s; 10,000 files < 10s | `filesystem/FRD.md` |
| Security | No network calls for core; symlink safety enforced | `filesystem/FRD.md` |
| Scalability | 10,000+ file monorepos | `filesystem/FRD.md` |
| Platform | Linux primary, macOS secondary | `README.md` |
| Concurrency | std::thread / rayon; async only in file-watch, mcp-server | `dispatcher/FRD.md` |
| Parsing | Full AST via tree-sitter; no regex fallback | `filesystem/FRD.md` |
| Diagnostics | `PARSE_WARN` for files that fail to parse | `filesystem/FRD.md` |

## Feature Map

Crate responsibilities are listed in [AGENTS.md](AGENTS.md#workspace-packages-structure) and each crate's `FRD.md` under `crates/`.

## Reference

- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md) · Testing: [TEST.md](TEST.md) · Contributing: [CONTRIBUTING.md](CONTRIBUTING.md) · Backlog: [BACKLOG.md](BACKLOG.md)
