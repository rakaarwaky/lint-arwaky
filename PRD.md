# PRD — Lint Arwaky

> Product Requirements. Describes WHAT this project does and WHY. Real condition lives in ROADMAP.md.

## Project Overview

A multi-language monorepo has no single architectural audit. Engineers run separate linters per language and merge three reports by hand, so a violation in Rust never blocks a Python change and vice versa.

Developers, AI agents, and DevOps engineers on Rust/Python/TypeScript monorepos with 10k+ files and 5–50+ person teams lack one unified tool that audits all three languages together, enforces architectural rules in CI/CD, and remediates drift autonomously for humans and AI agents.

Without one gate, drift across naming, layering, imports, quality, roles, orphans, documents, and folder structure compounds silently until a full audit takes hours and releases carry hidden architectural defects.

## Goals

- One unified architectural audit that blocks cross-language drift in CI/CD before it reaches a release.
- Autonomous remediation of naming, layering, import, quality, role, orphan, document, and structure violations for humans and AI agents.
- Sub-minute violation reporting on a 10k-file mixed-language tree at CI scale.

## Non-Goals

- Redesigning individual language linters (Clippy, Ruff, ESLint) — their native rules stay owned by each tool.
- A merchant-side checkout customization tool or any business-domain feature unrelated to architectural audit.
- Persistent scan history or network services beyond local workspace inspection.
- Internationalized UI or multi-language natural-language output beyond English diagnostics.

## Target Users

| Persona | Need | Current Pain | Desired Outcome |
|---------|------|--------------|-----------------|
| Rule-crate developer | One filesystem aggregate supplying parsed files, dependency graphs, file contents, tool availability, and language flags | Touching the filesystem directly in every rule crate | Implement rules without writing filesystem code |
| CI engineer | `scan .` reporting violations and `ci . --threshold N` gating a release | Merging three per-language reports by hand | Branch on exit code instead of parsing text output |
| Platform engineer | Every import validated against the layer-dependency matrix | Forbidden cross-layer imports surfacing late | Forbidden edges, missing mandatory imports, unused imports, dummy stubs, and cycles surface as lint failures |
| Maintainer | Dead, unused, or unreachable code flagged per layer | Dead code compounding across layers until audit takes hours | Each orphan yields a rule-coded finding naming the file and reachability gap |

## Scope

### In Scope

- Filesystem-derived data as the single source: parse source files into structured entries and imports, build a project-wide dependency graph, perform read/write/discovery/copy/remove and permission operations, probe for external tool binaries, and detect workspace root plus language presence.
- Safe, deterministic auto-fix corrections (unused-import removal, bypass-marker stripping, mechanical symbol renaming) applied only when requested, with a preview mode reporting would-be changes without touching disk.
- Terminal surface: one surface command module per subcommand (scan, check, fix, ci, report, git, watch, setup, plugin, skill, config, maintenance) parsing arguments, invoking dispatch, and formatting results.
- Single source of truth for configuration: discovery across a bounded priority chain, workspace-type detection, merging user settings onto embedded defaults, threshold and adapter validation, ignored-path assembly.
- Dispatcher action modules for each workspace verb (check, scan, ci, fix, config, git, setup, watch, docs, import, external, maintenance, naming, orphan, plugin, quality, role, structure, layer scan, version, suppression) merging findings from every enabled rule group into one report.
- Document-invariant audit across AES601–AES607 with machine-readable violation types.
- External-lint aggregation of Clippy, Rustfmt, cargo-audit, Ruff, Mypy, Bandit, ESLint, Prettier, TSC, and markdownlint-cli normalized into unified violations.
- Real-time file watching with debouncing, filtering, deduplication, and an initial full-project scan.
- Pre-commit hook installing/linting only changed files against the base branch and blocking commits on AES violations.
- Import-rule enforcement across the 7-layer AES architecture (AES201–AES205) governed by YAML configuration.
- Centralized real-time diagnostic feed behind `-v`/`--verbose` with per-phase timings on stderr, stdout kept clean.
- Maintenance capability: environment/toolchain health, external-linter adapter availability, cache reclamation, version probing, dependency/security summary, and binary self-update.
- MCP server exposing listable, domain-filterable command catalog, execute and config tools, skill reader, and docs scaffold, confined to the workspace.
- AES naming convention enforcement (AES101–AES103) across file names with stem shape, per-layer suffix/prefix policy, and test/bench prefix placement.
- Orphan detection (AES501–AES506) tracing reachability from container/main/binary entry points across import and DI relationships.

