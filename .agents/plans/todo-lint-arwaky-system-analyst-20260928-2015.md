# Plan: Lint Arwaky (product-level, all crates) — System Analyst

## Summary

This analysis covers the whole Lint Arwaky product surface as specified in `PRD.md`: a Rust workspace that audits Rust/Python/TypeScript code against AES rules and exposes the same capability through three surfaces (CLI, MCP, TUI) plus auto-fix, reporting, git hooks, and watch mode. Impacted modules are every member under `crates/` — the rule groups (`naming-rules`, `import-rules`, `quality-rules`, `role-rules`, `orphan-rules`, `doc-rules`, `structure-rules`), the orchestration layer (`dispatcher`), the three surfaces (`cli-commands` + `crates/root_cli_main_entry.rs`, `mcp-server`, `tui`), and the shared kernel (`crates/shared`) that owns the taxonomy and contracts. Technical constraints are fixed by the PRD: synchronous `std::thread`/`rayon` execution (async only in `file-watch` and `mcp-server`), tree-sitter AST parsing with no regex fallback, filesystem I/O centralized in `filesystem`, and a four-value exit-code contract (0/1/2/3). The technical specification is **not yet implementation-ready**: the two newest rule groups (AES6xx doc, AES7xx structure) landed in code but the product specification still describes a 24-rule / 5-group system, the CLI command set is declared twice in two diverging places, doc findings do not share the workspace violation entity, and the MCP `execute_command` action set no longer matches the CLI it claims parity with. Five CRITICAL, seven WARNING, and three INFO issues are raised below; the CRITICAL set must close before the next feature cycle because three of them invalidate PRD success metrics 2, 3, and 4. Note: `cargo` is not available in this analysis sandbox, so all findings are derived from static reading of specifications, configuration, and source — the two self-audit findings (SA-5-12, SA-5-13) carry an explicit "verify by running the binary" acceptance criterion.

### Scope 1: Specify the feature technical behavior
<!-- Check feature input and output, system behavior, processing steps, system boundaries, feature scope, technical assumptions, and impacted modules -->

#### Issue SA-1-01-20260928-2015

Title: [SA][CRITICAL] CLI command surface is declared twice and the two declarations have diverged

Label: interface, severity-critical

Location Path:
`crates/root_cli_main_entry.rs:32` (`enum Command`) · `crates/shared/src/cli_commands/taxonomy_command_vo.rs:41` (`enum Commands`)

Description: The product has two independent clap command enumerations. The binary actually parses `Command` in `crates/root_cli_main_entry.rs`, whose 26 variants are `Scan Check Quality Role Import Naming Orphan External Docs Ci Config Fix Git Doctor Security Dependencies Adapters Init Install McpConfig Watch InstallHook UninstallHook Version Update Skill`. The shared taxonomy `Commands` VO declares a different set: it has no `Docs`, `Update`, `Skill`, or `Check` variant, and it names two commands differently (`ConfigShow` → `config-show`, `GitDiff` → `git-diff`) from the shipped binary (`config`, `git`). `Commands` is referenced only by `crates/shared/tests/unit_shared_cli_commands.rs`, so the workspace test suite asserts the behavior of a command surface no user can invoke. Feature behavior — which inputs the system accepts — therefore has no single authoritative definition, and every future command must be added in two places or silently drift again.

Acceptance Criteria: `rg "enum Command" crates/` returns exactly one declaration; the binary's parser is constructed from that single type; `crates/shared/tests/unit_shared_cli_commands.rs` exercises the same type the binary uses; `lint-arwaky-cli docs --help`, `... update --help`, and `... skill --help` all succeed and each has a matching variant in the single enum.

Recommendation: Promote the binary's `Command` enum into `shared::cli_commands::taxonomy_command_vo` as the one source of truth, delete the stale `Commands` enum, and have `crates/root_cli_main_entry.rs` import it. Keep `check`/`scan` as clap aliases rather than separate variants.

Git Diff:

```diff
--- a/crates/shared/src/cli_commands/taxonomy_command_vo.rs
+++ b/crates/shared/src/cli_commands/taxonomy_command_vo.rs
-#[derive(Subcommand, Debug)]
-pub enum Commands {
-    /// Run all linters and calculate score.
-    #[command(alias = "check")]
-    Scan { ... }
-    /// Show active configuration
-    ConfigShow,
-}
+/// Single source of truth for the CLI verb set. The binary
+/// (`crates/root_cli_main_entry.rs`) parses this type; no second
+/// declaration is permitted.
+#[derive(Subcommand, Debug)]
+pub enum Commands {
+    #[command(alias = "check")]
+    Scan { ... },
+    /// Audit the Markdown document chain (AES601-AES607).
+    Docs { path: Option<String> },
+    /// Self-update the binary.
+    Update,
+    /// Read a skill document.
+    Skill { section: Option<String> },
+    /// Show active configuration
+    #[command(name = "config")]
+    Config,
+}
```

Open Questions: Should `check` remain a clap alias of `scan`, or become a distinct narrower verb (the MCP layer treats `"check" | "scan"` as the same action)?

#### Issue SA-1-02-20260928-2015

Title: [SA][CRITICAL] `check`/`scan` does not run the doc-rule group, so the self-audit metric cannot cover AES601–AES607

Label: scenario, severity-critical

Location Path:
`crates/dispatcher/src/surface_check_action.rs:70` (`collect_scan` / `ScanAggregates`) · `crates/cli-commands/src/surface_scan_command.rs:243` (`handle_docs`) · `PRD.md` → "Self-auditing capability"

