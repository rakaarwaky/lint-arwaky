# PRD — Lint Arwaky

> Product Requirements. Describes WHAT this project does and WHY. Real condition lives in ROADMAP.md.

## Project Overview

A multi-language monorepo has no single architectural audit. Engineers run separate linters per language and merge three reports by hand, so a violation in Rust never blocks a Python change and vice versa.

Developers, AI agents, and DevOps engineers on Rust/Python/TypeScript monorepos with 10k+ files and 5–50+ person teams lack one unified tool that audits all three languages together, enforces architectural rules in CI/CD, and remediates drift autonomously for humans and AI agents.

Without one gate, drift across naming, layering, imports, quality, roles, orphans, documents, and folder structure compounds silently until a full audit takes hours and releases carry hidden architectural defects.

## Goals

| # | Goal (capability) | Business outcome | Baseline → target | Measurement method | Owner · cadence |
|---|-------------------|------------------|-------------------|--------------------|-----------------|
| 1 | One scan reports all 41 AES codes across Rust, Python, and TS | Cross-language review passes per PR stop being a manual merge step | not yet measured → one gate per repo | Violations caught per scan on the reference monorepo; review passes avoided per PR | Business analyst · per release |
| 2 | `lint-arwaky-cli scan .` returns p95 under 60 s on a 10k-file tree, 4-core CI runner | The audit fits inside a single CI step instead of a scheduled job | not yet measured → p95 < 60 s | Scripted scan of a generated 10k-file fixture; elapsed-time distribution recorded in the release evidence | Platform engineer · per release |
| 3 | Every finding is machine-actionable; `fix .` remediates every safe, deterministic correction | Maintainers clear violations without hand-editing each file | not yet measured → 0 hand edits for safe fixes | Applied/skipped/failed counts from the fix report on the reference monorepo | Maintainer · per release |
| 4 | `ci . --threshold N` fails the build the moment an enabled rule group finds a violation | Cross-language drift blocks a release instead of surfacing after it | not yet measured → 0 drift escapes | Exit-code and finding counts from the CI job that gates the release | CI engineer · per release |

Metric baselines are recorded here rather than restated elsewhere; a release that cannot produce the measurement states "not measured" instead of claiming the target.

## Non-Goals

- Redesigning individual language linters (Clippy, Ruff, ESLint) — their native rules stay owned by each tool.
- A merchant-side checkout customization tool or any business-domain feature unrelated to architectural audit.
- Persistent scan history or network services beyond local workspace inspection.
- Internationalized UI or multi-language natural-language output beyond English diagnostics.

## Target Users

| Persona | Need | Current Pain | Desired Outcome |
|---------|------|--------------|-----------------|
| Rule developer | One place to get parsed files, dependency graphs, file contents, tool availability, and language flags | Reaching into the filesystem directly from every rule | Implement rules without writing any filesystem code |
| CI engineer | `scan .` reporting violations and `ci . --threshold N` gating a release | Merging three per-language reports by hand | Branch on exit code instead of parsing text output |
| Platform engineer | Every import validated against the layer-dependency matrix | Forbidden cross-layer imports surfacing late | Forbidden edges, missing mandatory imports, unused imports, dummy stubs, and cycles surface as lint failures |
| Maintainer | Dead, unused, or unreachable code flagged per layer | Dead code compounding across layers until audit takes hours | Each orphan yields a rule-coded finding naming the file and reachability gap |

## Scope

### In Scope