### Out of Scope

- Rule evaluation and violation reporting driven by the filesystem crate itself; auto-fix or file mutation driven by lint findings.
- Multi-file renames, structural refactors, adding new imports or types, semantic rewrites such as panic removal, formatting-only passes.
- Business logic and orchestration living outside the dispatcher and capability layers; IDE plugins, web dashboard, and cloud SaaS surfaces.
- Secret or credential scanning; semantic analysis beyond AST extraction; network or remote filesystem access.
- Language detection and tree walking owned outside the filesystem aggregate; concurrent or parallel adapter execution.
- IDE plugins or editor integrations; hot-reloading configuration during a running watch session; remote or cloud-based watching.
- Post-commit and pre-push hooks; whole-repository scans in the commit gate; auto-fixing of violations the hook blocks on.
- Compilation, type checking, or runtime behaviour analysis; dependency installation or package management.
- Persistent log files, non-stderr output channels, asynchronous or distributed scan execution, persistent scan history, network services.
- Server-side timeouts or cancellation (client owns the timeout); watch command over MCP; MCP JSON-RPC-level success status.
- Renaming legacy files that predate the convention; automatic renaming of files (reporting only).
- Certifying behavioural deadness that only tests can prove; auto-removal or rewriting of flagged files; cross-repository or distributed scanning.

---

## Project Requirements

### PR-LA-001: filesystem

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Rule-crate developer adding a new architecture check | one filesystem aggregate supplying parsed files, dependency graphs, file contents, tool availability, and language flags | I can implement the rule without writing any filesystem code | P0 |
| Rule-crate developer | parsed source files and extracted imports, identifiers, and declared symbols in one aggregate | I never touch the filesystem directly from a rule crate | P1 |

#### Proposed Solution

The filesystem crate is the single source of filesystem-derived data for every other crate: it parses source files into structured entries and imports, builds a project-wide dependency graph, performs read/write/discovery/copy/remove and permission operations, probes for external tool binaries, and detects the workspace root plus which languages are present. All of this is exposed through one aggregate so rule and feature crates never touch the filesystem directly.

#### Main User Flow

1. A rule crate requests filesystem-derived data through the filesystem aggregate.
2. The aggregate parses source files, extracts imports/identifiers/symbols, and builds the import/dependency graph with forward and reverse edges plus symbol maps.
3. The rule crate consumes parsed files, tool availability, and language-presence flags without writing filesystem code.

---

### PR-LA-002: auto-fix

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Rust maintainer resolving flagged unused imports | the flagged line removed automatically in place or previewed first | I can clean violations without hand-editing every flagged file | P0 |
| Maintainer | a reason-coded result stating whether a fix applied, was skipped, or failed and why | I know exactly what changed and what needs manual work | P1 |

#### Proposed Solution

Applies safe, deterministic corrections to source files that violate architectural rules, writing the corrected files back to disk only when requested. It removes unused imports, strips known bypass markers, and mechanically renames flagged symbols — each a local, single-file change that never restructures surrounding code. A preview mode reports the exact changes that would be made without touching any file on disk; every attempt yields a reason-coded result stating whether a fix applied, was skipped, or failed and why.

#### Main User Flow

1. The maintainer runs auto-fix in preview mode over a flagged violation report.
2. The system reports every would-be change with a reason-coded outcome (applied, skipped with reason, or failed with reason) while changing zero bytes on disk.
3. The maintainer applies the fixes, rewriting every fixable violation exactly once.

---

### PR-LA-003: cli-commands

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| CI engineer gating releases on a mixed-language monorepo | `scan .` reporting violations and `ci . --threshold N` gating a release | my pipeline can branch on the exit code instead of parsing text output | P0 |
| Operator | every command exiting with the code the workspace exit-code contract assigns to its outcome | I can script reliable release gates | P1 |

#### Proposed Solution

The cli-commands crate is the terminal surface of the tool: one surface command module per subcommand (scan, check, fix, ci, report, git, watch, setup, plugin, skill, config, maintenance) parses arguments, invokes the dispatch layer, and formats the returned result. Report-oriented subcommands render snapshots, per-member deltas, and formatted text through shared kernel output helpers; every command exits with the code the workspace exit-code contract assigns to its outcome.

