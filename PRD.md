# PRD — Lint Arwaky

> Product Requirements. Describes WHAT this project does and WHY. Real condition lives in [ROADMAP.md](ROADMAP.md).

## Problem Statement

Software projects accumulate quality debt silently — for example, AES607 shipped in PR #335 with an undetected counting bug because no CI gate ran `docs .` (see issue #350), surfacing only in a later audit.

In this context, **quality debt** is specifically scoped to architectural drift across seven structural categories: naming conventions, layer dependency and import purity, code quality thresholds (file lengths, bypasses, dead inheritance), role boundaries, orphan/unreachable components, document invariants, and vertical slice folder structure. It explicitly excludes business-logic correctness, semantic runtime bugs, and dynamic test quality.

Developers, AI agents, and DevOps engineers on multi-language monorepos (Rust/Python/TypeScript with 10k+ files and teams of 5–50+ engineers) lack a single unified tool that audits all three languages together, enforces architectural rules in CI/CD, and enables autonomous remediation for both human developers and AI coding agents.

## Goals & Success Metrics

| # | Goal | Measurement | Target | Business Outcome | Baseline → Target |
|---|------|-------------|--------|------------------|-------------------|
| 1 | Multi-language linting in a single pass | `scan` on mixed-language workspace | violations from all 3 languages | Fewer cross-language review passes per PR | 3 passes/PR → ≤ 1 pass/PR (-66%) |
| 2 | 34 AES rules enforced across 7 groups | `workspaces-bad` scan | every applicable rule code produces violations per supported language | Prevent silent architecture drift before merge | 12 post-merge defect escapes/qtr → 0 escapes/qtr |
| 3 | MCP server with 5 tools, full CLI parity | `execute_command` on every CLI command | all commands reachable | AI agents autonomously detect and fix lint issues | Manual developer triage in CI: 45 min/run → < 5 min/run (-89%) |
| 4 | Self-auditing | `lint-arwaky-cli check .` on this repo | 0 violations | Zero internal architectural regression across releases | 0 violations maintained across all releases (measured per release) |

## User Personas

- **AI Agent**: Autonomous linting, self-healing via MCP tools.
- **Developer**: Lint codebases locally; enforce architecture during development.
- **DevOps / CI Engineer**: Quality gates, trend reports, dependency scans with stable exit codes.

## Scope

- **In scope**: CLI binary, MCP server, TUI, 34 AES rules across seven groups, external linter adapters, SARIF/JUnit/JSON reports, git hooks, auto-fix (remove + replace + rename).
- **Out of scope**: IDE plugins, web dashboard, cloud SaaS, non-Rust implementation, structural/multi-file semantic refactors in auto-fix, Windows runtime support (deferred to post-v3.x pending dedicated runner ROI; cross-compilation target available but runtime unvalidated; estimated cost of Windows CI matrix exceeds current adoption demand where >92% of target developer/CI environments are Linux/macOS).

## Product Decisions (locked)

| Topic | Decision |
|-------|----------|
| Auto-fix safety | Remove + replace + rename only; no structural or multi-file edits |
| Surface parity | MCP, CLI, and TUI expose the same commands |
| MCP tools | 5: `execute_command`, `list_commands`, `read_skill`, `health_check`, `get_config` |
| Acceptance tests | One acceptance test file per FR, named after the FR ID |
| Doctor exit code | 0 when diagnostic completes; 2 only on internal failure |
| Auto-fix outcomes | Reason-coded: `Applied` / `Skipped(reason)` / `Failed(reason)` |
| Concurrency | `std::thread` / `rayon`; async only in file-watch and mcp-server |
| Parsing | Full AST via tree-sitter; no regex fallback |
| Filesystem I/O | Centralized in filesystem crate |
| Integration resilience | External lint adapters (clippy, ruff, eslint, …): one attempt per scan, skip-and-log on timeout or spawn failure — a retried tool run can report against a different tool state and mask the real result. Git diff resolution: a multi-strategy fallback chain, because every strategy queries the same trusted local repository and a fallback still yields a correct, if broader, file list. Retry/fallback is added only where a fallback still produces a materially useful and safe result. |
| MCP tool-call duration | Unbounded: no server-side timeout and no cancellation method. The client owns the timeout and must set it above the documented worst-case chain |

## Exit Code Contract

| Code | Name | When |
|------|------|------|
| 0 | Ok | Success; clean scan; doctor finished; dry-run completed |
| 1 | Policy fail | Violations found; CI threshold failed; vulnerabilities found |
| 2 | Runtime error | Path missing; pipeline crash; invalid args; I/O failure |
| 3 | Prerequisite missing | Required external tool not installed |

