# PRD — Lint Arwaky

> Product Requirements. Describes WHAT this project does and WHY. Real condition lives in ROADMAP.md.

## Problem Statement

A multi-language monorepo has no single architectural audit. Engineers run separate linters per language and merge three reports by hand, so a violation in Rust never blocks a Python change and vice versa.

Developers, AI agents, and DevOps engineers on Rust/Python/TypeScript monorepos with 10k+ files and 5–50+ person teams lack one unified tool that audits all three languages together, enforces architectural rules in CI/CD, and remediates drift autonomously for humans and AI agents.

Without one gate, drift across naming, layering, imports, quality, roles, orphans, documents, and folder structure compounds silently until a full audit takes hours and releases carry hidden architectural defects.

## Project Requirements

### PR-LA-001: filesystem

#### Description

| No | Detail |
| --- | --- |
| 1 | The filesystem crate is the single source of filesystem-derived data for every other crate: it parses source files into structured entries and imports, builds a project-wide dependency graph, performs read/write/discovery/copy/remove and permission operations, probes for external tool binaries, and detects the workspace root plus which languages are present. |
| 2 | All of this is exposed through one aggregate so rule and feature crates never touch the filesystem directly. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a rule-crate developer adding a new architecture check, I want one filesystem aggregate to supply parsed files, dependency graphs, file contents, tool availability, and language flags. |
| 2 | I want to implement the rule without writing any filesystem code. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Parse source files and extract imports, identifiers, and declared symbols; build a project-wide import/dependency graph with forward and reverse edges plus symbol maps. |
| 2 | Read, write, discover, copy, remove files and directories while honouring ignored paths; probe PATH for external tool binaries and report availability; detect the workspace root and the Rust, Python, JavaScript, and Markdown language-presence flags. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Rule evaluation and violation reporting; auto-fix or file mutation driven by lint findings. |
| 2 | Secret or credential scanning; semantic analysis beyond AST extraction; network or remote filesystem access. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | Every capability group exits 0 with zero failures on a single run of the crate. |
| 2 | A single run of the crate completes without any failures. |

### PR-LA-002: auto-fix

#### Description

| No | Detail |
| --- | --- |
| 1 | Applies safe, deterministic corrections to source files that violate architectural rules, writing the corrected files back to disk only when requested. It removes unused imports, strips known bypass markers, and mechanically renames flagged symbols — each a local, single-file change that never restructures surrounding code. |
| 2 | A preview mode reports the exact changes that would be made without touching any file on disk; every attempt yields a reason-coded result stating whether a fix applied, was skipped, or failed and why. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a Rust maintainer resolving flagged unused imports, I want the flagged line removed automatically in place or previewed first. |
| 2 | I want to clean violations without hand-editing every flagged file. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Unused import removal on flagged lines; stripping of known bypass markers (allow attributes, hack comments, inline skip markers, unwrap calls); mechanical word-boundary renaming of flagged symbols. |
| 2 | Preview reporting of all would-be changes without writing; a reason-coded outcome (applied, skipped with reason, or failed with reason) for every fix attempt. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Multi-file renames, structural refactors, adding new imports or types, semantic rewrites such as panic removal, formatting-only passes. |
| 2 | Violations with no mechanical fixer are reported as manual work, never rewritten. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | A single run over a flagged violation report changes zero bytes on disk in preview mode while accounting for every flagged violation by a reason-coded result. |
| 2 | Rewriting every fixable violation exactly once when applying. |

### PR-LA-003: cli-commands

#### Description

