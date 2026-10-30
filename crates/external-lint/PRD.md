# PRD — Lint Arwaky

> Product Requirements. Describes WHAT this project does and WHY. Real condition lives in ROADMAP.md.

## Problem Statement

A multi-language monorepo has no single architectural audit. Engineers run separate linters per language and merge three reports by hand, so a violation in Rust never blocks a Python change and vice versa.

Developers, AI agents, and DevOps engineers on Rust/Python/TypeScript monorepos with 10k+ files and 5–50+ person teams lack one unified tool that audits all three languages together, enforces architectural rules in CI/CD, and remediates drift autonomously for humans and AI agents.

Without one gate, drift across naming, layering, imports, quality, roles, orphans, documents, and folder structure compounds silently until a full audit takes hours and releases carry hidden architectural defects.

## Project Requirements

### PR-LA-001: Multi-language scanning (Rust, Python, JS/TS)

#### Description

| No | Detail |
| --- | --- |
| 1 | `scan` on a mixed-language workspace returns violations from all three languages in a single pass. |
| 2 | One unified tool audits Rust/Python/TypeScript together and enforces architectural rules in CI/CD. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a developer, I want one scan to audit my Rust/Python/TypeScript monorepo. |
| 2 | I want one report instead of three. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Tree-sitter AST parsing across Rust, Python, and TypeScript. |
| 2 | 36 AES rules across 7 groups enforced in a single pass. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Windows runtime support (cross-compile target only; >92% Linux/macOS userbase). |
| 2 | Business-logic correctness, semantic runtime bugs, and dynamic test quality. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | 3 passes/PR → ≤ 1 pass/PR (-66%) across the three languages. |
| 2 | `scan` on `workspaces-bad/` produces violations for every applicable rule code. |
| ∞ | — |

### PR-LA-002: 34 AES rules enforcement across 7 groups

#### Description

| No | Detail |
| --- | --- |
| 1 | 34 AES rules across Naming, Import, Quality, Role, Orphan, Doc, Structure groups are enforced per scan. |
| 2 | Silent drift never reaches main because every architectural rule is enforced in CI. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a DevOps engineer, I want every architectural rule enforced in CI. |
| 2 | I want silent drift to never reach main. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | 34 rules across 7 groups in Rust/Python/TypeScript. |
| 2 | Enforcement per scan across all three languages. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Business-logic correctness; semantic runtime bugs; dynamic test quality. |
| 2 | These are explicitly excluded from rule enforcement. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | 12 post-merge defect escapes/quarter → 0 escapes/quarter. |
| 2 | `workspaces-bad` produces violations for every applicable rule code in each language. |
| ∞ | — |

### PR-LA-003: CLI core commands (`check`, `scan`, `fix`, `ci`)

#### Description

| No | Detail |
| --- | --- |
| 1 | `check`, `scan`, `fix`, and `ci` are the four core CLI commands, each with a stable exit code. |
| 2 | Plus `report`, `git`, `watch`, `doctor`, `install-hook` complete the command surface. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a developer, I want `scan .` to report violations and `ci . --threshold N` to gate a release. |
| 2 | I want stable exit codes so I can branch on them in my pipeline. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | `scan`, `check`, `fix`, `ci` as the four core CLI commands. |
| 2 | `report`, `git`, `watch`, `doctor`, `install-hook` as additional commands. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | IDE plugins; web dashboard; cloud SaaS. |
| 2 | These surfaces are not part of the CLI core. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | Manual developer triage in CI: 45 min/run → < 5 min/run (-89%). |
| 2 | Each command exits with the correct code from the exit-code contract (0/1/2/3). |
| ∞ | — |

### PR-LA-004: MCP server with 5 tools, full CLI parity

#### Description

| No | Detail |
| --- | --- |
| 1 | An MCP server exposes every CLI command via 5 tools so AI agents can self-lint. |
| 2 | `execute_command`, `list_commands`, `read_skill`, `health_check`, `get_config` are the five tools. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As an AI agent, I want `execute_command` to reach every CLI command. |
| 2 | I want to detect and fix lint issues autonomously. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | 5 MCP tools: `execute_command`, `list_commands`, `read_skill`, `health_check`, `get_config`. |
| 2 | Full CLI parity so every CLI command is reachable via `execute_command`. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Server-side timeouts or cancellation (the client owns the timeout). |
| 2 | These are the client's responsibility, not the MCP server's. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | 45 min/run manual triage → < 5 min/run autonomous agent loop. |
| 2 | Every CLI command is reachable via `execute_command`. |
| ∞ | — |

### PR-LA-005: Self-auditing capability

#### Description

