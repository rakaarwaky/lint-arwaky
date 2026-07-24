# PRD — Lint Arwaky

## Problem Statement

Software projects accumulate quality debt silently. Developers lack a single tool that audits Rust + Python + JavaScript/TypeScript together, enforces architectural rules, and works for both human developers (CLI) and AI agents (MCP tools).

## Goals & Success Metrics

- Goal 1: Multi-language linting in a single pass (Rust, Python, JS/TS)
- Goal 2: 24 AES rules enforced across 5 groups (Naming, Import, Quality, Role, Orphan)
- Goal 3: MCP server with **5 tools** for autonomous AI-agent integration (full CLI parity on execute)
- Goal 4: Zero bypass tolerance (`noqa`, `type: ignore`, `#[allow(...)]` flagged)
- Goal 5: Self-auditing — project lints itself under its own rule engine

## User Personas

- **AI Agent**: Autonomous linting, self-healing, code review via MCP tools (same capability as CLI)
- **Developer**: Lint codebases, enforce architecture during local development
- **DevOps / CI**: Quality gates, trend reports, dependency scans (stable exit codes)
- **Contributor**: Extend adapters, add CLI commands
- **Reviewer**: Architecture audit, code quality analysis

## Scope

- In scope:

  - CLI binary (`lint-arwaky-cli`) for human developers
  - MCP server (`lint-arwaky-mcp`) for AI agents — **full parity** with CLI actions
  - TUI file browser (`lint-arwaky-tui`) — **P1 supported surface**
  - 24 AES rules across 5 groups (plus non-AES internal diagnostics such as `DIAG_IO`)
  - External linter adapters (Clippy, Ruff, MyPy, Bandit, ESLint, Prettier, TSC)
  - SARIF 2.1.0, JUnit XML, JSON reports
  - Git hooks integration
  - Auto-fix: **remove + replace + rename** (safe mechanical fixes only)

- Out of scope:

  - IDE plugins (VS Code, IntelliJ)
  - Web dashboard
  - Cloud-hosted SaaS
  - Non-Rust implementation
  - Structural / multi-file semantic refactors in auto-fix

## Product Decisions (locked)

| Topic | Decision |
| ----- | -------- |
| Auto-fix safety | **Remove + replace + rename** (not removal-only; not full rewrite) |
| MCP vs CLI | **Full parity** — no silent stubs; unsupported actions return explicit errors |
| MCP tools | **5 tools**: `execute_command`, `list_commands`, `read_skill`, `health_check`, **`get_config`** |
| Exit codes | **0 / 1 / 2 / 3** (see Exit Code Contract) |
| TUI priority | **P1** — supported surface; critical-path acceptance required |
| File-read failures | **Internal diagnostic `DIAG_IO`** — not part of the 24 AES rules |
| Acceptance tests | Filename standard: `acceptance_FR_00N.rs` |
| Doctor command | Exit **0** when diagnostic completes (missing tools listed in body); exit **2** only on internal failure |
| Auto-fix outcomes | **Reason-coded** results (`Applied` / `Skipped(reason)` / `Failed(reason)`), not bare bool |

## Exit Code Contract

| Code | Name | When |
| ---- | ---- | ---- |
| **0** | Ok | Success; clean scan; doctor finished (even if tools missing); dry-run completed |
| **1** | Policy fail | Violations found; CI threshold failed; vulnerabilities found; remaining issues after fix |
| **2** | Runtime error | Path missing; pipeline crash; invalid args; I/O failure of the command itself |
| **3** | Prerequisite missing | Required external tool not installed (e.g. cargo-audit for `security`) |

MCP JSON responses SHOULD include `exit_code` aligned with this contract.

## Feature Requirements (Prioritized)

### P0 — Must Have

- [x] Multi-language scanning (Rust, Python, JS/TS) — `code-analysis`, `import-rules`, `naming-rules`, `external-lint`
- [x] 24 AES rules enforcement — RULES_AES groups 1–5
- [x] CLI with `check`, `scan`, `fix`, `ci` commands — `cli-commands`
- [ ] MCP server with 5 tools + **full execute parity** (no stubs) — `mcp-server`
- [x] Self-auditing capability — workspace scans itself

### P1 — Should Have

- [x] External linter adapters (Clippy, Ruff, MyPy, Bandit, ESLint, Prettier, TSC)
- [x] SARIF 2.1.0, JUnit XML, JSON reports — `report-formatter`
- [x] Git hooks integration — `git-hooks`
- [x] Auto-fix capabilities (remove + replace + rename) — `auto-fix` (harden reason-coded outcomes)
- [x] Watch mode for continuous linting — `file-watch`
- [ ] TUI file browser (critical-path acceptance) — `tui`
- [ ] Workspace exit-code contract enforced everywhere — `cli-commands`, `maintenance`
- [ ] Acceptance tests standardized to `acceptance_FR_00N.rs`

### P2 — Nice to Have

- [ ] Windows support
- [ ] Deeper monorepo performance optimizations

## Feature Map (crate → responsibility)

| Crate | Primary value | Key FR focus |
| ----- | ------------- | ------------ |
| `shared` | Taxonomy VOs, contracts, utilities | Foundation types |
| `config-system` | Config load, merge, workspace detect | FR-001–010 |
| `code-analysis` | AES301–305 quality rules | FR-001–006 (`DIAG_IO`) |
| `import-rules` | AES201–205 import boundaries | FR-001–005 |
| `naming-rules` | AES101–102 naming | FR-001–002 |
| `role-rules` | AES401–406 layer roles | FR-001–008 |
| `orphan-detector` | AES501–506 orphan detection | FR-001–011 |
| `external-lint` | Clippy/Ruff/ESLint adapters | FR-001–007 |
| `auto-fix` | Mechanical fixes | FR-001–005 |
| `report-formatter` | text/json/sarif/junit | FR-001–007 |
| `cli-commands` | Human CLI surface | FR-001–015 |
| `mcp-server` | AI MCP surface (parity + 5 tools) | FR-001–006 |
| `git-hooks` | Pre-commit / git-diff | FR-001–006 |
| `file-watch` | Continuous lint | FR-001–006 |
| `project-setup` | init / install / mcp-config | FR-001–007 |
| `maintenance` | doctor / security / deps | FR-001–007 |
| `tui` | Interactive terminal UI | FR-001–012 |

## Non-functional Requirements (High-level)

- Performance: Scan 1000 files in < 5 seconds
- Security: No network calls required for core functionality
- Scalability: Handle monorepos with 10,000+ files
- Platform: Linux (primary), macOS (secondary)
- Binary: Static release via `cargo build --release`
- Traceability: Acceptance tests named `acceptance_FR_00N.rs` per FR where practical

## Open Questions / Risks

- Windows support timeline
- Performance optimization for very large codebases
- Integration with CI/CD platforms (GitHub Actions, GitLab CI) — exit code 3 may require workflow updates
- MCP full parity implementation effort (stub removal)

## Glossary

- **AES**: Agentic Engineering System — architecture rules (24 product rules AES101–AES506)
- **DIAG_IO**: Non-AES internal diagnostic for unreadable files / I/O errors
- **MCP**: Model Context Protocol — JSON-RPC tool interface for AI agents
- **Parity**: Same business outcome for an action whether invoked via CLI or MCP