| No | Detail |
| --- | --- |
| 1 | The cli-commands crate is the terminal surface of the tool: one surface command module per subcommand (scan, check, fix, ci, report, git, watch, setup, plugin, skill, config, maintenance) parses arguments, invokes the dispatch layer, and formats the returned result. |
| 2 | Report-oriented subcommands render snapshots, per-member deltas, and formatted text through shared kernel output helpers; every command exits with the code the workspace exit-code contract assigns to its outcome. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a CI engineer gating releases on a mixed-language monorepo, I want `scan .` to report violations and `ci . --threshold N` to gate a release. |
| 2 | I want my pipeline to branch on the exit code instead of parsing text output. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Argument parsing and result formatting for all twelve subcommands (scan, check, fix, ci, report, git, watch, setup, plugin, skill, config, maintenance). |
| 2 | Delegation of workspace work through the dispatcher actions; report snapshot, per-member delta, and formatted-text rendering via the kernel output helpers; stable exit codes per the workspace exit-code contract; `docs <path>` accepting an audit root rather than a filter. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Business logic and orchestration, which live in the dispatcher and capability layers; IDE plugins, web dashboard, and cloud SaaS surfaces. |
| 2 | MCP and TUI surfaces, secret detection, and external linter wrapping belong to other crates. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | All twelve subcommands return their contract exit code on a known-fixture run. |
| 2 | Callers can branch on status without parsing output. |

### PR-LA-004: config-system

#### Description

| No | Detail |
| --- | --- |
| 1 | The config-system crate is the single source of truth for lint-arwaky configuration: it discovers a project's config file across a bounded priority chain, detects the workspace type and its member directories, merges user settings onto embedded defaults, validates thresholds and adapter enablement, and assembles the ignored-path set that every command honours. |
| 2 | It caches each parsed config by content so repeated reads stay cheap, and refuses unsafe inputs such as symlinks escaping the project root or crafted language values. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a DevOps engineer who owns the single lint_arwaky.config.yaml for a multi-language monorepo team, I want config discovery, merging, validation, and ignored-path resolution to happen once and be reused by every command. |
| 2 | I want a single config edit to change lint behaviour across the whole tool without touching command code. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Config discovery across the five-level priority chain with first-match-wins and embedded-default fallback; workspace type detection and member enumeration under crates/, packages/, and modules/. |
| 2 | Rule-based merge of user configuration onto embedded defaults including TOML [tool.lint-arwaky] sections; threshold and adapter-enablement validation with accumulated errors; ignored-path assembly merging defaults with config additions while deduplicating and filtering empties; per-key content-hash caching; symlink and path-injection safeguards. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Rule evaluation and violation reporting; auto-fix and file mutation; external linter execution; file watching and incremental re-scans; git-hook installation; secret or credential scanning. |
| 2 | IDE plugins, web dashboards, and cloud SaaS; MCP transport and tool exposure; network or remote filesystem access. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | 100% of seeded configuration changes (threshold, ignored path, adapter toggle, rule enabled/exception flag) take effect on the next command run with zero config errors. |
| 2 | A project with no config file always scores through valid embedded defaults. |

### PR-LA-005: dispatcher

#### Description

| No | Detail |
| --- | --- |
| 1 | The dispatcher crate is the single source of truth for workspace action business logic: the CLI, MCP, and TUI surfaces call its action modules for each workspace verb and format the returned results themselves. |
| 2 | It supports single-file, subfolder, and whole-workspace scans, merges findings from every enabled rule group into one report, honors cooperative cancellation, and filters external tool output to workspace members. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As an integrator adding a new Lint Arwaky surface, I want one shared action entry point per workspace verb. |
| 2 | I want CLI, MCP, and TUI to behave identically without duplicating business logic. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Workspace-verb action modules (check, scan, ci, fix, config, git, setup, watch, docs, import, external, maintenance, naming, orphan, plugin, quality, role, structure, layer scan, version, suppression); single-file, subfolder, and whole-workspace scan modes with cancellation flags. |
| 2 | Merged violation reports across enabled rule groups; external-tool output filtering to workspace members; violation-item construction from lint results and JSON with default filling; per-run entries store; shared output-component formatting. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Surface-specific rendering and presentation; command-line parsing and interactive TUI workflows; rule-engine and language-parsing implementations. |
| 2 | Asynchronous or distributed scan execution; persistent scan history or network services. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | Every workspace verb maps to exactly one action module. |
| 2 | Every lint result round-trips to a violation item with its fields preserved, and every canceled scan stops before its completion milestone. |