- Filesystem-derived data as the single source: parse source files into structured entries and imports, build a project-wide dependency graph, perform read/write/discovery/copy/remove and permission operations, probe for external tool binaries, and detect workspace root plus language presence.
- Safe, deterministic auto-fix corrections (unused-import removal, bypass-marker stripping, mechanical symbol renaming) applied only when requested, with a preview mode reporting would-be changes without touching disk.
- Terminal surface: one surface command module per subcommand (scan, check, fix, ci, report, git, watch, setup, plugin, config, maintenance, docs) parsing arguments, invoking dispatch, and formatting results.
- Single source of truth for configuration: discovery across a bounded priority chain, workspace-type detection, merging user settings onto embedded defaults, threshold and adapter validation, ignored-path assembly.
- Dispatcher action entry points for each workspace verb (check, scan, ci, fix, config, git, setup, watch, docs, import, external, maintenance, naming, orphan, plugin, quality, role, structure, layer scan, version, suppression) merging findings from every enabled rule group into one report.
- Document-invariant audit across AES601–AES610 (all ten recognized documents) with machine-readable violation types; the ARCHITECTURE.md Rule Index is the single source for rule counts (41 rules).
- External-lint aggregation of Clippy, Rustfmt, cargo-audit, Ruff, Mypy, Bandit, ESLint, Prettier, TSC, and markdownlint-cli normalized into unified violations.
- Real-time file watching with debouncing, filtering, deduplication, and an initial full-project scan.
- Pre-commit hook installing/linting only changed files against the base branch and blocking commits on AES violations.
- Import-rule enforcement across the 7-layer AES architecture (AES201–AES205) governed by YAML configuration.
- Centralized real-time diagnostic feed behind `-v`/`--verbose` with per-phase timings on stderr, stdout kept clean.
- Maintenance capability: environment/toolchain health, external-linter adapter availability, cache reclamation, version probing, dependency/security summary, and binary self-update.
- MCP server exposing listable, domain-filterable command catalog, execute and config tools, health check, and docs scaffold, confined to the workspace.
- AES naming convention enforcement (AES101–AES103) across file names with stem shape, per-layer suffix/prefix policy, and test/bench prefix placement.
- Orphan detection (AES501–AES506) tracing reachability from container/main/binary entry points across import and DI relationships.

### Out of Scope

- Rule evaluation and violation reporting driven by the filesystem source itself; auto-fix or file mutation driven by lint findings.
- Multi-file renames, structural refactors, adding new imports or types, semantic rewrites such as panic removal, formatting-only passes.
- Business logic and orchestration living outside the dispatcher and capability layers; IDE plugins, web dashboard, and cloud SaaS surfaces.
- Secret or credential scanning; semantic analysis beyond AST extraction; network or remote filesystem access.
- Language detection and tree walking owned outside the filesystem source; concurrent or parallel adapter execution.
- IDE plugins or editor integrations; hot-reloading configuration during a running watch session; remote or cloud-based watching.
- Post-commit and pre-push hooks; whole-repository scans in the commit gate; auto-fixing of violations the hook blocks on.
- Compilation, type checking, or runtime behaviour analysis; dependency installation or package management.
- Persistent log files, non-stderr output channels, asynchronous or distributed scan execution, persistent scan history, network services.
- Server-side timeouts or cancellation (client owns the timeout); watch command over MCP; MCP JSON-RPC-level success status.
- Renaming legacy files that predate the convention; automatic renaming of files (reporting only).
- Certifying behavioural deadness that only tests can prove; auto-removal or rewriting of flagged files; cross-repository or distributed scanning.

---

## Project Requirements

### PR-LA-001: auto-fix

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Rust maintainer resolving flagged unused imports | the flagged line removed automatically in place or previewed first | I can clean violations without hand-editing every flagged file | P2 |
| Maintainer | a reason-coded result stating whether a fix applied, was skipped, or failed and why | I know exactly what changed and what needs manual work | P2 |

#### Proposed Solution

Applies safe, deterministic corrections to source files that violate architectural rules, writing the corrected files back to disk only when requested. It removes unused imports, strips known bypass markers, and mechanically renames flagged symbols — each a local, single-file change that never restructures surrounding code. A preview mode reports the exact changes that would be made without touching any file on disk; every attempt yields a reason-coded result stating whether a fix applied, was skipped, or failed and why.

#### Main User Flow

1. The maintainer runs auto-fix in preview mode over a flagged violation report.
2. The system reports every would-be change with a reason-coded outcome (applied, skipped with reason, or failed with reason) while changing zero bytes on disk.
3. The maintainer applies the fixes, rewriting every fixable violation exactly once.

#### Acceptance Criteria

- `AC-PR-LA-001-1`: Preview reports every would-be change with a reason code and leaves the file byte-identical; exit 0.
- `AC-PR-LA-001-2`: An applied fix rewrites each fixable violation exactly once; a second run reports no change; exit 0.
- `AC-PR-LA-001-3`: An unsafe fix is skipped with a reason and the file untouched; the run exits 1 because findings remain.

---

### PR-LA-002: cli-commands

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| CI engineer gating releases on a mixed-language monorepo | `scan .` reporting violations and `ci . --threshold N` gating a release | my pipeline can branch on the exit code instead of parsing text output | P2 |
| Operator | every command exiting with the code the workspace exit-code contract assigns to its outcome | I can script reliable release gates | P2 |