Description: `ScanAggregates` wires `quality`, `role`, `import`, `naming`, `external`, `orphan`, `config`, and `structure`, but not the doc orchestrator. The AES6xx group is reachable only through the separate `docs` subcommand, which is invoked through its own path (`handle_docs` → `collect_docs`) and returns plain strings instead of participating in the scan pipeline. PRD success metric 4 states "`lint-arwaky-cli check .` on this repo → 0 violations", and PRD metric 2 states "all rule codes produce violations on `workspaces-bad`". Neither statement can be evaluated for AES601–AES607 today, because `check` never asks the doc group anything. The feature boundary between "what `check` covers" and "what `docs` covers" is undocumented in `PRD.md`, so the two commands' scopes are an implementation accident rather than a decision.

Acceptance Criteria: Either (a) `ScanAggregates` gains a `docs` member and `lint-arwaky-cli check .` reports AES601–AES607 findings for a workspace with a broken FRD, or (b) `PRD.md` explicitly records that the self-audit metric is the pair `check .` + `docs .` and CI runs both. A test in `crates/doc-rules/tests/` asserts the chosen behavior.

Recommendation: Prefer (a). Add `docs: Arc<dyn IDocRunnerAggregate>` to `ScanAggregates` and fold doc findings into the `Vec<ViolationItem>` stream (depends on SA-2-04). Keep `docs` as a focused subcommand for fast document-only runs.

Git Diff:

```diff
--- a/crates/dispatcher/src/surface_check_action.rs
+++ b/crates/dispatcher/src/surface_check_action.rs
 pub struct ScanAggregates {
     pub quality: Arc<dyn ICodeAnalysisAggregate>,
     pub orphan: Arc<dyn IOrphanAggregate>,
     pub config: Arc<dyn IConfigOrchestratorAggregate>,
     pub structure: Arc<dyn IStructureAggregate>,
+    /// AES601-AES607. Walks the document chain itself; consumes no file index.
+    pub docs: Arc<dyn IDocRunnerAggregate>,
     pub fs_seam: Arc<FilesystemSeam>,
 }
```

Open Questions: Should the doc group be skippable for source-only scans (e.g. `--skip-group docs`) to keep `watch` cheap?

#### Issue SA-1-03-20260928-2015

Title: [SA][WARNING] Doc-rule scope is stated as AES601–AES605 in three places while AES606 and AES607 ship

Label: scenario, severity-warning

Location Path:
`crates/cli-commands/src/surface_scan_command.rs:240` (doc comment) · `crates/doc-rules/FRD.md` → Requirements · `lint_arwaky.config.yaml:476`

Description: `crates/doc-rules/src/lib.rs` correctly states the group covers AES601–AES607, but the CLI `handle_docs` doc comment still says "doc invariant audit (AES601–AES605)", the `doc-rules` FRD lists three requirements that predate both AES606 and AES607, and the FRD's Scenarios table names only AES605 and AES606. A reader of the specification cannot tell whether AES607 is in scope for the `docs` command at all. Related: `DocRequest` exposes a single verb, `AuditAll { root }` — there is no single-document verb, so the system boundary for incremental callers (`watch`, `git`) is unspecified.

Acceptance Criteria: Every in-repo mention of the doc group's rule range reads `AES601–AES607`; `crates/doc-rules/FRD.md` carries one requirement row per shipped behavior including heading structure and FR/protocol parity; the FRD states explicitly whether an incremental single-document audit is in or out of scope.

Recommendation: Update the comment and the FRD in the same change that adds a rule to the group; add an `AuditDoc { path }` verb to `DocRequest` only if SA-4-11 resolves in favor of incremental watch support, otherwise record "whole-chain audit only" as a locked technical assumption.

Git Diff:

```diff
--- a/crates/cli-commands/src/surface_scan_command.rs
+++ b/crates/cli-commands/src/surface_scan_command.rs
-/// `docs` — doc invariant audit (AES601–AES605) over the workspace document
+/// `docs` — doc invariant audit (AES601-AES607) over the workspace document
 /// chain. Bypasses the file index because the doc chain is a fixed file set,
 /// and calls the doc orchestrator directly on the target path.
```

Open Questions: Is a single-document audit verb needed for `watch`, or is the whole-chain audit cheap enough to re-run on every save?

### Scope 2: Design the logical data model
<!-- Check entity definition, key attributes, entity relationships, business constraints, data lifecycle, and CRUD operations required by the feature -->

#### Issue SA-2-04-20260928-2015

Title: [SA][CRITICAL] `DocFinding` is not a violation entity — no line, column, or severity

Label: data, severity-critical

Location Path:
`crates/shared/src/doc_rules/taxonomy_doc_request.rs:16` (`DocFinding`) · `crates/shared/src/common/taxonomy_violation_item_vo.rs:14` (`ViolationItem`)

Description: Every other rule group emits `ViolationItem { code: ErrorCode, file: FilePath, line: LineNumber, column: ColumnNumber, message: LintMessage, severity: Severity }`. The doc group emits `DocFinding { code: &'static str, violation_type: &'static str, doc: String, message: String }` — raw strings, no position, no severity. Three consequences follow. (1) `crates/doc-rules/FRD.md` sets the NFR "Every finding names a file and a line number"; the entity cannot satisfy it. (2) SARIF 2.1.0 and JUnit XML reporting, a P1 PRD requirement, requires a region and a level per result; doc findings cannot be rendered in either format. (3) Doc findings cannot be merged into the scan violation stream (SA-1-02) or scored by the compliance calculator, because `utility_compliance_score` consumes severities. The doc group therefore sits outside the product's data model rather than inside it.