| No | Detail |
| --- | --- |
| 1 | `lint-arwaky-cli check .` on this repo reports 0 violations — the linter audits itself. |
| 2 | `docs .` reports 0 invariant findings; self-lint gate catches regressions silently. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a maintainer, I want the project to self-lint so the rule engine never regresses silently. |
| 2 | I want `check .` on this repo to return 0 violations. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Self-lint gate; doc audit (`docs .`). |
| 2 | `workspaces-bad`/`workspaces-good` fixtures for validation. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Structural or multi-file auto-fix edits (only remove/replace/rename are safe). |
| 2 | These edits are excluded from auto-fix scope. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | `check .` on this repo → 0 violations; `docs .` → 0 invariant findings. |
| 2 | 0 internal architectural regressions across all releases. |
| ∞ | — |

### PR-LA-006: External linter adapters (Clippy, Ruff, ESLint)

#### Description

| No | Detail |
| --- | --- |
| 1 | Optional adapters invoke Clippy/Ruff/ESLint/tool-native codes when installed. |
| 2 | External tool findings are surfaced in one unified report alongside native rules. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a developer, I want the linter to surface external tool findings in one report. |
| 2 | I want Clippy, Ruff, and ESLint results included without running each tool separately. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | 10 external adapters with a per-adapter 60–180 s ceiling and skip-and-log on timeout. |
| 2 | Missing tools are skipped and logged. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | An overall budget across the adapter phase (MCP clients set their own timeout). |
| 2 | Timeout budgeting is the client's responsibility. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | 10k-file scan in < 10s on CI hardware with adapters enabled. |
| 2 | `adapters` lists all installed tools; missing tools are skipped and logged. |
| ∞ | — |

### PR-LA-007: SARIF 2.1.0, JUnit XML, JSON reports

#### Description

| No | Detail |
| --- | --- |
| 1 | `--format sarif/junit/json` produces three report formats alongside the default text. |
| 2 | SARIF 2.1.0, JUnit XML, and JSON report writers emit valid output for each format. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a CI engineer, I want SARIF/JUnit/JSON so I can pipe results into external dashboards. |
| 2 | I want valid, machine-readable reports from every format. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | SARIF 2.1.0 report writer. |
| 2 | JUnit XML and JSON report writers. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | SARIF import (write-only). |
| 2 | Reading SARIF back into the tool is not supported. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | `scan . --format sarif` emits valid SARIF; `--format junit` emits valid JUnit XML. |
| 2 | 0 malformed reports across the three formats in CI. |
| ∞ | — |

### PR-LA-008: Git hooks integration

#### Description

| No | Detail |
| --- | --- |
| 1 | `install-hook` creates a working pre-commit hook that runs the linter on staged files. |
| 2 | Violations are caught before `git commit`. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a developer, I want a pre-commit hook so violations are caught before `git commit`. |
| 2 | I want `install-hook` to create a working hook that fires on staged files. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Pre-commit hook installation/uninstallation. |
| 2 | Git-diff base scanning; linting only staged files. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Post-commit / pre-push hooks. |
| 2 | These hook types are explicitly out of scope. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | `install-hook` creates a working pre-commit hook that fires on staged files. |
| 2 | 0 pre-commit hook false-negatives across the test suite. |
| ∞ | — |

### PR-LA-009: Auto-fix capabilities (remove + replace + rename)

#### Description

| No | Detail |
| --- | --- |
| 1 | `fix` applies mechanical fixes (remove/replace/rename); `--dry-run` previews without writing files. |
| 2 | Fixes are safe mechanical transformations with no behavioral changes. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a developer, I want safe mechanical fixes so I can apply them without hand-editing. |
| 2 | I want `--dry-run` to preview changes before applying them. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Remove/replace/rename mechanical fixes. |
| 2 | `Applied`/`Skipped(reason)`/`Failed(reason)` outcomes for every fix attempt. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Structural or multi-file semantic rewrites. |
| 2 | These are not mechanical fixes and are excluded. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | `fix` applies fixes; `--dry-run` previews without writing files. |
| 2 | 100% of mechanical fixes are safe (0 behavioral changes). |
| ∞ | — |

### PR-LA-010: Watch mode for continuous linting

#### Description

| No | Detail |
| --- | --- |
| 1 | `watch` re-lints on file save in the foreground or background. |
| 2 | Continuous re-lint gives immediate feedback as files change. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a developer, I want continuous re-lint so I see violations as I type. |
| 2 | I want the linter to re-run automatically when I save a file. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | File-system watch via tokio; incremental re-scan. |
| 2 | Re-lint on file save in foreground or background. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | IDE integration (headless terminal-only). |
| 2 | Editor/IDE plugins are excluded. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | `watch` re-lints on file save; exits cleanly on `Ctrl-C`. |
| 2 | 0 missed re-lints across the file-watch acceptance suite. |
| ∞ | — |

### PR-LA-011: TUI file browser