#### Proposed Solution

The terminal surface of the tool: one module per subcommand (scan, check, fix, ci, report, git, watch, setup, plugin, config, maintenance, docs) parses arguments, delegates to the dispatcher, and formats the returned result. Report-oriented subcommands render snapshots and per-member deltas through shared output helpers. Every command exits with the code the workspace exit-code contract assigns to its outcome.

#### Main User Flow

1. The operator invokes a subcommand with arguments.
2. The surface module parses arguments, delegates workspace work through the dispatcher actions, and formats the result.
3. The command exits with the contract exit code; the caller branches on status without parsing output.

#### Acceptance Criteria

- `AC-PR-LA-002-1`: Every documented subcommand exits with the code assigned in the Workspace Exit Code Contract below.
- `AC-PR-LA-002-2`: `scan .` exits 1 on the bad workspace with every offending file and rule listed, and 0 on the good workspace.
- `AC-PR-LA-002-3`: An unknown subcommand or invalid argument exits 2 with a usage message on stderr and nothing on stdout.

---

### PR-LA-003: config-system

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| DevOps engineer owning the single lint_arwaky.config.yaml for a multi-language monorepo team | config discovery, merging, validation, and ignored-path resolution to happen once and be reused by every command | a single config edit changes lint behaviour across the whole tool without touching command code | P0 |
| Operator | a project with no config file scoring through valid embedded defaults | I can adopt the tool without authoring configuration | P0 |

#### Proposed Solution

The config system is the single source of truth for lint-arwaky configuration: it discovers a project's config file across a bounded priority chain, detects the workspace type and its member directories, merges user settings onto embedded defaults, validates thresholds and adapter enablement, and assembles the ignored-path set that every command honours. Every read returns the same validated, merged configuration regardless of how many times the file is opened in one run.

#### Main User Flow

1. A command starts and the config system discovers the config file across the five-level priority chain.
2. It merges user settings onto embedded defaults, validates thresholds and adapter enablement, and assembles the ignored-path set.
3. Every command honours the cached, validated configuration on that run.

#### Acceptance Criteria

- `AC-PR-LA-003-1`: With no config file present, embedded defaults are used: `scan .` exits 0 on a clean workspace and 1 on a violating one.
- `AC-PR-LA-003-2`: One config edit (a rule set to `enabled: false`) changes behaviour for every subcommand in that run, with no rebuild.
- `AC-PR-LA-003-3`: An invalid threshold or adapter value exits 2 and names the offending config key.

---

### PR-LA-004: dispatcher

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Integrator adding a new Lint Arwaky surface | one shared action entry point per workspace verb | CLI, MCP, and TUI behave identically without duplicating business logic | P2 |
| Operator | every lint result round-tripping to a violation item with its fields preserved | canceled scans stop before their completion milestone | P2 |

#### Proposed Solution

The dispatcher is the single source of truth for workspace action business logic: the CLI, MCP, and TUI surfaces call it for each workspace verb and format the returned results themselves. It supports single-file, subfolder, and whole-workspace scans, merges findings from every enabled rule group into one report, honours cooperative cancellation, and filters external tool output to workspace members.

#### Main User Flow

1. A surface invokes the dispatcher for a workspace verb.
2. The dispatcher runs the scan mode (single-file, subfolder, or whole-workspace) with cancellation flags, merges findings across enabled rule groups, and filters external-tool output to workspace members.
3. Each lint result round-trips to a violation item with fields preserved; a canceled scan stops before its completion milestone.

#### Acceptance Criteria

- `AC-PR-LA-004-1`: The same verb through the CLI and through MCP returns identical violation fields (file, line, rule, severity).
- `AC-PR-LA-004-2`: Cancelling before the completion milestone returns a partial report and does not report success.
- `AC-PR-LA-004-3`: A panic isolated to one rule group is reported as a violation while the remaining groups still produce findings.

---

### PR-LA-005: doc-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Release manager auditing a multi-package workspace before merge | the document chain checked for invariant drift with machine-readable findings | I can block a pull request on concrete violations instead of relying on manual document review | P1 |
| Reviewer | one finding per seam carrying its expected machine-readable violation type | the audit is concrete and comparable across seams | P1 |

#### Proposed Solution

Audits every recognized Markdown document against the ten document rules (AES601–AES610) and reports findings that carry machine-readable violation types. A whole-workspace audit is the intended default; a sub-directory run audits only what it discovers there, with crosslink and master-document invariants resolving against that audit root rather than the whole workspace.