Acceptance Criteria: `DocFinding` carries a line number and a severity (or is replaced by `ViolationItem`); `lint-arwaky-cli docs . --format sarif` emits a valid SARIF result with a `region.startLine` for each finding; the doc-rules NFR "every finding names a file and a line number" is verified by a test that asserts a non-zero line on each finding.

Recommendation: Give the checker the line index it already has while parsing the Markdown and map `DocFinding` onto `ViolationItem` at the dispatcher boundary, using the per-rule severity from `lint_arwaky.config.yaml`.

Git Diff:

```diff
--- a/crates/shared/src/doc_rules/taxonomy_doc_request.rs
+++ b/crates/shared/src/doc_rules/taxonomy_doc_request.rs
 pub struct DocFinding {
     pub code: &'static str,
     pub violation_type: &'static str,
     pub doc: String,
+    /// 1-based line in `doc` the invariant fired on; required for SARIF
+    /// regions and for parity with `ViolationItem`.
+    pub line: LineNumber,
+    /// Severity resolved from the rule's config entry, not hardcoded.
+    pub severity: Severity,
     pub message: String,
 }
```

Open Questions: For whole-document invariants such as AES606 `h1_count`, is line 1 an acceptable anchor, or should the entity carry `Option<LineNumber>`?

#### Issue SA-2-05-20260928-2015

Title: [SA][CRITICAL] AES607 has no entry in `lint_arwaky.config.yaml`, so the rule cannot be configured

Label: data, severity-critical

Location Path:
`lint_arwaky.config.yaml:476-498` (`architecture.rules`, AES600 block)

Description: The configuration data model registers one key per rule: AES101–AES102, AES201–AES205, AES301–AES305, AES401–AES406, AES501–AES506, AES601–AES606, AES701–AES705. AES607 (FR/Protocol Parity), shipped in commit `779480d`, is absent. Every other rule can be disabled, scoped, or re-severitied through config; AES607 cannot. This breaks the implicit business constraint "a rule is a configurable entity" and makes the config file an incomplete projection of the rule catalog. There is no schema-level guard preventing the next rule from landing the same way.

Acceptance Criteria: `AES607` appears under `architecture.rules` with `enabled` and `severity`; setting `AES607.enabled: false` suppresses its findings in a `docs` run; a test (or the config validator) fails when a rule code known to the engine has no config key.

Recommendation: Add the missing key and add a startup validation that diffs the engine's rule registry against the config keys, reporting any rule present in one and missing in the other.

Git Diff:

```diff
--- a/lint_arwaky.config.yaml
+++ b/lint_arwaky.config.yaml
     AES606: # Doc heading structure — h1_count | h2_missing | h2_unexpected
       enabled: true
       severity: HIGH
 
+    AES607: # FR/protocol parity — fr_count vs capability-seam class count
+      enabled: true
+      severity: HIGH
+
     # ─── AES700: Folder structure ───
```

Open Questions: Should the config validator treat an unknown rule key as a hard error or a warning, given user configs may target older binaries?

#### Issue SA-2-06-20260928-2015

Title: [SA][WARNING] Doc findings cross the dispatcher boundary as `Vec<String>`, discarding entity identity

Label: data, severity-warning

Location Path:
`crates/dispatcher/src/surface_docs_action.rs:16` (`collect_docs`)

Description: `collect_docs` receives structured `DocFinding` values and immediately flattens them to `format!("{} {}: {}: {}", code, violation_type, doc, message)`. Downstream code — the CLI, and any future MCP or TUI consumer — can only re-parse that string. The finding's lifecycle therefore ends at the dispatcher: it cannot be filtered by `--filter AES606`, cannot be counted per code, cannot be suppressed by `ignored_rules`, and cannot be serialized as JSON. Every other `collect_*` dispatcher function returns typed values (`Vec<ViolationItem>`, `ToolchainDiagnostics`, `CiReport`), so this is a local inconsistency in the data model, not a product-wide convention.

Acceptance Criteria: `collect_docs` returns typed findings; `--filter AES606` narrows the `docs` output; `ignored_rules` in config suppresses a doc rule; text formatting happens only in `cli-commands`.

Recommendation: Return `Result<Vec<ViolationItem>, String>` (after SA-2-04) and move the `format!` into `utility_output_text_formatter`.

Git Diff:

```diff
--- a/crates/dispatcher/src/surface_docs_action.rs
+++ b/crates/dispatcher/src/surface_docs_action.rs
-pub fn collect_docs(
-    root: &str,
-    doc_orchestrator: Arc<dyn IDocRunnerAggregate>,
-) -> Result<Vec<String>, String> {
+pub fn collect_docs(
+    root: &str,
+    doc_orchestrator: Arc<dyn IDocRunnerAggregate>,
+) -> Result<Vec<ViolationItem>, String> {
-    Ok(findings.into_iter().map(|f| format!("{} {}: {}: {}", f.code, f.violation_type, f.doc, f.message)).collect())
+    Ok(findings.into_iter().map(ViolationItem::from_doc_finding).collect())
 }
```

Open Questions: Does `violation_type` survive the mapping as part of the message, or does it need a first-class field on `ViolationItem` for machine routing?