### PR-LA-006: doc-rules

#### Description

| No | Detail |
| --- | --- |
| 1 | The doc-rules crate enforces the AES document-invariant contract across a workspace by auditing every recognized Markdown document against the seven named document rules (AES601–AES607) and reporting findings that carry machine-readable violation types. |
| 2 | A whole-workspace audit is the intended default; a sub-directory run audits only what it discovers there, with crosslink and master-document invariants resolving against that audit root rather than the whole workspace. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a release manager auditing a multi-crate workspace before merge, I want the document chain checked for invariant drift with machine-readable findings. |
| 2 | I want to block a pull request on concrete violations instead of relying on manual document review. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | FR-ID format, ID-family membership, and FR/protocol-class parity with BACKLOG qualifier alignment; section-order and sanctioned-shape conformance; spec purity against source paths and status vocabulary. |
| 2 | Crosslink integrity with master-only state vocabulary; one-H1/required-H2/verbatim-line heading structure; template table-shape and entry-table conformance; required root-document presence; member-folder document presence by role; template body and placeholder conformance; PR-ID to module-folder ownership. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Auto-fix or rewriting of documents; external tool invocation; auditing business-logic correctness or runtime behaviour of the rules. |
| 2 | Treating a sub-directory audit as a filtered view of a whole-workspace audit; structural or multi-file semantic rewrites. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | On a conforming workspace, `lint-arwaky-cli docs .` reports zero findings across all ten seams. |
| 2 | A workspace containing one violation per seam yields at least one finding per seam carrying its expected machine-readable violation type. |

### PR-LA-007: external-lint

#### Description

| No | Detail |
| --- | --- |
| 1 | The external-lint crate aggregates industry-standard linters and formatters — Cargo Clippy, Rustfmt, cargo-audit, Ruff, Mypy, Bandit, ESLint, Prettier, TSC, and markdownlint-cli — across Rust, Python, JS/TS, and Markdown, normalizing their reports into the unified violation format using tool-native rule codes and mapped severities. |
| 2 | It also exposes auto-fix through each adapter's native fix command. All adapters run sequentially as subprocesses, and language detection is delegated to the filesystem aggregate rather than performed by this crate. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a DevOps engineer gating a mixed-language monorepo in CI, I want external linter findings from Clippy, Ruff, and ESLint surfaced in the same report as native rules. |
| 2 | I want to enforce one quality gate instead of running each tool separately. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Language-driven adapter selection across ten default adapters (clippy, rustfmt, cargo-audit, ruff, mypy, bandit, eslint, prettier, tsc, markdownlint); subprocess execution with per-tool timeouts (120–180s for Rust tools, 60s for others). |
| 2 | Normalization of raw JSON/text output into unified violations carrying tool-native rule codes and mapped severities; auto-fix invocation through each adapter's native fix command; post-filtering of results by ignored paths and suppressed rule codes. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Language detection and tree walking (owned by the filesystem aggregate); concurrent or parallel adapter execution (adapters run sequentially). |
| 2 | Structural or multi-file semantic rewrites beyond each tool's native fix; an overall timeout budget across the adapter phase. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | 100% of seeded findings from every wrapped tool appear in the unified report with their tool-native rule codes when all adapters run cleanly. |
| 2 | When no language is detected the run reports zero adapter findings and exits 0. |

### PR-LA-008: file-watch

#### Description