#### Main User Flow

1. The auditor runs `lint-arwaky-cli docs .` for a whole-workspace audit or targets a sub-directory.
2. The document audit inspects every recognized Markdown document against AES601–AES610.
3. The auditor reviews the reported findings, each carrying a machine-readable violation type and the offending document path.

#### Acceptance Criteria

- `AC-PR-LA-005-1`: `docs .` audits the ten recognized documents and reports one finding per seam with its AES6xx rule code.
- `AC-PR-LA-005-2`: A sub-directory run resolves crosslinks and master-document invariants against that audit root, not the workspace root.
- `AC-PR-LA-005-3`: A document missing a required H2 exits 1 and names the document, the rule code, and the line.

---

### PR-LA-006: external-lint

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| DevOps engineer gating a mixed-language monorepo in CI | external linter findings from Clippy, Ruff, and ESLint surfaced in the same report as native rules | I enforce one quality gate instead of running each tool separately | P2 |
| Operator | auto-fix through each adapter's native fix command | I can remediate external-lint findings without leaving the tool | P2 |

#### Proposed Solution

Aggregates industry-standard linters and formatters — Clippy, Rustfmt, cargo-audit, Ruff, Mypy, Bandit, ESLint, Prettier, TSC, and markdownlint-cli — across Rust, Python, JS/TS, and Markdown, normalizing their reports into the unified violation format using tool-native rule codes and mapped severities. Auto-fix runs through each tool's native fix command. The tools run sequentially, and language detection is delegated to the filesystem source rather than performed here.

#### Main User Flow

1. The run detects present languages via the filesystem source and selects adapters accordingly.
2. Each adapter runs sequentially with per-tool timeouts; raw JSON/text output is normalized into unified violations carrying tool-native rule codes and mapped severities.
3. Results are post-filtered by ignored paths and suppressed rule codes; auto-fix is available through each adapter's native fix command.
4. An absent required adapter yields exit 3 with a visible scanner-missing warning; a timed-out adapter yields a partial report naming the tool; a run with no external findings yields exit 0.

#### Acceptance Criteria

- `AC-PR-LA-006-1`: Only the adapters for languages present in the workspace run; an absent tool is reported, never silently skipped.
- `AC-PR-LA-006-2`: Adapter output is normalized to the unified violation shape with tool-native codes and mapped severities.
- `AC-PR-LA-006-3`: A missing required adapter exits 3 with a visible warning, a timed-out adapter yields a partial report, and no external findings exits 0.

---

### PR-LA-007: file-watch

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Rust maintainer iterating on a mixed-language monorepo | the linter re-running automatically when I save a file | architectural violations surface within seconds without a manual scan command | P3 |
| Developer | a long-lived watch session producing fresh violation results as files are saved | I keep feedback live without reissuing scan commands | P3 |

#### Proposed Solution

The file-watch capability monitors configured directories for filesystem changes in real time and re-runs linting on the changed files. It debounces rapid bursts of changes, filters events to lintable files, removes duplicate events for the same path, and performs an initial full-project scan at session start.

#### Main User Flow

1. A watch session starts and performs an initial baseline scan of the whole project.
2. On a file save, the watcher debounces, filters to lintable files, deduplicates, and re-runs lint on the changed files.
3. Fresh violation results appear in the watch session's reported output within seconds of the save.

#### Acceptance Criteria

- `AC-PR-LA-007-1`: The initial full scan runs before watching starts and reports the same counts as a one-shot `scan .`.
- `AC-PR-LA-007-2`: Saving a violating file surfaces the finding within the debounce window without reissuing a command.
- `AC-PR-LA-007-3`: Ignored paths and suppressed rule codes stay filtered for the whole watch session.

---

### PR-LA-008: filesystem

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Rule developer adding a new architecture check | one place to get parsed files, dependency graphs, file contents, tool availability, and language flags | I can implement the rule without writing any filesystem code | P0 |
| Rule developer | parsed source files and extracted imports, identifiers, and declared symbols from a single source | I never reach into the filesystem directly from a rule | P0 |

#### Proposed Solution

A single source supplies every piece of filesystem-derived data the other capabilities need: it parses source files into structured entries and imports, builds a project-wide dependency graph, performs read, write, discovery, copy, remove, and permission operations, probes for external tool binaries, and detects the workspace root plus which languages are present. Rule and feature code consumes this data and never touches the filesystem directly.