### Scope 3: Design the API contract
<!-- Check endpoint definition, request and response schema, status codes, error contract, versioning, and contract between frontend backend and external systems -->

#### Issue SA-3-07-20260928-2015

Title: [SA][CRITICAL] MCP `execute_command` is missing five CLI actions, breaking the stated full-parity contract

Label: integration, severity-critical

Location Path:
`crates/mcp-server/src/surface_mcp_action_command.rs:421-484` (action match) · `PRD.md` → "MCP server with 5 tools, full execute parity"

Description: The action match accepts `check|scan, ci, fix, doctor, orphan, security, quality, import, naming, role, external, dependencies, version, watch, adapters, install-hook, uninstall-hook, init, install, mcp-config, config-show`. The shipped CLI additionally offers `docs`, `git`, `update`, `skill`, and names its config command `config`, not `config-show`. An agent calling `execute_command { action: "docs" }` or `{ action: "config" }` receives `{"error": "Unknown action: docs", "exit_code": 2}` — a runtime error for a command the product documents as available. PRD success metric 3 ("every CLI command is reachable via `execute_command`") is therefore not met, and there is no automated contract test that would catch the next divergence.

Acceptance Criteria: A contract test enumerates the CLI verb set and asserts `execute_command` returns a non-"Unknown action" response for each; `{"action":"docs"}` returns doc findings; `{"action":"config"}` is accepted as an alias of `config-show`.

Recommendation: Derive the accepted action set from the single command enum (SA-1-01) instead of a hand-written match, and add the missing arms. Fail the build on any enum variant with no MCP arm.

Git Diff:

```diff
--- a/crates/mcp-server/src/surface_mcp_action_command.rs
+++ b/crates/mcp-server/src/surface_mcp_action_command.rs
             "version" => self.execute_version(),
             "watch" => self.execute_watch(),
+            "docs" => self.execute_docs(path),
+            "git" => self.execute_git_diff(path),
+            "skill" => self.handle_read_skill(None),
+            "update" => self.execute_self_update(),
-            "config-show" => {
+            "config" | "config-show" => {
```

Open Questions: Should `update` (self-update) be exposed over MCP at all, or deliberately excluded as a privileged operation — and if excluded, should the parity metric be reworded?

#### Issue SA-3-08-20260928-2015

Title: [SA][CRITICAL] `COMMAND_CATALOG` — the contract behind `list_commands` — omits shipped commands

Label: integration, severity-critical

Location Path:
`crates/shared/src/cli_commands/taxonomy_command_vo.rs:291` (`COMMAND_CATALOG`)

Description: `list_commands` is the discovery endpoint agents use to learn the product's verb set, and it is rendered entirely from `COMMAND_CATALOG`. The catalog lists 21 entries and omits `docs`, `update`, `skill`, and `structure`-related scanning guidance entirely. An AI agent following the documented workflow can never discover the document audit — the exact capability the AES6xx group was built to provide. The catalog is also hand-maintained as a `&[(&str, &str, &str)]` tuple slice with no compile-time link to the command enum, so it will drift again on the next command.

Acceptance Criteria: `list_commands` returns an entry for every CLI verb; a test asserts `catalog_names == command_enum_names`; each entry's `example` string is executable verbatim.

Recommendation: Generate the catalog from the clap command metadata (`Command::get_subcommands()`) at startup, or keep the static table but add the equality test so drift fails CI.

Git Diff:

```diff
--- a/crates/shared/src/cli_commands/taxonomy_command_vo.rs
+++ b/crates/shared/src/cli_commands/taxonomy_command_vo.rs
     (
         "doctor",
         "System health diagnostics",
         "lint-arwaky-cli doctor",
     ),
+    (
+        "docs",
+        "Audit the Markdown document chain (AES601-AES607)",
+        "lint-arwaky-cli docs .",
+    ),
+    (
+        "skill",
+        "Read an AES skill document by section",
+        "lint-arwaky-cli skill aes-docs",
+    ),
+    (
+        "update",
+        "Self-update the binary to the latest release",
+        "lint-arwaky-cli update",
+    ),
```

Open Questions: Should `list_commands` expose each command's parameter schema too, so agents stop guessing `args` keys (see SA-3-09)?

#### Issue SA-3-09-20260928-2015

Title: [SA][WARNING] MCP request/response schema is untyped, unversioned, and offers no format parity

Label: integration, severity-warning

Location Path:
`crates/mcp-server/src/surface_mcp_tool_command.rs:41-70` (`handle_execute_command`) · `crates/mcp-server/src/surface_mcp_action_command.rs` (JSON builders)

Description: Three contract gaps compound here. (1) Request: `execute_command` reads exactly three keys out of a free-form `args` object — `path` (default `"."`), `threshold` (default `80`), `dry_run` (default `false`) — and silently ignores everything else, so the CLI's `--member`, `--filter`, `--format`, `--base`, and `--client` flags are unreachable and a typo in a key name fails silently as a default. (2) Response: each arm hand-builds a `serde_json::json!` object; there is no declared schema and no `schema_version` field, so consumers cannot detect a breaking change. The `status` string vocabulary is inconsistent across arms (`success|failure`, `pass|fail`, `clean|findings|tool_missing`, `partial`). (3) Errors: the error contract is the ad-hoc shape `{"error": String, "exit_code": n}` with no stable error code, so clients must string-match. The PRD's P1 requirement for SARIF/JUnit output has no MCP path at all.