#### Main User Flow

1. The operator invokes a subcommand with arguments.
2. The surface module parses arguments, delegates workspace work through the dispatcher actions, and formats the result.
3. The command exits with the contract exit code; the caller branches on status without parsing output.

---

### PR-LA-004: config-system

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| DevOps engineer owning the single lint_arwaky.config.yaml for a multi-language monorepo team | config discovery, merging, validation, and ignored-path resolution to happen once and be reused by every command | a single config edit changes lint behaviour across the whole tool without touching command code | P0 |
| Operator | a project with no config file scoring through valid embedded defaults | I can adopt the tool without authoring configuration | P1 |

#### Proposed Solution

The config-system crate is the single source of truth for lint-arwaky configuration: it discovers a project's config file across a bounded priority chain, detects the workspace type and its member directories, merges user settings onto embedded defaults, validates thresholds and adapter enablement, and assembles the ignored-path set that every command honours. It caches each parsed config by content so repeated reads stay cheap, and refuses unsafe inputs such as symlinks escaping the project root or crafted language values.

#### Main User Flow

1. A command starts and the config system discovers the config file across the five-level priority chain.
2. It merges user settings onto embedded defaults, validates thresholds and adapter enablement, and assembles the ignored-path set.
3. Every command honours the cached, validated configuration on that run.

---

### PR-LA-005: dispatcher

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Integrator adding a new Lint Arwaky surface | one shared action entry point per workspace verb | CLI, MCP, and TUI behave identically without duplicating business logic | P0 |
| Operator | every lint result round-tripping to a violation item with its fields preserved | canceled scans stop before their completion milestone | P1 |

#### Proposed Solution

The dispatcher crate is the single source of truth for workspace action business logic: the CLI, MCP, and TUI surfaces call its action modules for each workspace verb and format the returned results themselves. It supports single-file, subfolder, and whole-workspace scans, merges findings from every enabled rule group into one report, honors cooperative cancellation, and filters external tool output to workspace members.

#### Main User Flow

1. A surface invokes the dispatcher action module for a workspace verb.
2. The dispatcher runs the scan mode (single-file, subfolder, or whole-workspace) with cancellation flags, merges findings across enabled rule groups, and filters external-tool output to workspace members.
3. Each lint result round-trips to a violation item with fields preserved; a canceled scan stops before its completion milestone.

---

### PR-LA-006: doc-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Release manager auditing a multi-crate workspace before merge | the document chain checked for invariant drift with machine-readable findings | I can block a pull request on concrete violations instead of relying on manual document review | P0 |
| Reviewer | one finding per seam carrying its expected machine-readable violation type | the audit is concrete and comparable across seams | P1 |

#### Proposed Solution

The doc-rules crate enforces the AES document-invariant contract across a workspace by auditing every recognized Markdown document against the seven named document rules (AES601–AES607) and reporting findings that carry machine-readable violation types. A whole-workspace audit is the intended default; a sub-directory run audits only what it discovers there, with crosslink and master-document invariants resolving against that audit root rather than the whole workspace.

#### Main User Flow

1. The auditor runs `lint-arwaky-cli docs .` for a whole-workspace audit (or targets a sub-directory).
2. The crate audits every recognized Markdown document against AES601–AES607.
3. On a conforming workspace the run reports zero findings; a workspace with one violation per seam yields at least one finding per seam carrying its expected machine-readable violation type.

---

### PR-LA-007: external-lint

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| DevOps engineer gating a mixed-language monorepo in CI | external linter findings from Clippy, Ruff, and ESLint surfaced in the same report as native rules | I enforce one quality gate instead of running each tool separately | P0 |
| Operator | auto-fix through each adapter's native fix command | I can remediate external-lint findings without leaving the tool | P1 |

#### Proposed Solution

The external-lint crate aggregates industry-standard linters and formatters — Cargo Clippy, Rustfmt, cargo-audit, Ruff, Mypy, Bandit, ESLint, Prettier, TSC, and markdownlint-cli — across Rust, Python, JS/TS, and Markdown, normalizing their reports into the unified violation format using tool-native rule codes and mapped severities. It also exposes auto-fix through each adapter's native fix command. All adapters run sequentially as subprocesses, and language detection is delegated to the filesystem aggregate rather than performed by this crate.

#### Main User Flow