MCP JSON responses MUST include a top-level integer `exit_code` aligned with
this contract, for every tool action and every outcome. An MCP client decides
success or failure from that field, never from JSON-RPC-level success: a run
that found violations is a successful tool call carrying `exit_code: 1`. The
response envelope is specified in the MCP surface's design document.

**Precedence when multiple conditions apply (highest wins):**
`Runtime error (2)` > `Prerequisite missing (3)` > `Policy fail (1)` > `Ok (0)`.

*Examples:*

- If runtime I/O fails while scanning files that also contain rule violations → Exit Code `2` (Runtime error outranks Policy fail).
- If an external tool is missing for a subset of files while other scanned files produce violations → Exit Code `3` (Prerequisite missing outranks Policy fail).
- If all checks pass cleanly with 0 violations and all prerequisites present → Exit Code `0` (Ok).

**`fix` aggregation rule:**

- If any fix attempt produces `Failed(reason)` (e.g. I/O failure, write error) → Exit Code `2` (Runtime error). This outranks the policy-fail result below.
- If all attempted items are `Skipped(reason)` (0 `Applied`, 0 `Failed`) → Exit Code `0` (Ok, skip reasons emitted as warnings).
- Otherwise, when every attempt is `Applied` or `Skipped` with zero `Failed`:
  - If re-scanning finds 0 violations remaining → Exit Code `0` (Ok), with skip reasons printed as warnings.
  - If violations remain that auto-fix cannot resolve (e.g. manual-only rules) → Exit Code `1` (Policy fail), consistent with `scan`.
- `fix --dry-run` and live `fix` share identical exit-code aggregation semantics. A dry-run never writes files, so it exits `0` unless an attempt itself produced `Failed(reason)`.

## AES Rule Summary (32 Rules)

Seven groups: **Naming** (AES101–103, 3), **Import** (AES201–205, 5), **Quality** (AES301–305, 5), **Role** (AES401–406, 6), **Orphan** (AES501–506, 6), **Doc** (AES601–605, 5), and **Structure** (AES701–704, 4). The `scan`/`check` command runs the six code linters plus structure; **Doc** rules are audited over the document chain via the `docs` command. Full rule definitions: [RULES_AES.md](RULES_AES.md).

## Feature Requirements (Prioritized)

> Real condition for each feature lives in [ROADMAP.md](ROADMAP.md). This section is specification only.

### P0 — Must Have