Acceptance Criteria: `ExecuteCommandArgs` is a typed struct with named optional fields covering every CLI flag, and unknown keys are rejected with a documented error; every response carries `schema_version` and a `status` drawn from one documented enum; errors carry a stable `error_code`; `execute_command` accepts `format: "sarif"|"junit"|"json"`.

Recommendation: Define the request/response pair as taxonomy types in `shared::mcp_server`, serialize through `serde` instead of `json!` literals, and reuse the existing report-formatter aggregate for `format`.

Git Diff:

```diff
--- a/crates/shared/src/mcp_server/taxonomy_mcp_tool_args_vo.rs
+++ b/crates/shared/src/mcp_server/taxonomy_mcp_tool_args_vo.rs
-pub struct ExecuteCommandArgs {
-    pub action: String,
-    pub args: Option<serde_json::Value>,
-}
+#[serde(deny_unknown_fields)]
+pub struct ExecuteCommandArgs {
+    pub action: String,
+    pub path: Option<String>,
+    pub threshold: Option<u32>,
+    pub dry_run: Option<bool>,
+    pub member: Option<String>,
+    pub filter: Option<String>,
+    pub format: Option<Format>,
+    pub base: Option<String>,
+}
```

Open Questions: Is a breaking change to the MCP argument shape acceptable now, or must the loose `args` object stay supported for one release with a deprecation warning?

### Scope 4: Map the sequence diagram and edge cases
<!-- Check component interaction, happy path, failure path, timeout scenario, retry behavior, fallback path, and technical edge cases from business requirements -->

#### Issue SA-4-10-20260928-2015

Title: [SA][CRITICAL] Exit code 3 (prerequisite missing) is emitted on only one path, so the failure contract is unenforced

Label: scenario, severity-critical

Location Path:
`crates/cli-commands/src/surface_maintenance_command.rs:83` · `crates/external-lint/FRD.md:232` · `PRD.md` → "Exit Code Contract"

Description: The PRD defines exit code 3 as "Required external tool not installed", a product-wide contract. In the workspace, `ExitCode::PREREQUISITE_MISSING` is constructed in exactly one place. The external-lint specification says the opposite for the same condition: "Missing binary mapped to 'tool not found' warning… All errors are per-adapter", and "Adapter timeout exceeded → error logged, other adapters continue". So `lint-arwaky-cli external .` on a machine with no `ruff` and no `eslint` completes with exit 0 or 1 rather than 3. There is no specified rule for which commands must escalate a missing prerequisite to code 3 and which may degrade to a warning, so CI consumers cannot write a reliable gate.

Acceptance Criteria: `PRD.md` states, per command family, whether a missing external tool yields 3 or degrades to a warning; each command family has a test asserting its documented code; a run with all external tools absent produces the documented code for `external`, `scan`, `ci`, and `doctor`.

Recommendation: Keep degradation as the default for aggregate scans (a partial scan is still useful) and reserve 3 for single-tool commands and for `--require-adapters`. Record that decision in the PRD's locked-decisions table, then align the code.

Git Diff:

```diff
--- a/PRD.md
+++ b/PRD.md
 | 3 | Prerequisite missing | Required external tool not installed |
+
+Escalation rule: aggregate commands (`scan`, `check`, `ci`, `external`) degrade a
+missing adapter to a warning and keep their 0/1 outcome; single-tool commands and
+any run passing `--require-adapters` return 3. `doctor` always returns 0 unless it
+fails internally.
```

Open Questions: Should `ci` escalate to 3 when a *weighted* adapter (weight > 1.0, e.g. `architecture`) is unavailable, since the score is then not comparable across runs?

#### Issue SA-4-11-20260928-2015

Title: [SA][WARNING] `watch` re-runs only the source-code groups; doc and structure drift is invisible until a manual scan

Label: scenario, severity-warning

Location Path:
`crates/dispatcher/src/surface_watch_action.rs:13` · `crates/file-watch/FRD.md` (FR-001, 200 ms debounce)

Description: Watch mode re-triggers analysis through an injected `ICodeAnalysisAggregate` only. Editing `FRD.md`, deleting a `BACKLOG.md`, or moving a `capabilities_*` file into `shared/` — the precise conditions AES6xx and AES7xx exist to catch — produces no feedback in a watch session. The sequence for a change event is also unspecified beyond the debounce: there is no documented behavior for a change arriving while a scan is in flight (queue, cancel, or drop), and no fallback if the watcher backend fails mid-session after a successful start (`notify` inotify watch limits are a realistic failure on large monorepos).

Acceptance Criteria: `file-watch/FRD.md` contains a sequence for: event → debounce → in-flight scan present → documented resolution; a watch session over a workspace where `FRD.md` is edited reports the resulting AES6xx finding (or the FRD documents the exclusion and the reason); inotify watch-limit exhaustion produces a documented user-visible message rather than silence.

Recommendation: Route watch re-scans through the same `collect_scan` entry point the CLI uses (which, after SA-1-02, includes doc and structure), and specify "latest-wins: a new event cancels the in-flight scan" as the concurrency rule.

Git Diff:

```diff
--- a/crates/file-watch/FRD.md
+++ b/crates/file-watch/FRD.md
+### Edge case: change during an in-flight scan
+
+A debounced event arriving while a scan is running cancels that scan and starts a
+new one (latest-wins). No event is queued; at most one scan runs at a time.
+
+### Edge case: watcher backend failure after start
+
+If the OS watch limit is exhausted mid-session, the watcher stops, emits a
+user-visible error naming the limit, and exits with code 2 rather than idling.
```