#### Main User Flow

1. A rule requests filesystem-derived data through the single source.
2. The source parses source files, extracts imports, identifiers, and symbols, and builds the import and dependency graph with forward and reverse edges plus symbol maps.
3. The rule consumes parsed files, tool availability, and language-presence flags without writing any filesystem code.

#### Acceptance Criteria

- `AC-PR-LA-008-1`: Parsing yields entries, imports, and a project-wide dependency graph for Rust, Python, and TS; no rule reads the filesystem itself.
- `AC-PR-LA-008-2`: Workspace root and language presence are detected for a nested member, while a bare non-workspace path is rejected with exit 2.
- `AC-PR-LA-008-3`: Read, write, discovery, and remove operations honour the assembled ignored-path set.

---

### PR-LA-009: git-hooks

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Rust maintainer committing on a feature branch | the pre-commit hook linting only my changed files against the base branch | I learn about AES violations at commit time with the commit blocked and violations listed | P3 |
| Committer | a clean change producing no violations so the commit proceeds | I am not blocked when my change is conforming | P3 |

#### Proposed Solution

Installs, uninstalls, and runs a pre-commit hook that lints only the files changed against the base branch, blocking the commit when any changed file carries an AES violation. A clean change produces no violations and the commit proceeds; a file the user wrote that violates an AES rule blocks the commit with a named violation. `install-hook` creates a working pre-commit hook that fires on staged files.

#### Main User Flow

1. The maintainer installs the hook (idempotent, backing up any pre-existing custom hook).
2. On commit, the hook classifies changed files against the base branch, filters to lintable files, and lints them.
3. A clean change proceeds; a violating file blocks the commit with its named violation listed.

#### Acceptance Criteria

- `AC-PR-LA-009-1`: `install` writes a pre-commit hook that lints only the files changed against the base branch.
- `AC-PR-LA-009-2`: A commit introducing an AES violation is blocked and the finding is printed.
- `AC-PR-LA-009-3`: A commit that touches no violating file is allowed and exits 0.

---

### PR-LA-010: import-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Platform engineer guarding architecture in a Rust, Python, and TypeScript monorepo | every import validated against the layer-dependency matrix | forbidden cross-layer edges, missing mandatory imports, unused imports, dummy stubs, and cycles surface as lint failures | P1 |
| Reviewer | each violation naming the forbidden layer and the importing file | I can act on a precise architectural boundary breach | P1 |

#### Proposed Solution

The import-rules capability enforces correct structural boundaries and dependency flows across the 7-layer AES architecture by validating every import statement against a config-driven dependency matrix. It detects forbidden cross-layer imports, missing mandatory imports, unused imports, dummy stubs created to satisfy checkers, and circular cross-layer dependencies in Rust, Python, and TypeScript sources. All rule behaviour is governed by YAML configuration.

#### Main User Flow

1. The system parses every import statement across the workspace.
2. Each import is validated against the config-driven dependency matrix.
3. The user reviews any reported boundary violations, each naming the forbidden layer and importing file.

#### Acceptance Criteria

- `AC-PR-LA-010-1`: A forbidden cross-layer import yields AES201 naming the import, the file, and the source line.
- `AC-PR-LA-010-2`: A missing mandatory import (AES202) and an unused import (AES203) are each reported once per file, with no duplicate finding.
- `AC-PR-LA-010-3`: A dependency cycle (AES205) names every file participating in the cycle.

---

### PR-LA-011: logging

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Maintainer debugging a slow `lint-arwaky-cli scan .` on a large tree | raising log verbosity at runtime through the CLI flag without rebuilding or exporting an environment variable | I can pinpoint where the scan stalls | P2 |
| Operator | per-phase timings and skipped-directory reasons on stderr with stdout kept clean for piping | I can diagnose scan performance without polluting machine-readable output | P2 |

#### Proposed Solution

A single verbose flag (`-v`/`--verbose`) turns on the scan's real-time diagnostic feed on stderr: which directories it walks, which it skips and why, how long each linter phase takes, and the final result. Stdout stays clean for piping.

#### Main User Flow

1. The user runs `lint-arwaky scan -v` to enable verbosity.
2. The run emits structured phase-timing events (phase name + elapsed milliseconds) and walker-progress events (enter, skip-with-reason, discovered-count) on stderr at the enabled verbosity tier.
3. With verbosity disabled, zero events are emitted while the underlying scan counts remain identical.

