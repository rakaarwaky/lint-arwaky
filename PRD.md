# PRD — Lint Arwaky (v1.1.0)

---

## Problem Statement

Software projects accumulate quality debt silently. Developers lack a single tool that audits Rust + Python + JavaScript/TypeScript together, enforces architectural rules, and works for both human developers and AI agents

---

## Goals & Success Metrics

- **Goal 1**: Multi-language linting in a single pass (Rust, Python, JS/TS)
- **Goal 2**: 24 AES rules enforced across 5 groups (Naming, Import, Quality, Role, Orphan)
- **Goal 3**: MCP server with **5 tools** for autonomous AI-agent integration (full CLI parity on execute)
- **Goal 4**: Self-auditing — project lints itself under its own rule engine

---

## User Personas

- **AI Agent**: Autonomous linting, self-healing
- **Developer**: Lint codebases, enforce architecture during local development
- **DevOps / CI**: Quality gates, trend reports, dependency scans (stable exit codes)

---

## Scope

- **In scope**:

  - CLI binary (`lint-arwaky-cli`)
  - MCP server (`lint-arwaky-mcp`)
  - TUI file browser (`lint-arwaky-tui`) *
  - 24 AES rules across 5 groups
  - Non-AES diagnostics: `PARSE_WARN` (parse failure warning from filesystem crate)
  - External linter adapters (Clippy, Rustfmt, cargo-audit, Ruff, MyPy, Bandit, ESLint, Prettier, TSC)
  - SARIF 2.1.0, JUnit XML, JSON reports
  - Git hooks integration
  - Auto-fix: remove + replace + rename
- **Out of scope**:

  - IDE plugins (VS Code, IntelliJ)
  - Web dashboard
  - Cloud-hosted SaaS
  - Non-Rust implementation
  - Structural / multi-file semantic refactors in auto-fix

---

## Product Decisions (locked)


| Topic                 | Decision                                                                                                |
| ----------------------- | --------------------------------------------------------------------------------------------------------- |
| Auto-fix safety       | **Remove + replace + rename**                                                                          |
| MCP vs CLIv s TUI     | **Full parity**                                                                                        |
| MCP tools             | **5 tools**: `execute_command`, `list_commands`, `read_skill`, `health_check`, `get_config`             |
| Acceptance tests      | Filename standard:`acceptance_FR_00N.rs`                                                                |
| Doctor command        | Exit**0** when diagnostic completes (missing tools listed in body); exit **2** only on internal failure |
| Auto-fix outcomes     | **Reason-coded** results (`Applied` / `Skipped(reason)` / `Failed(reason)`), not bare bool              |
| Concurrency model     | **std::thread / rayon** — no async runtime (tokio removed) across all crates                           |
| AST parsing           | **Full AST via tree-sitter** — all languages, no regex fallback                                        |
| Filesystem operations | **Centralized in filesystem crate** —other crates do not perform file I/O or parsing                  |

---

## Exit Code Contract


| Code  | Name                 | When                                                                                     |
| ------- | ---------------------- | ------------------------------------------------------------------------------------------ |
| **0** | Ok                   | Success; clean scan; doctor finished (even if tools missing); dry-run completed          |
| **1** | Policy fail          | Violations found; CI threshold failed; vulnerabilities found; remaining issues after fix |
| **2** | Runtime error        | Path missing; pipeline crash; invalid args; I/O failure of the command itself            |
| **3** | Prerequisite missing | Required external tool not installed (e.g. cargo-audit for`security`)                    |

MCP JSON responses SHOULD include `exit_code` aligned with this contract.

---

## AES Rule Summary (24 Rules)