Open Questions: Is "latest-wins with cancellation" acceptable given scans are synchronous `rayon` work that cannot be cancelled mid-pass, or must the system finish the current scan and coalesce pending events instead?

#### Issue SA-4-12-20260928-2015

Title: [SA][WARNING] The `docs` audit has no failure, timeout, or partial-result path

Label: scenario, severity-warning

Location Path:
`crates/dispatcher/src/surface_docs_action.rs:20-24` · `crates/doc-rules/src/capabilities_doc_checker.rs`

Description: `collect_docs` returns `Err` for exactly one condition — the root path is not a directory. Everything after that is assumed infallible: the checker walks the workspace itself (it deliberately bypasses the file index), reads Markdown, and returns `DocResponse::Findings`. The specification does not say what happens when a document is unreadable (permissions), is not valid UTF-8, is a symlink pointing outside the workspace — a case the `filesystem` FRD explicitly guards for source files — or is large enough to matter. Because the group bypasses the `filesystem` crate, it also bypasses that crate's centralized I/O safety rules, which the PRD lists as a locked decision ("Filesystem I/O centralized in filesystem crate"). There is no `PARSE_WARN` equivalent for documents.

Acceptance Criteria: `doc-rules/FRD.md` documents the behavior for unreadable, non-UTF-8, symlinked-outside, and oversized documents; a run over a workspace containing each case completes with a diagnostic per skipped document and a non-crashing exit; the doc walk honors `ignored_paths` from config.

Recommendation: Route the document walk through `IFileSystemIOProtocol` so symlink and permission handling is inherited rather than reimplemented, and emit a `DOC_READ_WARN` diagnostic per skipped file, mirroring `PARSE_WARN`.

Git Diff:

```diff
--- a/crates/doc-rules/FRD.md
+++ b/crates/doc-rules/FRD.md
+## Edge Cases
+
+| Case | Behavior |
+| --- | --- |
+| Document unreadable (permissions) | Skip, emit `DOC_READ_WARN`, continue the walk |
+| Document is not valid UTF-8 | Skip, emit `DOC_READ_WARN`, continue the walk |
+| Document is a symlink leaving the workspace root | Skip, emit `DOC_READ_WARN`, never follow |
+| Path in `ignored_paths` | Not walked |
```

Open Questions: Should a skipped document count against the scan's exit code, or is a warning with exit 0 the right outcome?

### Scope 5: Trace the requirement to specification
<!-- Check requirement coverage, orphan requirement detection, ambiguous requirement flag, clarification request to Business Analyst, and cross-feature impact analysis -->

#### Issue SA-5-13-20260928-2015

Title: [SA][CRITICAL] Product docs specify 24 rules in 5 groups; the system ships 36 rules in 7 groups

Label: data, severity-critical

Location Path:
`PRD.md` → "AES Rule Summary (24 Rules)" · `README.md:5,50,52,105-109` · `RULES_AES.md` (root, 396 lines, 24 codes) · `.agents/rules/RULES_AES.md` (614 lines, 36 codes)

Description: The rule catalog exists twice and the two copies disagree. `.agents/rules/RULES_AES.md` defines 36 codes across 7 groups (adding Group 6 Document Invariants AES601–AES607 and Group 7 Folder Structure AES701–AES705) and carries a materially expanded AES403 (six sub-checks, HIGH/MEDIUM/LOW) and AES404. The root `RULES_AES.md` still defines 24 codes across 5 groups with the older AES403/AES404 text — and it is the copy that `PRD.md` and `README.md` link to. PRD success metric 2 is literally worded "24 AES rules enforced across 5 groups … all 24 rule codes produce violations", so the acceptance criterion for the product's second goal tests a subset of the shipped system and would pass while twelve rules go unverified. Cross-feature impact: `doc-rules` and `structure-rules` are unreferenced by the PRD feature list, `README.md`'s crate map omits both, and `AGENTS.md` role docs point agents at the stale file.

Acceptance Criteria: One rule catalog file exists (the other is a link or is deleted); `PRD.md` and `README.md` state 36 rules across 7 groups and list `doc-rules` and `structure-rules` in the crate map; PRD metric 2 reads "all 36 rule codes produce violations on `workspaces-bad`"; a `workspaces-bad` scan produces at least one finding for each of the 36 codes.

Recommendation: Make `.agents/rules/RULES_AES.md` the single source and reduce root `RULES_AES.md` to a pointer (or vice versa — but pick one), then update PRD, README, and the crate map in the same change. This is the highest-value fix in this plan: it is the root cause of SA-2-05, SA-3-08, and SA-5-14.

Git Diff:

```diff
--- a/PRD.md
+++ b/PRD.md
-| 2 | 24 AES rules enforced across 5 groups | `workspaces-bad` scan | all 24 rule codes produce violations |
+| 2 | 36 AES rules enforced across 7 groups | `workspaces-bad` scan | all 36 rule codes produce violations |
-## AES Rule Summary (24 Rules)
-Five groups: **Naming** (AES101–102, 2), **Import** (AES201–205, 5), **Quality** (AES301–305, 5), **Role** (AES401–406, 6), **Orphan** (AES501–506, 6). Full rule definitions: [RULES_AES.md](RULES_AES.md).
+## AES Rule Summary (36 Rules)
+Seven groups: **Naming** (AES101–102, 2), **Import** (AES201–205, 5), **Quality** (AES301–305, 5), **Role** (AES401–406, 6), **Orphan** (AES501–506, 6), **Document** (AES601–AES607, 7), **Structure** (AES701–AES705, 5). Full rule definitions: [RULES_AES.md](RULES_AES.md).
```