| No | Detail |
| --- | --- |
| 1 | The file-watch capability monitors configured directories for filesystem changes in real time and re-runs linting on the changed files. |
| 2 | It debounces rapid bursts of changes, filters events to lintable files, removes duplicate events for the same path, and performs an initial full-project scan at session start. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a Rust maintainer iterating on a mixed-language monorepo with live re-lint feedback, I want the linter to re-run automatically when I save a file. |
| 2 | I want architectural violations to surface within seconds without issuing a manual scan command. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Real-time filesystem change detection with debouncing; filtering and deduplication of change events to lintable files; per-file lint re-invocation after each retained change. |
| 2 | Start, stop, and availability lifecycle for a watch session; initial baseline scan of the whole project at session start. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | IDE plugins or editor integrations; hot-reloading configuration during a running session (restart required). |
| 2 | Remote or cloud-based watching; structural or multi-file auto-fixes. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | On a 1,000-file fixture tree, 100% of saved lintable files produce a re-lint result in the watch session's reported output within 5 seconds of the save. |
| 2 | Developers keep a long-lived watch session running and receive fresh violation results as files are saved. |

### PR-LA-009: git-hooks

#### Description

| No | Detail |
| --- | --- |
| 1 | Installs, uninstalls, and runs a pre-commit hook that lints only the files changed against the base branch, blocking the commit when any changed file carries an AES violation. |
| 2 | A clean change produces no violations and the commit proceeds; a file the user wrote that violates an AES rule blocks the commit with a named violation. `install-hook` creates a working pre-commit hook that fires on staged files. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a Rust maintainer committing on a feature branch, I want the pre-commit hook to lint only my changed files against the base branch. |
| 2 | I want to learn about AES violations at commit time with the commit blocked and violations listed. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Diff detection and pre-commit check (changed-file classification against the base branch, lintable-file filter, blocking on violations). |
| 2 | Idempotent hook installation with backup of pre-existing custom hooks; idempotent hook uninstallation that restores the pre-install state; project config initialization and ignore-rule add/remove; stable manager identity (git hook manager). |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Post-commit and pre-push hooks; whole-repository scans in the commit gate (only changed files are linted). |
| 2 | Auto-fixing of the violations the hook blocks on; enforcing lint policy outside the commit gate. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | In a repository seeded with one AES violation in a changed file, running the pre-commit check once exits nonzero with at least one reported violation for that file. |
| 2 | Installing the hook in a repository with a custom pre-commit hook leaves the original content intact in a backup file and reinstalls idempotently with no duplicate backups. |

### PR-LA-010: import-rules

#### Description

| No | Detail |
| --- | --- |
| 1 | The import-rules crate enforces correct structural boundaries and dependency flows across the 7-layer AES architecture by validating every import statement against a config-driven dependency matrix. |
| 2 | It detects forbidden cross-layer imports, missing mandatory imports, unused imports, dummy stubs created to satisfy checkers, and circular cross-layer dependencies in Rust, Python, and TypeScript sources. All rule behaviour is governed by YAML configuration. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a platform engineer guarding architecture in a Rust, Python, and TypeScript monorepo, I want every import validated against the layer-dependency matrix. |
| 2 | I want forbidden cross-layer edges, missing mandatory imports, unused imports, dummy stubs, and cycles to surface as lint failures. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Forbidden-layer import detection (AES201); mandatory-layer import enforcement (AES202); unused import detection (AES203). |
| 2 | Dummy construct detection (AES204); circular dependency detection (AES205). |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Secret or credential detection (hardcoded API keys, tokens, passwords). Auto-remediation or applying fixes to violating files (detection and reporting only). |
| 2 | Compilation, type checking, or runtime behaviour analysis. Dependency installation or package management. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | Every run on a clean repository exits 0 with zero findings. |
| 2 | Any repository containing at least one forbidden layer edge, missing mandatory import, unused import, dummy construct, or cycle exits non-zero, and each violation names the forbidden layer and the importing file. |

### PR-LA-011: logging

#### Description