#### Acceptance Criteria

- `AC-PR-LA-011-1`: `-v` adds per-phase timings on stderr and leaves stdout byte-identical to a non-verbose run.
- `AC-PR-LA-011-2`: No log file is written; every diagnostic goes to stderr.
- `AC-PR-LA-011-3`: An error names the failing phase and the actionable fix.

---

### PR-LA-012: maintenance

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Platform engineer on-call for a multi-repo CI fleet woken by a scan that suddenly started missing violations | a single upkeep check telling me which of the ten linter adapters is down | I can distinguish a bad environment from a bad rule set | P3 |
| Operator | knowing which toolchain version I am running and whether the installed binary is stale, in one call | I can decide whether to update tooling before re-scanning | P3 |

#### Proposed Solution

Before any lint run, answers "is this environment healthy and current": reports toolchain health across the Rust, Python, and JS toolchains; audits the availability of the ten bundled external-linter adapters; reclaims tool-owned cache artifacts; probes for newer versions of those adapters; and reports a dependency and security summary for the target project. It can atomically replace the installed lint-arwaky binary with a verified release. The outcome is structured data returned to the calling surface, never a lint verdict and never a code-change patch.

#### Main User Flow

1. The operator requests a maintenance health check.
2. The capability reports toolchain health, per-adapter availability across all ten external-linter adapters and the four language groups, and cache/tool/version statistics.
3. The operator reviews the health report with per-tool availability, version currency, and cache statistics.

#### Acceptance Criteria

- `AC-PR-LA-012-1`: `doctor` reports toolchain and binary state, lists missing tools, and exits 0.
- `AC-PR-LA-012-2`: `adapters` lists each adapter with its available or not-installed status.
- `AC-PR-LA-012-3`: Cache reclamation removes only the tool's own cache paths.

---

### PR-LA-013: mcp-server

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| AI agent authoring code in a fresh session with no prior lint context | listing, filtering, and executing linter commands through MCP tools against a confined workspace | I can detect and remediate architecture violations autonomously without a human running the CLI | P3 |
| AI agent | read-only authorization refusing mutating actions | I never accidentally mutate the workspace when only reading | P3 |

#### Proposed Solution

The MCP server exposes the linter to MCP clients: a listable, domain-filterable command catalog, execute and config tools wired into the shared dispatcher, a health check, and a docs scaffold capability. It confines every client-supplied path to the workspace and classifies mutating actions so a read-only authorization mode can refuse them, keeping MCP results identical to the CLI for the same operation.

#### Main User Flow

1. The client lists commands, optionally with a domain filter (empty result for an unknown domain).
2. The client executes a command wired into dispatcher actions with path validation; every client-supplied path is confined to the workspace.
3. Mutating actions are classified; under read-only authorization they are refused, and results match the CLI for the same operation.

#### Acceptance Criteria

- `AC-PR-LA-013-1`: `tools/list` returns exactly the five contracted tools (execute_command, list_commands, health_check, get_config, docs_scaffold) within 2 seconds.
- `AC-PR-LA-013-2`: `execute_command` on a path outside the workspace is refused with exit_code 2.
- `AC-PR-LA-013-3`: Read-only mode refuses mutating actions, and an MCP result matches the CLI result for the same operation.

---

### PR-LA-014: naming-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Maintainer curating a 10k-file monorepo under the AES 7-layer convention | every file name checked for layer, concern, and role | reviewers reject non-conforming names in CI before merge | P1 |
| Operator | each violation naming the violated rule the moment any single file name violates the convention | I can enforce the convention in CI | P1 |

#### Proposed Solution

Enforces the AES 7-layer naming convention across file names so every file's name reveals its layer, concern, and role. Validates stem shape, per-layer suffix and prefix policy, and test/bench file placement and prefix discipline, reporting each violation with the observed shape and concrete move-or-rename guidance.

#### Main User Flow

1. The system checks every file name against stem shape, per-layer policy, and test prefix rules.
2. The user reviews the naming violations report, with each entry showing the non-conforming name and suggested correction.

#### Acceptance Criteria

- `AC-PR-LA-014-1`: A stem violating the layer suffix policy yields AES102 naming the file and the expected suffix.
- `AC-PR-LA-014-2`: A test file without a legal prefix yields AES103; the same file with a legal prefix yields no finding.
- `AC-PR-LA-014-3`: A legacy file that predates the convention is reported, never renamed.

---