1. The run detects present languages via the filesystem aggregate and selects adapters accordingly.
2. Each adapter runs sequentially as a subprocess with per-tool timeouts; raw JSON/text output is normalized into unified violations carrying tool-native rule codes and mapped severities.
3. Results are post-filtered by ignored paths and suppressed rule codes; auto-fix is available through each adapter's native fix command.

---

### PR-LA-008: file-watch

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Rust maintainer iterating on a mixed-language monorepo | the linter re-running automatically when I save a file | architectural violations surface within seconds without a manual scan command | P0 |
| Developer | a long-lived watch session producing fresh violation results as files are saved | I keep feedback live without reissuing scan commands | P1 |

#### Proposed Solution

The file-watch capability monitors configured directories for filesystem changes in real time and re-runs linting on the changed files. It debounces rapid bursts of changes, filters events to lintable files, removes duplicate events for the same path, and performs an initial full-project scan at session start.

#### Main User Flow

1. A watch session starts and performs an initial baseline scan of the whole project.
2. On a file save, the watcher debounces, filters to lintable files, deduplicates, and re-runs lint on the changed files.
3. Fresh violation results appear in the watch session's reported output within seconds of the save.

---

### PR-LA-009: git-hooks

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Rust maintainer committing on a feature branch | the pre-commit hook linting only my changed files against the base branch | I learn about AES violations at commit time with the commit blocked and violations listed | P0 |
| Committer | a clean change producing no violations so the commit proceeds | I am not blocked when my change is conforming | P1 |

#### Proposed Solution

Installs, uninstalls, and runs a pre-commit hook that lints only the files changed against the base branch, blocking the commit when any changed file carries an AES violation. A clean change produces no violations and the commit proceeds; a file the user wrote that violates an AES rule blocks the commit with a named violation. `install-hook` creates a working pre-commit hook that fires on staged files.

#### Main User Flow

1. The maintainer installs the hook (idempotent, backing up any pre-existing custom hook).
2. On commit, the hook classifies changed files against the base branch, filters to lintable files, and lints them.
3. A clean change proceeds; a violating file blocks the commit with its named violation listed.

---

### PR-LA-010: import-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Platform engineer guarding architecture in a Rust, Python, and TypeScript monorepo | every import validated against the layer-dependency matrix | forbidden cross-layer edges, missing mandatory imports, unused imports, dummy stubs, and cycles surface as lint failures | P0 |
| Reviewer | each violation naming the forbidden layer and the importing file | I can act on a precise architectural boundary breach | P1 |

#### Proposed Solution

The import-rules crate enforces correct structural boundaries and dependency flows across the 7-layer AES architecture by validating every import statement against a config-driven dependency matrix. It detects forbidden cross-layer imports, missing mandatory imports, unused imports, dummy stubs created to satisfy checkers, and circular cross-layer dependencies in Rust, Python, and TypeScript sources. All rule behaviour is governed by YAML configuration.

#### Main User Flow

1. The run parses every import statement in Rust, Python, and TypeScript sources.
2. Each import is validated against the config-driven dependency matrix (AES201 forbidden-layer, AES202 mandatory-layer, AES203 unused-import, AES204 dummy construct, AES205 circular dependency).
3. A clean repository exits 0 with zero findings; any violation names the forbidden layer and the importing file.

---

### PR-LA-011: logging

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Maintainer debugging a slow `la scan .` on a large tree | raising log verbosity at runtime through the CLI flag without rebuilding or exporting an environment variable | I can pinpoint where the scan stalls | P0 |
| Operator | per-phase timings and skipped-directory reasons on stderr with stdout kept clean for piping | I can diagnose scan performance without polluting machine-readable output | P1 |

#### Proposed Solution

The logging crate centralizes the scan's real-time diagnostic feed behind a single orchestrator so that `lint-arwaky scan -v` writes to stderr what it is doing: which directories it walks, which it skips and why, how long each linter phase takes, and what the final result is. It exposes a verbosity value type driven by the `-v`/`--verbose` flag and keeps stdout clean for piping.

#### Main User Flow

1. The user runs `lint-arwaky scan -v` to enable verbosity.
2. The crate emits structured phase-timing events (phase name + elapsed milliseconds) and walker-progress events (enter, skip-with-reason, discovered-count) on stderr at the enabled verbosity tier.
3. With verbosity disabled, zero events are emitted while the underlying scan counts remain identical.

---