| Group       | Rules                                                                                                                                                                          | Count  | FRD             |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ----------------- |
| **Naming**  | AES101 (naming convention), AES102 (suffix/prefix validation)                                                                                                                  | 2      | naming-rules    |
| **Import**  | AES201 (layer dependency), AES202 (mandatory imports), AES203 (unused imports), AES204 (dummy imports), AES205 (circular dependencies)                                         | 5      | import-rules    |
| **Quality** | AES301 (max lines), AES302 (min lines), AES303 (mandatory definitions + dead inheritance), AES304 (bypass detection), AES305 (duplicate code)                                  | 5      | code-analysis   |
| **Role**    | AES401 (taxonomy purity), AES402 (contract primitives), AES403 (capability implementation), AES404 (utility purity), AES405 (agent composition), AES406 (surface passive role) | 6      | role-rules      |
| **Orphan**  | AES501 (taxonomy orphan), AES502 (contract orphan), AES503 (capabilities orphan), AES504 (utility orphan), AES505 (agent orphan), AES506 (surface orphan)                      | 6      | orphan-detector |
| **Total**   |                                                                                                                                                                                | **24** |                 |

---

## Feature Requirements (Prioritized)

### P0 — Must Have

- [ ]  Multi-language scanning (Rust, Python, JS/TS) — `filesystem`, `code-analysis`, `import-rules`, `naming-rules`, `role-rules`, `orphan-detector`, `external-lint`
- [ ]  24 AES rules enforcement — RULES_AES groups 1–5
- [ ]  CLI with `check`, `scan`, `fix`, `ci` commands — `cli-commands`
- [ ]  MCP server with 5 tools + **full execute parity** (no stubs) — `mcp-server`
- [ ]  Self-auditing capability — workspace scans itself

### P1 — Should Have

- [ ]  External linter adapters (Clippy, Rustfmt, cargo-audit, Ruff, MyPy, Bandit, ESLint, Prettier, TSC) — `external-lint`
- [ ]  SARIF 2.1.0, JUnit XML, JSON reports — `report-formatter`
- [ ]  Git hooks integration — `git-hooks`
- [ ]  Auto-fix capabilities (remove + replace + rename) — `auto-fix` 
- [ ]  Watch mode for continuous linting — `file-watch`
- [ ]  TUI file browser (critical-path acceptance) — `tui`
- [ ]  Workspace exit-code contract enforced everywhere — `cli-commands`, `maintenance`
- [ ]  Acceptance tests standardized to `acceptance_FR_00N.rs`

### P2 — Nice to Have

- [ ]  Windows support
- [ ]  Deeper monorepo performance optimizations

---

## Feature Map (crate → responsibility)


| Crate              | Primary value                                       |
| -------------------- | ----------------------------------------------------- |
| `shared`           | Taxonomy VOs, contracts, utilities                  |
| `config-system`    | Config load, merge, workspace detect                |
| `filesystem`       | File walking, AST parsing, graph construction       |
| `naming-rules`     | AES101–102 naming conventions                      |
| `import-rules`     | AES201–205 import boundaries (+ purpose sub-check) |
| `code-analysis`    | AES301–305 quality rules                           |
| `role-rules`       | AES401–406 layer roles                             |
| `orphan-detector`  | AES501–506 orphan detection                        |
| `external-lint`    | Clippy/Ruff/ESLint adapters (tool-native codes)     |
| `auto-fix`         | Mechanical fixes (remove + replace + rename)        |
| `report-formatter` | text/json/sarif/junit output                        |
| `cli-commands`     | Human CLI surface                                   |
| `mcp-server`       | AI MCP surface (parity + 5 tools)                   |
| `git-hooks`        | Pre-commit / git-diff                               |
| `file-watch`       | Continuous lint                                     |
| `project-setup`    | init / install / mcp-config                         |
| `maintenance`      | doctor / security / deps                            |
| `tui`              | Interactive terminal UI                             |

---

## Non-functional Requirements (High-level)

- **Performance**: Scan 1,000 files in < 5 seconds (full pipeline: walk + parse + all checks).
- **Security**: No network calls required for core functionality. Symlink safety enforced (skip targets outside workspace).
- **Scalability**: Handle monorepos with 10,000+ files.
- **Platform**: Linux (primary), macOS (secondary).
- **Binary**: Static release via `cargo build --release`.
- **Traceability**: Acceptance tests named `acceptance_FR_00N.rs` per FR where practical.
- **Concurrency**: std::thread / rayon across all crates. No async runtime dependency.
- **Parsing**: Full AST via tree-sitter for all languages. No regex or line-based parsing in final implementation.
- **Diagnostics**: `PARSE_WARN` (non-AES warning) for files that fail to parse. Emitted by filesystem crate.

## Reference

- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