### PR-LA-015: orphan-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Maintainer of a Rust/Python/TypeScript monorepo where dead code accumulates across layers | orphaned files flagged per layer | I can remove unreachable components before they compound | P1 |
| Reviewer | each orphan yielding a rule-coded finding naming the file and the reachability gap | I can act on a precise reachability gap | P1 |

#### Proposed Solution

Detects dead, unused, or unreachable code components across the 7-layer AES architecture by tracing reachability from valid entry points (composition roots, binary entries, main files) through import and dependency-injection relationships, then flagging orphaned files in each layer. Each orphan yields a rule-coded finding (AES501–AES506) naming the file and the reason it is unreachable.

#### Main User Flow

1. The run traces reachability from container/main/binary entry points across import and DI relationships using graph data from the filesystem layer.
2. Per-layer orphan detection fires for taxonomy (AES501), contract (AES502), capabilities (AES503), utility (AES504), agent (AES505), and surface (AES506).
3. The user reviews the orphan report to identify unreachable components and their missing reference paths.

#### Acceptance Criteria

- `AC-PR-LA-015-1`: An unreachable module yields the AES5xx code for its layer together with the reachability gap.
- `AC-PR-LA-015-2`: Container, main, and binary entry points are never reported as orphans.
- `AC-PR-LA-015-3`: A file reachable only from a test is reported, because tests do not certify production reachability.

---

### PR-LA-016: project-setup

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Operator | scaffolding commands for new crates and members | project setup runs once per repo | P2 |

#### Proposed Solution

The project-setup capability scaffolds a new lint-arwaky crate or member folder from a template: Cargo.toml, FRD.md, BACKLOG.md, and the minimal source files. It registers the crate in the workspace Cargo.toml and verifies the AES7-layer layout.

#### Main User Flow

1. The operator runs `lint-arwaky-cli setup <crate>` to create a new crate folder.
2. The setup command writes the template files and updates workspace manifests.
3. The operator reviews the scaffolded crate and fills in feature specifics.

#### Acceptance Criteria

- `AC-PR-LA-016-1`: `init` writes a configuration the tool then reads without error, and `doctor` exits 0 afterwards.
- `AC-PR-LA-016-2`: An existing configuration is backed up rather than overwritten, and the run reports the backup path.
- `AC-PR-LA-016-3`: Docs scaffolding previews under `dry_run` and writes only when run without it.

---

### PR-LA-017: quality-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Reviewer | quality violations with named rule codes | quality gates can block CI on concrete defects | P2 |

#### Proposed Solution

The quality-rules capability enforces code quality standards (complexity, duplication, dead code) across all supported languages. It maps tool-native quality rule codes to AES5xx violation codes and reports each finding with the file and line.

#### Main User Flow

1. The run parses source files and computes quality metrics.
2. Each metric is compared against the configured threshold.
3. Violations are reported with rule code, file, and line.

#### Acceptance Criteria

- `AC-PR-LA-017-1`: A file above the line cap yields AES301 and one below the floor yields AES302, each naming the limit.
- `AC-PR-LA-017-2`: A bypass marker yields AES304 with the forbidden pattern quoted.
- `AC-PR-LA-017-3`: Duplicated code yields AES305 naming both locations.

---

### PR-LA-018: report-formatter

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| CI engineer | violation reports in a stable machine-readable format | pipelines can parse and act on lint results | P2 |

#### Proposed Solution

The report-formatter capability renders violation findings into human-readable and machine-readable formats (text, JSON, SARIF, JUnit). It groups findings by rule code and file, and applies suppression and threshold filtering.

#### Main User Flow

1. The dispatcher hands the merged findings to the report formatter.
2. The formatter renders findings grouped by rule code and file.
3. The report is written to stdout or to a report file.

#### Acceptance Criteria

- `AC-PR-LA-018-1`: Text, JSON, SARIF, and JUnit outputs carry the same findings, codes, paths, lines, and severities.
- `AC-PR-LA-018-2`: `report .` shows the per-member delta against the previous run and exits 0.
- `AC-PR-LA-018-3`: A report with zero findings renders an explicit clean line instead of an empty table.

---

### PR-LA-019: role-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Maintainer | every file checked for its AES layer and role | reviewers reject files in the wrong layer | P2 |

#### Proposed Solution

The role-rules capability validates that each file's name and contents match its assigned AES layer (agent, capabilities, contract, root, surface, taxonomy, utility). It reports files placed in the wrong layer or missing layer suffixes.