### PR-LA-012: maintenance

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Platform engineer on-call for a multi-repo CI fleet woken by a scan that suddenly started missing violations | a single upkeep check telling me which of the ten linter adapters is down | I can distinguish a bad environment from a bad rule set | P0 |
| Operator | knowing which toolchain version I am running and whether the installed binary is stale, in one call | I can decide whether to update tooling before re-scanning | P1 |

#### Proposed Solution

The maintenance capability answers "is this environment healthy and current" before any lint run. It reports environment health across the Rust, Python, and JS toolchains, audits the availability of the bundled external-linter adapters, reclaims tool-owned cache artifacts, probes for newer versions of those adapters, reports a dependency and security summary for the target project, and can atomically replace the installed lint-arwaky binary with a verified release. The outcome is structured data returned to the calling surface, never a lint verdict and never a code-change patch. All commands return codes 0/1/2/3 per the contract; MCP `execute_command` carries `exit_code`.

#### Main User Flow

1. The operator requests a maintenance health check.
2. The capability reports toolchain health, per-adapter availability across all ten external-linter adapters and the four language groups, and cache/tool/version statistics.
3. At least one missing adapter is flagged unavailable while every other entry still reports a definite status; no entry is dropped and no probe failure blocks the remaining probes.

---

### PR-LA-013: mcp-server

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| AI agent authoring code in a fresh session with no prior lint context | listing, filtering, and executing linter commands through MCP tools against a confined workspace | I can detect and remediate architecture violations autonomously without a human running the CLI | P0 |
| AI agent | read-only authorization refusing mutating actions | I never accidentally mutate the workspace when only reading | P1 |

#### Proposed Solution

The mcp-server crate exposes the linter to MCP clients over a local server: a listable, domain-filterable command catalog, execute and config tools wired straight into the dispatcher actions, a skill reader for prompt material, and a docs scaffold capability. It confines every client-supplied path to the workspace and classifies mutating actions so a read-only authorization mode can refuse them, keeping MCP results identical to the CLI for the same operation.

#### Main User Flow

1. The client lists commands, optionally with a domain filter (empty result for an unknown domain).
2. The client executes a command wired into dispatcher actions with path validation; every client-supplied path is confined to the workspace.
3. Mutating actions are classified; under read-only authorization they are refused, and results match the CLI for the same operation.

---

### PR-LA-014: naming-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Maintainer curating a 10k-file monorepo under the AES 7-layer convention | every file name checked for layer, concern, and role | reviewers reject non-conforming names in CI before merge | P0 |
| Operator | each violation naming the violated rule the moment any single file name violates the convention | I can enforce the convention in CI | P1 |

#### Proposed Solution

Enforces the AES 7-layer naming convention across file names so every file's name reveals its layer, concern, and role. Validates stem shape, per-layer suffix and prefix policy, and test/bench file placement and prefix discipline, reporting each violation with the observed shape and concrete move-or-rename guidance.

#### Main User Flow

1. The run checks every file name for stem shape (AES101), per-layer suffix/prefix policy (AES102), and test/bench prefix and placement discipline (AES103).
2. A fully conforming tree exits 0 with zero findings; any single violating file name exits 1 with at least one finding naming the violated rule.

---

### PR-LA-015: orphan-rules

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Maintainer of a Rust/Python/TypeScript monorepo where dead code accumulates across layers | orphaned files flagged per layer | I can remove unreachable components before they compound | P0 |
| Reviewer | each orphan yielding a rule-coded finding naming the file and the reachability gap | I can act on a precise reachability gap | P1 |

#### Proposed Solution

Detects dead, unused, or unreachable code components across the 7-layer AES architecture by tracing reachability from valid entry points (containers, binary entries, main files) through import and dependency-injection relationships, then flagging orphaned files in each layer. Graph data is supplied by the filesystem layer, so analysis performs no I/O itself. Each orphan yields a rule-coded finding (AES501–AES506) naming the file and the reason it is unreachable.

#### Main User Flow

1. The run traces reachability from container/main/binary entry points across import and DI relationships using graph data from the filesystem layer.
2. Per-layer orphan detection fires for taxonomy (AES501), contract (AES502), capabilities (AES503), utility (AES504), agent (AES505), and surface (AES506).
3. Each orphaned file yields exactly one finding of its layer code; each live/wired/referenced file yields zero.

---

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](ARCHITECTURE.md) |