#### Description

| No | Detail |
| --- | --- |
| 1 | `lint-arwaky-tui` renders a file tree and runs lint actions in an interactive TUI. |
| 2 | Navigate files and trigger lints without leaving the terminal. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a developer, I want a TUI so I can navigate files and trigger lints without leaving the terminal. |
| 2 | I want interactive lint actions from a file tree view. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | File tree; lint actions; exit. |
| 2 | Interactive TUI for file navigation and lint triggering. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | GUI dashboard (out of scope by Decision 2). |
| 2 | A GUI dashboard is explicitly not part of the TUI. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | TUI renders the file tree, runs lint actions, and exits cleanly. |
| 2 | 0 TUI crashes in the acceptance suite. |
| ∞ | — |

### PR-LA-012: Workspace exit-code contract enforced everywhere

#### Description

| No | Detail |
| --- | --- |
| 1 | Every command returns codes 0/1/2/3 per the contract. |
| 2 | `check`, `scan`, `fix`, `ci`, `doctor`, and the MCP `execute_command` envelope carry stable exit codes. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a CI engineer, I want stable exit codes so I can branch on them in my pipeline. |
| 2 | I want to branch on status without parsing output. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | `check`, `scan`, `fix`, `ci`, `doctor`, and the MCP `execute_command` envelope. |
| 2 | Stable exit codes 0/1/2/3 per the workspace exit-code contract. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | MCP JSON-RPC-level success (the envelope carries `exit_code`, not JSON-RPC status). |
| 2 | JSON-RPC status is not the contract; `exit_code` is. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | All commands return 0/1/2/3 per contract; MCP `execute_command` carries `exit_code`. |
| 2 | 0 exit-code contract violations in the acceptance suites. |
| ∞ | — |

### PR-LA-013: Acceptance tests standardized (one test file per FR)

#### Description

| No | Detail |
| --- | --- |
| 1 | Each FR carries one acceptance test file under `crates/*/tests/` named after the FR. |
| 2 | `acceptance_FR_NNN` naming convention; legacy domain-specific suites remain valid. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a maintainer, I want one acceptance file per FR so coverage traces back to the spec. |
| 2 | I want test inventory to map to FR IDs in `crates/*/FRD.md`. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | `acceptance_FR_NNN` naming for acceptance test files. |
| 2 | Legacy domain-specific suites remain valid; `crates/*/tests/` acceptance test inventory maps to FR IDs. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Renaming existing legacy acceptance files (dedicated refactor owns that). |
| 2 | Legacy renames are owned by a separate refactor effort. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | `crates/*/tests/` acceptance test inventory maps to FR IDs in `crates/*/FRD.md`. |
| 2 | 100% of new FRs ship with an `acceptance_FR_NNN` file. |
| ∞ | — |

### PR-LA-014: Windows support

#### Description

| No | Detail |
| --- | --- |
| 1 | Cross-compile target available; Windows runtime deferred. |
| 2 | Prebuilt binary available so Windows developers can self-lint on Windows. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a Windows developer, I want a prebuilt binary so I can self-lint on Windows. |
| 2 | I want to run lint-arwaky on Windows without building from source. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Cross-compile target; experimental. |
| 2 | Build + test suite passes on a Windows CI runner (experimental cross-compile target). |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Dedicated Windows CI runner; >92% Linux/macOS userbase justifies deferring runtime CI. |
| 2 | Windows runtime CI is deferred due to userbase distribution. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | Build + test suite passes on a Windows CI runner (experimental cross-compile target). |
| 2 | 0 Windows runtime regression reports across releases. |
| ∞ | — |

### PR-LA-015: Deeper monorepo performance optimizations

#### Description

| No | Detail |
| --- | --- |
| 1 | 10k-file scan in < 10s on CI hardware with rayon parallelism. |
| 2 | Per-language parser cache and skip-pragmas keep scan time bounded. |
| ∞ | — |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a developer on a 10k-file monorepo, I want the scan to stay under 10s so I can run it per PR. |
| 2 | I want fast feedback on every pull request. |
| ∞ | — |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Rayon indexing; per-language parser cache; skip-pragmas. |
| 2 | Scan completes in < 10s on CI hardware for 10k files. |
| ∞ | — |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | GPU acceleration; distributed scan. |
| 2 | These are not part of the performance optimization scope. |
| ∞ | — |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | 10k-file scan completes in < 10s on CI hardware. |
| 2 | 10k-file scan in < 10s in the Criterion suite. |
| ∞ | — |

## Reference

| Artifact     | Purpose                                          | Link |
|--------------|--------------------------------------------------|------|
| Roadmap      | Timeline, phasing, and dependencies               | [ROADMAP.md](ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra         | [ARCHITECTURE.md](ARCHITECTURE.md) |