#### Main User Flow

1. The run classifies each file by its folder path and name suffix.
2. Each file is validated against its expected layer contract.
3. Violations are reported with the expected and observed layer.

#### Acceptance Criteria

- `AC-PR-LA-019-1`: A capabilities file implementing no contract yields AES403 naming the expected role.
- `AC-PR-LA-019-2`: A utility file carrying business rules yields AES404.
- `AC-PR-LA-019-3`: A surface file owning dispatch logic yields AES406.

---

### PR-LA-020: shared

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Rule developer | shared contracts, VOs, and parsers in one place | every rule consumes the same data model | P2 |

#### Proposed Solution

The shared capability hosts the contracts, value objects, doc-rules codegen, and document templates used by every other crate. It is a pure dependency: no surface logic, no business rules, just shared data types and helpers.

#### Main User Flow

1. A rule imports a shared contract VO.
2. The VO carries parsed file data, dependency edges, and language flags.
3. All rules read the same shape of data, preventing drift.

#### Acceptance Criteria

- `AC-PR-LA-020-1`: Every rule consumes the same value-object shape, so a field addition is a single-site change.
- `AC-PR-LA-020-2`: The shared crate imports no surface or capability crate (AES201 holds for shared).
- `AC-PR-LA-020-3`: Generated document constants match the templates; editing a template without regenerating fails the build.

---

### PR-LA-021: structure-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Platform engineer | folder structure validated against AES701–AES704 | structural drift is caught in CI | P2 |

#### Proposed Solution

The structure-rules capability enforces AES7-layer folder purity (AES701), feature health (AES702), surface purity (AES703), and test-suite coverage (AES704), plus PR-LA id to folder parity (AES706).

#### Main User Flow

1. The run walks every crate folder and checks its role and contents.
2. Each structure rule fires for its layer-specific contract.
3. Violations are reported with the rule code and file.

#### Acceptance Criteria

- `AC-PR-LA-021-1`: A feature folder missing a test category yields AES704 naming the missing category.
- `AC-PR-LA-021-2`: A shared folder holding business logic yields AES701.
- `AC-PR-LA-021-3`: A member-root file placed outside its layer folder yields AES705.

---

### PR-LA-022: tui

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Developer | a terminal UI for browsing violations | violations can be reviewed without leaving the terminal | P2 |

#### Proposed Solution

The tui capability provides a terminal user interface for lint-arwaky: browsing violation reports, navigating files, and triggering fix actions from within the terminal.

#### Main User Flow

1. The operator opens the TUI over a scan report.
2. The TUI lists violations grouped by rule code and file.
3. The operator selects a violation to view details or apply a fix.

#### Acceptance Criteria

- `AC-PR-LA-022-1`: Scanning shows progress in the TUI and Esc cancels the running scan instead of quitting.
- `AC-PR-LA-022-2`: Every destructive action requires an explicit confirmation and never auto-executes.
- `AC-PR-LA-022-3`: A terminal smaller than the supported minimum shows a readable notice rather than clipped output.

### Workspace Exit Code Contract

Every subcommand returns one of four codes. This section is the single definition; no other document restates it.

| Code | Meaning | Emitted when |
|------|---------|--------------|
| 0 | Ok / clean | The command completed and found nothing to fail on |
| 1 | Policy fail | Violations found, CI above threshold, or findings left un-applied after a fix |
| 2 | Runtime error | Bad path, invalid arguments, invalid configuration, dispatch failure |
| 3 | Prerequisite missing | A required external tool is not installed |

| Command | 0 | 1 | 2 | 3 |
|---------|---|---|---|---|
| `scan`, `check` | clean | violations found | bad path or invalid target | required tool missing |
| `ci` | at or below threshold | above threshold | runtime failure | required tool missing |
| `fix` | every fixable finding applied, or `--dry-run` | findings remain | runtime failure | required tool missing |
| `docs` | no document violation | document violation found | runtime failure | — |
| `report` | delta rendered | — | previous snapshot unreadable | — |
| `doctor`, `adapters` | diagnostic completed | — | runtime failure | missing tools are reported in the body, not as a code |
| `watch` | session ended cleanly | session ended with violations | runtime failure | required tool missing |
| MCP `execute_command` | mirrors the CLI verb | mirrors the CLI verb | dispatch or path error (`exit_code: 2`) | mirrors the CLI verb |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](ARCHITECTURE.md) |