| No | Detail |
| --- | --- |
| 1 | The logging crate centralizes the scan's real-time diagnostic feed behind a single orchestrator so that `lint-arwaky scan -v` writes to stderr what it is doing: which directories it walks, which it skips and why, how long each linter phase takes, and what the final result is. |
| 2 | It exposes a verbosity value type driven by the `-v`/`--verbose` flag and keeps stdout clean for piping. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a maintainer debugging a slow `la scan .` on a large tree, I want to raise the log verbosity at runtime through the CLI flag without rebuilding or exporting an environment variable. |
| 2 | I want to see per-phase timings and skipped-directory reasons on stderr and pinpoint where the scan stalls. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Filter construction that maps verbosity levels (default, info, debug) to directives for the root and audit targets with unknown levels falling back to default. |
| 2 | Global subscriber installation that is idempotent and honours an ANSI on/off setting; structured phase-timing events that emit a phase name and elapsed milliseconds; walker-progress events reporting enter, skip-with-reason, and discovered-count signals visible only at the enabled verbosity tier. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | No application logic, no scan results, no linter output, no file content, no configuration file parsing, no network or storage I/O. |
| 2 | No persistent log files, and no non-stderr output channels. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | A seeded scan run with verbosity enabled emits a machine-checkable trail on stderr in which reported enter plus skip events equal the seeded file count. |
| 2 | Every phase emits a structured event with a positive elapsed-millisecond value; verbosity disabled produces zero emitted events while the underlying scan counts remain identical. |

### PR-LA-012: maintenance

#### Description

| No | Detail |
| --- | --- |
| 1 | The maintenance capability answers "is this environment healthy and current" before any lint run. It reports environment health across the Rust, Python, and JS toolchains, audits the availability of the bundled external-linter adapters, reclaims tool-owned cache artifacts, probes for newer versions of those adapters, reports a dependency and security summary for the target project, and can atomically replace the installed lint-arwaky binary with a verified release. |
| 2 | The outcome is structured data returned to the calling surface, never a lint verdict and never a code-change patch. All commands return codes 0/1/2/3 per the contract; MCP `execute_command` carries `exit_code`. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a platform engineer on-call for a multi-repo CI fleet woken by a scan that suddenly started missing violations, I want a single upkeep check that tells me which of the ten linter adapters is down. |
| 2 | I want to know which version of the toolchain I am running and whether the installed binary itself is stale — in one call — so I can distinguish a bad environment from a bad rule set. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Environment and toolchain health reporting (doctor); per-adapter availability checks across all ten external-linter adapters and the four language groups. |
| 2 | Tool-owned cache cleanup that never touches user source or config; per-adapter version probing and installation of the managed tools; dependency listing and security advisory scanning of the target project; binary self-update from GitHub releases with integrity verification before replace; project statistics (file counts by language). |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Lint verdicts, architecture-rule enforcement, and any violation reporting; automatic fixing of findings the security scan reports. |
| 2 | The maintenance capability surfaces findings but applies no patch. CLI exit-code contracts and the MCP command envelope are owned by the cli-commands and mcp-server surfaces that merely format this capability's output. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | A single health check against a seeded environment returns exactly ten adapter entries covering the Rust, Python, JS, and Markdown groups. |
| 2 | At least one missing adapter is flagged as unavailable while every other entry still reports a definite status — no entry is dropped and no probe failure blocks the remaining probes. |

### PR-LA-013: mcp-server

#### Description