- **FR-PRD-001**: Multi-language scanning (Rust, Python, JS/TS). Acceptance: `scan` on mixed-language workspace returns violations from all three (evidence: the `filesystem` crate's FR-001 acceptance suite, `workspaces-bad/`).
- **FR-PRD-002**: 34 AES rules enforcement. Acceptance: `workspaces-bad` produces violations for every applicable code-level rule in each supported language; doc rules are audited over the document chain (evidence: `TEST.md` Section 3.2, `doc-rules` contract test suite).
- **FR-PRD-003**: CLI core commands (`check`, `scan`, `fix`, `ci`). Acceptance: each exits with the correct code from the Exit Code Contract (evidence: `cli-commands` and `dispatcher` acceptance suites).
- **FR-PRD-004**: MCP server with 5 tools, full execute parity. Acceptance: every CLI command is reachable via `execute_command` (evidence: `mcp-server` acceptance suite).
- **FR-PRD-005**: Self-auditing capability. Acceptance: `lint-arwaky-cli check .` on this repo reports 0 violations (evidence: `AGENTS.md` Definition of Done gate).

### P1 — Should Have

- **FR-PRD-006**: External linter adapters (Clippy, Ruff, ESLint, etc.). Acceptance: `adapters` lists all installed tools (evidence: `external-lint` adapter selection acceptance suite).
- **FR-PRD-007**: SARIF 2.1.0, JUnit XML, JSON reports. Acceptance: `--format sarif|junit|json` produces valid output (evidence: `report-formatter` FR-001 acceptance suite).
- **FR-PRD-008**: Git hooks integration. Acceptance: `install-hook` creates a working pre-commit hook (evidence: `git-hooks` acceptance suite).
- **FR-PRD-009**: Auto-fix capabilities (remove + replace + rename). Acceptance: `fix` applies mechanical fixes; `--dry-run` previews (evidence: `auto-fix` acceptance suites for AES101/AES201/AES304, and the fix-processor unit suite covering the FR-PRD-003 exit-code aggregation contract).
- **FR-PRD-010**: Watch mode for continuous linting. Acceptance: `watch` re-lints on file save (evidence: `file-watch` acceptance suite).
- **FR-PRD-011**: TUI file browser. Acceptance: TUI renders file tree, runs lint actions, exits cleanly (evidence: `tui` acceptance suite).
- **FR-PRD-012**: Workspace exit-code contract enforced everywhere. Acceptance: all commands return codes 0/1/2/3 per contract (evidence: `cli-commands` acceptance suite, `TEST.md` Section 3.4).
- **FR-PRD-013**: Acceptance tests standardized: one test file per FR, named after the FR ID (evidence: `crates/*/tests/` acceptance test inventory; note: migration to `acceptance_FR_NNN` naming is ongoing across crates—filesystem and report-formatter already compliant).

### P2 — Nice to Have

- **FR-PRD-014**: Windows support. Acceptance: build + test suite passes on a Windows CI runner (evidence: experimental cross-compile target in `DEPLOY.md`). Until then, Windows builds are experimental and are not a supported runtime target.
- **FR-PRD-015**: Deeper monorepo performance optimizations. Acceptance: 10k-file scan completes in < 10s on CI hardware (evidence: `crates/filesystem` acceptance suite FR-005).

### Requirement Hierarchy & Traceability

Traceability flows through four standardized layers:

1. **Product Level (PRD)**: `FR-PRD-NNN` defines high-level product capabilities.
2. **Roadmap Level (ROADMAP)**: `FR-<CRATE_CODE>` tracks crate-level delivery milestones (`FR-FILE`, `FR-AUTO`, etc.).
3. **Feature Level (Crate FRD)**: `FR-<FeatureName>-NNN` defines granular functional requirements within each crate (`FR-AutoFix-001`, `FR-Filesystem-001`).
4. **Verification Level (Acceptance Tests)**: `acceptance_FR_NNN` test suites verify `FR-<FeatureName>-NNN`, scoped by crate directory namespace. Legacy suites under their established names (domain-specific or rule-specific) remain valid evidence; new suites follow the `acceptance_FR_NNN` pattern.

## Open Questions / Risks

| # | Question / Risk | Owner | Consulted | Deadline | Status |
|---|-----------------|-------|-----------|----------|--------|
| 1 | Is 10k-file scan < 10s achievable on the CI runner tier? | @raka | DevOps / CI Engineer, Tech Lead | v3.7.0 | closed: verified < 5s for 1k files, < 10s for 10k files with rayon |
| 2 | Auto-fix: semantic rewrites allowed, or mechanical-only? | @raka | Developer, AI Agent personas | v3.7.0 | locked: mechanical-only (remove/replace/rename) to ensure safety |
| 3 | Windows: dedicated CI runner or cross-compile only? | @raka | DevOps / CI Engineer, Developer personas | post-v3.x | closed: cross-compile available, runtime CI deferred (>92% Linux/macOS userbase) |

## Non-functional Requirements (High-level)

| Category | Commitment | Detail lives in |
|----------|------------|-----------------|
| Performance (filesystem indexing) | 1,000 files < 2s; 10,000 files < 10s — the index build alone: discovery, read, parse | `filesystem/FRD.md` |
| Performance (full pipeline) | 1,000 files < 5s; 10,000 files < 15s — indexing plus every rule group, excluding external adapters | `filesystem/FRD.md`, `README.md` |
| Performance (regression detection) | The budget above is checked nightly by Criterion against a cached baseline; regressions fail the scheduled job | `.github/workflows/benchmarks.yml` |
| Worst-case external-lint chain | A single `scan`/`ci`/MCP `execute_command` call can block ~14 minutes: 10 adapters run sequentially, each with its own 60–180s ceiling and no overall budget. MCP clients must set a timeout above this bound | `external-lint/FRD.md`, `mcp-server` design |
| Security | No network calls for core; symlink safety enforced | `filesystem/FRD.md` |
| Scalability | 10,000+ file monorepos | `filesystem/FRD.md` |
| Platform | Linux primary, macOS secondary | `README.md` |
| Concurrency | std::thread / rayon; async only in file-watch, mcp-server | `dispatcher/FRD.md` |
| Parsing | Full AST via tree-sitter; no regex fallback | `filesystem/FRD.md` |
| Diagnostics | `PARSE_WARN` for files that fail to parse | `filesystem/FRD.md` |

## Feature Map

Crate responsibilities are listed in [README.md](README.md#project-structure) and each crate's `FRD.md` under `crates/`.

## Reference

- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md) · Testing: [TEST.md](TEST.md) · Contributing: [CONTRIBUTING.md](CONTRIBUTING.md) · Roadmap: [ROADMAP.md](ROADMAP.md)
