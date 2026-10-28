# PRD — Lint Arwaky

> Product Requirements. Describes WHAT this project does and WHY. Real condition lives in [ROADMAP.md](ROADMAP.md).

## Problem Statement

Software projects accumulate quality debt silently — for example, AES607 shipped in PR #335 with an undetected counting bug because no CI gate ran `docs .` (see issue #350), surfacing only in a later audit.

In this context, **quality debt** is specifically scoped to architectural drift across seven structural categories: naming conventions, layer dependency and import purity, code quality thresholds (file lengths, bypasses, dead inheritance), role boundaries, orphan/unreachable components, document invariants, and vertical slice folder structure. It explicitly excludes business-logic correctness, semantic runtime bugs, and dynamic test quality.

Developers, AI agents, and DevOps engineers on multi-language monorepos (Rust/Python/TypeScript with 10k+ files and teams of 5–50+ engineers) lack a single unified tool that audits all three languages together, enforces architectural rules in CI/CD, and enables autonomous remediation for both human developers and AI coding agents.

## Project Requirements

### PR-LA-001: Multi-language scanning (Rust, Python, JS/TS)

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | `scan` on a mixed-language workspace returns violations from all three languages in a single pass.|
| User Story   | As a developer, I want one scan to audit my Rust/Python/TypeScript monorepo so I get one report instead of three.|
| In Scope     | Tree-sitter AST parsing; 36 AES rules across 7 groups; Rust/Python/TypeScript.         |
| Out of Scope | Windows runtime support (cross-compile target only; >92% Linux/macOS userbase).         |
| Acceptance   | `scan` on `workspaces-bad/` produces violations for every applicable rule code.       |
| Success Metric | 3 passes/PR → ≤ 1 pass/PR (-66%) across the three languages.                        |

### PR-LA-002: 34 AES rules enforcement across 7 groups

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | 34 AES rules across Naming, Import, Quality, Role, Orphan, Doc, Structure groups are enforced per scan.|
| User Story   | As a DevOps engineer, I want every architectural rule enforced in CI so silent drift never reaches main.|
| In Scope     | 34 rules across 7 groups in Rust/Python/TypeScript.                                    |
| Out of Scope | Business-logic correctness; semantic runtime bugs; dynamic test quality.               |
| Acceptance   | `workspaces-bad` produces violations for every applicable rule code in each language.  |
| Success Metric | 12 post-merge defect escapes/quarter → 0 escapes/quarter.                            |

### PR-LA-003: CLI core commands (`check`, `scan`, `fix`, `ci`)

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | `check`, `scan`, `fix`, and `ci` are the four core CLI commands, each with a stable exit code.|
| User Story   | As a developer, I want `scan .` to report violations and `ci . --threshold N` to gate a release.|
| In Scope     | `scan`, `check`, `fix`, `ci`, plus `report`, `git`, `watch`, `doctor`, `install-hook`.|
| Out of Scope | IDE plugins, web dashboard, cloud SaaS.                                                 |
| Acceptance   | Each command exits with the correct code from the exit-code contract (0/1/2/3).         |
| Success Metric | Manual developer triage in CI: 45 min/run → < 5 min/run (-89%).                    |

### PR-LA-004: MCP server with 5 tools, full CLI parity

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | An MCP server exposes every CLI command via 5 tools so AI agents can self-lint.        |
| User Story   | As an AI agent, I want `execute_command` to reach every CLI command so I can detect and fix lint issues autonomously.|
| In Scope     | 5 MCP tools: `execute_command`, `list_commands`, `read_skill`, `health_check`, `get_config`.|
| Out of Scope | Server-side timeouts or cancellation (the client owns the timeout).                    |
| Acceptance   | Every CLI command is reachable via `execute_command`.                                   |
| Success Metric | 45 min/run manual triage → < 5 min/run autonomous agent loop.                         |

### PR-LA-005: Self-auditing capability

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | `lint-arwaky-cli check .` on this repo reports 0 violations — the linter audits itself.|
| User Story   | As a maintainer, I want the project to self-lint so the rule engine never regresses silently.|
| In Scope     | Self-lint gate, doc audit (`docs .`), and `workspaces-bad`/`workspaces-good` fixtures.|
| Out of Scope | Structural or multi-file auto-fix edits (only remove/replace/rename are safe).         |
| Acceptance   | `check .` on this repo → 0 violations; `docs .` → 0 invariant findings.               |
| Success Metric | 0 internal architectural regressions across all releases.                            |

### PR-LA-006: External linter adapters (Clippy, Ruff, ESLint)

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | Optional adapters invoke Clippy/Ruff/ESLint/tool-native codes when installed.          |
| User Story   | As a developer, I want the linter to surface external tool findings in one report.     |
| In Scope     | 10 external adapters with a per-adapter 60–180 s ceiling and skip-and-log on timeout.   |
| Out of Scope | An overall budget across the adapter phase (MCP clients set their own timeout).        |
| Acceptance   | `adapters` lists all installed tools; missing tools are skipped and logged.            |
| Success Metric | 10k-file scan in < 10s on CI hardware with adapters enabled.                        |

### PR-LA-007: SARIF 2.1.0, JUnit XML, JSON reports

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | `--format sarif, junit, json` produces three report formats alongside the default text.  |
| User Story   | As a CI engineer, I want SARIF/JUnit/JSON so I can pipe results into external dashboards.|
| In Scope     | SARIF 2.1.0, JUnit XML, and JSON report writers.                                       |
| Out of Scope | SARIF import (write-only).                                                             |
| Acceptance   | `scan . --format sarif` emits valid SARIF; `--format junit` emits valid JUnit XML.     |
| Success Metric | 0 malformed reports across the three formats in CI.                                  |