| No | Detail |
| --- | --- |
| 1 | The mcp-server crate exposes the linter to MCP clients over a local server: a listable, domain-filterable command catalog, execute and config tools wired straight into the dispatcher actions, a skill reader for prompt material, and a docs scaffold capability. |
| 2 | It confines every client-supplied path to the workspace and classifies mutating actions so a read-only authorization mode can refuse them, keeping MCP results identical to the CLI for the same operation. `crates/*/tests/` acceptance test inventory maps to FR IDs in `crates/*/FRD.md`. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As an AI agent authoring code in a fresh session with no prior lint context, I want to list, filter, and execute linter commands through MCP tools against a confined workspace. |
| 2 | I want to detect and remediate architecture violations autonomously without a human running the CLI. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Command catalog (list_commands) with optional domain filter and empty result for an unknown domain; command execution (execute_command) wired into dispatcher actions with path validation. |
| 2 | Workspace path confinement rejecting out-of-workspace paths; mutating-action classification enabling read-only authorization refusal; get_config returning effective architecture configuration; read_skill reading skill documentation by section; docs scaffold writing missing templates with dry-run preview. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Server-side timeouts or cancellation (the client owns the timeout); business logic beyond argument validation, catalog rules, and authorization (delegated to the dispatcher). |
| 2 | The watch command over MCP; MCP JSON-RPC-level success status (the envelope carries exit_code). |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | Every seeded catalog entry satisfies the name/description/example triple. |
| 2 | Every seeded traversal path is rejected, and every seeded mutating action under read-only authorization is refused. |

### PR-LA-014: naming-rules

#### Description

| No | Detail |
| --- | --- |
| 1 | Enforces the AES 7-layer naming convention across file names so every file's name reveals its layer, concern, and role. |
| 2 | Validates stem shape, per-layer suffix and prefix policy, and test/bench file placement and prefix discipline, reporting each violation with the observed shape and concrete move-or-rename guidance. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a maintainer curating a 10k-file monorepo under the AES 7-layer convention, I want every file name checked for layer, concern, and role. |
| 2 | I want reviewers to reject non-conforming names in CI before merge. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | AES101 stem-shape validation (mixed-case identifiers, all-uppercase rejection, support-directory flattening, separator normalization, barrel skipping). |
| 2 | AES102 per-layer suffix/prefix policy; AES103 test and bench prefix and placement discipline. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Renaming legacy files that predate the convention (a dedicated refactor owns that); automatic renaming of files (reporting only). |
| 2 | File discovery and I/O (delegated to the filesystem crate); semantic or runtime correctness checks. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | `lint-arwaky-cli check .` exits 0 with zero AES101/AES102/AES103 findings on a fully conforming tree. |
| 2 | Exits 1 with at least one finding naming the violated rule the moment any single file name violates the convention. |

### PR-LA-015: orphan-rules

#### Description

| No | Detail |
| --- | --- |
| 1 | Detects dead, unused, or unreachable code components across the 7-layer AES architecture by tracing reachability from valid entry points (containers, binary entries, main files) through import and dependency-injection relationships, then flagging orphaned files in each layer. |
| 2 | Graph data is supplied by the filesystem layer, so analysis performs no I/O itself. Each orphan yields a rule-coded finding (AES501–AES506) naming the file and the reason it is unreachable. |

#### User Story

| No | Detail |
| --- | --- |
| 1 | As a maintainer of a Rust/Python/TypeScript monorepo where dead code accumulates across layers, I want orphaned files flagged per layer so I can remove unreachable components before they compound. |
| 2 | Each orphan yields a rule-coded finding naming the file and the reachability gap. |

#### In Scope

| No | Detail |
| --- | --- |
| 1 | Reachability tracing from container/main/binary entry points across import and DI relationships; per-layer orphan detection for taxonomy (AES501), contract (AES502), capabilities (AES503), utility (AES504), agent (AES505), and surface (AES506). |
| 2 | Per-file findings naming the file and the reachability gap; configurable entry-point patterns and per-rule enable/disable. |

#### Out of Scope

| No | Detail |
| --- | --- |
| 1 | Certifying behavioural deadness that only tests can prove; auto-removal or rewriting of flagged files. |
| 2 | Cross-repository or distributed scanning; GPU acceleration. |

#### Success Metric

| No | Detail |
| --- | --- |
| 1 | Each orphaned file yields exactly one finding of its layer code (AES501–AES506). |
| 2 | Each live/wired/referenced file yields zero, and an empty graph yields zero errors. |

## Reference

| Artifact | Purpose | Link|
|----------|---------|-----|
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](ROADMAP.md)|
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](ARCHITECTURE.md)|