Open Questions: Which copy of `RULES_AES.md` is canonical — the root file that users read, or the `.agents/` file that agents read? (Clarification requested from the Business Analyst.)

#### Issue SA-5-14-20260928-2015

Title: [SA][CRITICAL] The repository violates its own AES607 in two crates, contradicting the self-audit goal

Label: data, severity-critical

Location Path:
`crates/doc-rules/FRD.md` (3 FRs) vs `crates/shared/src/doc_rules/contract_doc_protocol.rs` (2 protocol seams) · `crates/structure-rules/FRD.md` (3 FRs) vs `crates/shared/src/structure_rules/` (1 protocol seam)

Description: AES607 requires "an FRD's requirement count equals its feature's count of `I*Protocol` capability-seam classes". Counting FR rows against `pub trait I*Protocol` declarations across all documented crates gives parity everywhere except the two newest groups: `doc-rules` declares FR-DOC-001..003 against `IDocCheckerProtocol` + `IDocAuditProtocol` (3 vs 2), and `structure-rules` declares FR-STR-001..003 against `IStructureAuditProtocol` alone (3 vs 1). The rule introduced in commit `779480d` therefore fires on the very crate that implements it. PRD success metric 4 ("`check .` on this repo reports 0 violations") cannot hold — though note that because `check` does not run the doc group at all (SA-1-02), the violation is currently hidden rather than absent, which is the worse failure mode: the metric passes for the wrong reason.

Acceptance Criteria: `lint-arwaky-cli docs .` reports zero AES607 findings for this repository; each crate's FR count equals its protocol-seam count, either by splitting protocols or by merging requirement rows; a CI job runs the doc audit on the repo itself and fails on any finding.

Recommendation: For `structure-rules`, FR-STR-002 (path resolution) and FR-STR-003 (self-audit) are properties of FR-STR-001 rather than separate capabilities — fold them into a single requirement with acceptance notes. For `doc-rules`, FR-DOC-003 (per-document reportable findings) is the reporting seam; either give it a protocol or merge it into FR-DOC-001.

Git Diff:

```diff
--- a/crates/structure-rules/FRD.md
+++ b/crates/structure-rules/FRD.md
 | ID | Requirement |
 |----|-------------|
-| FR-STR-001 | `scan` and `check` report AES701–AES705 findings for the audit target. |
-| FR-STR-002 | Findings carry workspace-root-relative paths so a member-directory scan resolves them correctly. |
-| FR-STR-003 | `check .` reports 0 structure violations for this repository. |
+| FR-STR-001 | `scan` and `check` report AES701–AES705 findings for the audit target, each carrying a workspace-root-relative path that resolves from any member directory; this repository reports zero such findings. |
```

Open Questions: Does AES607 count a protocol trait or a protocol *file*? The commit message says "by class, never by file" — the FRD should restate that so the fix direction is unambiguous.

#### Issue SA-5-15-20260928-2015

Title: [SA][WARNING] Non-functional targets conflict between the PRD and the owning FRD

Label: non-functional, severity-warning

Location Path:
`PRD.md` → "Non-functional Requirements (High-level)" · `crates/filesystem/FRD.md:318,350`

Description: The PRD commits to "1,000 files < 5s; 10,000 files < 10s" and delegates the detail to `filesystem/FRD.md`, which instead states "1,000 files in under 2 s; 10,000 files in under 10 s" for pipeline throughput and "1,000 files parsed in parallel → completes in < 1s". Two of the three numbers disagree, and neither document says whether the budget covers the full scan (including external adapters, which the external-lint FRD allows up to 180 s for Clippy alone) or only the index build. The PRD's open-questions table still carries `v3.7.0` deadlines for items that the workspace has since moved past, and item 1 ("Is 10k-file scan < 10s achievable?") has no measurement harness referenced anywhere. A target that is stated twice with different numbers and no measurement boundary cannot be traced to a test.

Acceptance Criteria: One number per metric exists across PRD and FRD; each target names its measurement boundary (index-only vs full scan, external adapters included or excluded) and the hardware tier; a benchmark under `crates/filesystem/benches/` produces the figure the target is judged against.

Recommendation: Keep the PRD's numbers as the product commitment, restate the FRD's as the stricter internal budget explicitly labeled "index build only", and add the hardware tier to both.

Git Diff:

```diff
--- a/PRD.md
+++ b/PRD.md
-| Performance | 1,000 files < 5s; 10,000 files < 10s | `filesystem/FRD.md` |
+| Performance (full scan, AES groups only, external adapters excluded, CI runner tier) | 1,000 files < 5s; 10,000 files < 10s | `filesystem/FRD.md` |
--- a/crates/filesystem/FRD.md
+++ b/crates/filesystem/FRD.md
-| Pipeline throughput | 1,000 files in under 2 s; 10,000 files in under 10 s | Time a full build-index run over workspaces of each size |
+| Pipeline throughput (index build only; internal budget, stricter than the PRD commitment) | 1,000 files in under 2 s; 10,000 files in under 10 s | `bench_filesystem` over generated workspaces of each size |
```