### PR-LA-008: Git hooks integration

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | `install-hook` creates a working pre-commit hook that runs the linter on staged files. |
| User Story   | As a developer, I want a pre-commit hook so violations are caught before `git commit`.|
| In Scope     | Pre-commit hook installation/uninstallation; git-diff base scanning.                   |
| Out of Scope | Post-commit / pre-push hooks.                                                          |
| Acceptance   | `install-hook` creates a working pre-commit hook that fires on staged files.           |
| Success Metric | 0 pre-commit hook false-negatives across the test suite.                           |

### PR-LA-009: Auto-fix capabilities (remove + replace + rename)

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | `fix` applies mechanical fixes (remove/replace/rename); `--dry-run` previews.          |
| User Story   | As a developer, I want safe mechanical fixes so I can apply them without hand-editing. |
| In Scope     | Remove/replace/rename; `Applied`/`Skipped(reason)`/`Failed(reason)` outcomes.          |
| Out of Scope | Structural or multi-file semantic rewrites.                                             |
| Acceptance   | `fix` applies fixes; `--dry-run` previews without writing files.                       |
| Success Metric | 100% of mechanical fixes are safe (0 behavioral changes).                            |

### PR-LA-010: Watch mode for continuous linting

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | `watch` re-lints on file save in the foreground or background.                         |
| User Story   | As a developer, I want continuous re-lint so I see violations as I type.              |
| In Scope     | File-system watch via tokio; incremental re-scan.                                       |
| Out of Scope | IDE integration (headless terminal-only).                                               |
| Acceptance   | `watch` re-lints on file save; exits cleanly on `Ctrl-C`.                              |
| Success Metric | 0 missed re-lints across the file-watch acceptance suite.                            |

### PR-LA-011: TUI file browser

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | `lint-arwaky-tui` renders a file tree and runs lint actions in an interactive TUI.     |
| User Story   | As a developer, I want a TUI so I can navigate files and trigger lints without leaving the terminal.|
| In Scope     | File tree, lint actions, and exit.                                                      |
| Out of Scope | GUI dashboard (out of scope by Decision 2).                                             |
| Acceptance   | TUI renders the file tree, runs lint actions, and exits cleanly.                        |
| Success Metric | 0 TUI crashes in the acceptance suite.                                               |

### PR-LA-012: Workspace exit-code contract enforced everywhere

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | Every command returns codes 0/1/2/3 per the contract.                                   |
| User Story   | As a CI engineer, I want stable exit codes so I can branch on them in my pipeline.     |
| In Scope     | `check`, `scan`, `fix`, `ci`, `doctor`, and the MCP `execute_command` envelope.        |
| Out of Scope | MCP JSON-RPC-level success (the envelope carries `exit_code`, not JSON-RPC status).    |
| Acceptance   | All commands return 0/1/2/3 per contract; MCP `execute_command` carries `exit_code`.   |
| Success Metric | 0 exit-code contract violations in the acceptance suites.                          |

### PR-LA-013: Acceptance tests standardized (one test file per FR)

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | Each FR carries one acceptance test file under `crates/*/tests/` named after the FR.   |
| User Story   | As a maintainer, I want one acceptance file per FR so coverage traces back to the spec.|
| In Scope     | `acceptance_FR_NNN` naming; legacy domain-specific suites remain valid.                 |
| Out of Scope | Renaming existing legacy acceptance files (dedicated refactor owns that).              |
| Acceptance   | `crates/*/tests/` acceptance test inventory maps to FR IDs in `crates/*/FRD.md`.       |
| Success Metric | 100% of new FRs ship with an `acceptance_FR_NNN` file.                           |

### PR-LA-014: Windows support

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | Cross-compile target available; Windows runtime deferred.                              |
| User Story   | As a Windows developer, I want a prebuilt binary so I can self-lint on Windows.        |
| In Scope     | Cross-compile target; experimental.                                                     |
| Out of Scope | Dedicated Windows CI runner; >92% Linux/macOS userbase justifies deferring runtime CI. |
| Acceptance   | Build + test suite passes on a Windows CI runner (experimental cross-compile target).  |
| Success Metric | 0 Windows runtime regression reports across releases.                              |

### PR-LA-015: Deeper monorepo performance optimizations

| Aspect       | Detail                                                                                  |
|--------------|------------------------------------------------------------------------------------------|
| Description  | 10k-file scan in < 10s on CI hardware with rayon parallelism.                          |
| User Story   | As a developer on a 10k-file monorepo, I want the scan to stay under 10s so I can run it per PR.|
| In Scope     | Rayon indexing, per-language parser cache, skip-pragmas.                                |
| Out of Scope | GPU acceleration; distributed scan.                                                    |
| Acceptance   | 10k-file scan completes in < 10s on CI hardware.                                        |
| Success Metric | 10k-file scan in < 10s in the Criterion suite.                                      |

## Reference

| Artifact     | Purpose                                          | Link|
|--------------|--------------------------------------------------|------|
| Roadmap      | Timeline, phasing, and dependencies               | [ROADMAP.md](ROADMAP.md)|
| Architecture | System boundaries, tech stack, and infra         | [ARCHITECTURE.md](ARCHITECTURE.md)|