Open Questions: What hardware tier defines "CI hardware" in PRD open question 1 — the current GitHub-hosted runner, or a pinned self-hosted spec?

## Violations

- **AES603 (Spec Purity)** — `crates/doc-rules/FRD.md` "API" table names concrete symbols (`RootDocRulesContainer::orchestrator()`, `DocAuditor::audit`), edging into implementation detail inside a specification document. Same pattern in `crates/structure-rules/FRD.md`. INFO.
- **AES604 (Crosslinks)** — `PRD.md` links `BACKLOG.md` at the repository root, which does not exist (only per-crate backlogs do). The crosslink is dangling. WARNING.
- **AES607 (FR/Protocol Parity)** — `crates/doc-rules` (3 FR / 2 seams) and `crates/structure-rules` (3 FR / 1 seam). CRITICAL — see SA-5-14.
- **PRD locked decision "Filesystem I/O centralized in filesystem crate"** — `doc-rules` walks the workspace itself and reads Markdown outside `IFileSystemIOProtocol`. WARNING — see SA-4-12.
- **PRD locked decision "Surface parity: MCP, CLI, and TUI expose the same commands"** — five actions missing from MCP. CRITICAL — see SA-3-07.

## Action Items

- [ ] **P0** SA-5-13 — Collapse the duplicated `RULES_AES.md` to one canonical file and correct PRD/README to 36 rules / 7 groups. Unblocks SA-2-05, SA-3-08, SA-5-14.
- [ ] **P0** SA-1-01 — Delete the duplicate CLI command enum; keep one authoritative `Commands` type in `shared`.
- [ ] **P0** SA-3-07 — Add the five missing MCP actions and a parity contract test derived from the command enum.
- [ ] **P0** SA-2-04 — Give `DocFinding` a line number and severity; map it to `ViolationItem`.
- [ ] **P0** SA-2-05 — Register `AES607` in `lint_arwaky.config.yaml` and add a registry-vs-config validation.
- [ ] **P0** SA-5-14 — Reconcile FR counts in `doc-rules` and `structure-rules` so the repo passes its own AES607.
- [ ] **P1** SA-1-02 — Fold the doc group into `ScanAggregates` so `check .` covers AES601–AES607.
- [ ] **P1** SA-4-10 — Record the exit-code-3 escalation rule in the PRD and align the implementation.
- [ ] **P1** SA-3-08 — Generate `COMMAND_CATALOG` from clap metadata or add a drift test.
- [ ] **P1** SA-2-06 — Return typed findings from `collect_docs`.
- [ ] **P2** SA-3-09 — Type the MCP request/response contract, add `schema_version` and `format` support.
- [ ] **P2** SA-4-11 — Specify watch concurrency and extend re-scan coverage to doc/structure.
- [ ] **P2** SA-4-12 — Specify doc-audit edge cases and route the walk through the filesystem crate.
- [ ] **P2** SA-1-03 — Correct the AES601–AES605 range comments and the doc-rules FRD scope.
- [ ] **P3** SA-5-15 — Reconcile the performance NFR numbers and name the measurement boundary.

## Fixed Document

### `PRD.md`

- Success metric 2 → "36 AES rules enforced across 7 groups … all 36 rule codes produce violations".
- "AES Rule Summary (24 Rules)" → "(36 Rules)" with the Document (AES601–AES607) and Structure (AES701–AES705) groups added.
- Feature Requirements → add "Document-chain audit (AES601–AES607) via `docs`" and "Folder-structure audit (AES701–AES705) via `scan`" under P0/P1 as appropriate.
- Exit Code Contract → append the escalation rule distinguishing aggregate commands from single-tool commands (SA-4-10).
- Non-functional Requirements → performance row gains its measurement boundary and hardware tier (SA-5-15).
- Reference section → remove or create the dangling root `BACKLOG.md` link.

### `README.md`

- Line 5 and the "AES Rules (24)" heading → 36 rules / 7 groups.
- Crate map → add `doc-rules/ # AES601–AES607` and `structure-rules/ # AES701–AES705`.

### `RULES_AES.md` (root) and `.agents/rules/RULES_AES.md`

- Keep one canonical catalog; replace the other with a one-line pointer. The surviving file must carry the expanded AES403 six-sub-check table and AES404 text.

### `lint_arwaky.config.yaml`

- Add the `AES607` rule entry under the AES600 block (SA-2-05).

### `crates/doc-rules/FRD.md`

- Requirements table → one row per shipped behavior, count reconciled with the protocol-seam count (SA-5-14).
- Add an "Edge Cases" table for unreadable / non-UTF-8 / symlinked / ignored documents (SA-4-12).
- Integration section → state whether `docs` is folded into `check` (SA-1-02) and whether an incremental verb exists (SA-1-03).

### `crates/structure-rules/FRD.md`

- Merge FR-STR-002 and FR-STR-003 into FR-STR-001 (SA-5-14).

### `crates/file-watch/FRD.md`

- Add the in-flight-scan concurrency rule and the watcher-failure fallback (SA-4-11).

### `crates/filesystem/FRD.md`

- Label the throughput row as the index-only internal budget (SA-5-15).

## Severity

| Level | Meaning |
| --- | --- |
| CRITICAL | Missing feature behavior, broken data model, incomplete API contract, or uncovered business requirement. Immediate fix. |
| WARNING | Ambiguous technical spec, missing edge case, unclear error contract, or weak traceability. Fix this cycle. |
| INFO | Specification refinement, optimization, or documentation improvement. Deferrable. |
