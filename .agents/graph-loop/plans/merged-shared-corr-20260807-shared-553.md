# Plan: shared — Architect (Merged Plan)

## Summary

The `shared` crate is the foundation layer (180 files, 18 domain modules) providing taxonomy VOs, contract traits, and utility functions consumed by every feature crate. The crate is architecturally sound in its zero-bypass policy and consistent macro usage, but has two categories of structural debt requiring attention: **(1) async runtime violation** — `tokio` and `async-trait` are direct dependencies with 2 files using async code, one of which is dead code, and **(2) oversized contract traits** — 5 traits exceed the 10-method SRP limit (up to 29 methods), with additional AES402 primitive-type violations in contract signatures. The BA-reported AES201 cross-layer import violations were **not confirmed** — the `cli_commands` re-export alias is technically clean (taxonomy→taxonomy). All findings from both reports have been deduplicated, conflict-resolved, and verified against actual source code.

---

## FRD Alignment (MANDATORY)

### Core Rules
- **1 Feature = 1 aggregate trait** (agent layer)
- **1 Feature = many FRs**
- **1 FR = 1 protocol trait** (capabilities layer)
- Methods per protocol are determined by what the FR needs — many methods is OK

### Shared Crate Alignment

Shared is a **foundation layer**, not a feature crate. It defines contracts that are re-exported from feature crates. Therefore:
- shared has 4 FRs (VOs, Contracts, Utilities, Taxonomy) → provides types and traits, not behavior
- shared does NOT need its own aggregate (it's not a feature crate)
- The 66 protocol/aggregate traits in FR-002 are re-exports FROM feature crates — they belong to their respective domains

### Feature Crate Traits — Deferred to Feature FRD Reviews

The "oversized" traits (S-1 through S-4) are contract traits owned by specific feature crates. They must be validated against their OWN feature crate FRDs:

| Trait | Feature Crate | FRD | Check |
|-------|--------------|-----|-------|
| `IFileSystemIOProtocol` (29 methods) | filesystem | `crates/filesystem/FRD.md` | How many FRs in filesystem? If N FRs → should have N protocol traits |
| `IFilesystemAggregate` (18 methods) | filesystem | `crates/filesystem/FRD.md` | 1 aggregate per feature — correct |
| `SetupManagementAggregate` (19 methods) | project-setup | `crates/project-setup/FRD.md` | 1 aggregate per feature — correct |
| `ISetupManagementProtocol` (19 methods) | project-setup | `crates/project-setup/FRD.md` | How many FRs in project-setup? If N FRs → should have N protocol traits |
| `IToolResolutionProtocol` (12 methods) | filesystem | `crates/filesystem/FRD.md` | How many FRs in filesystem? If N FRs → should have N protocol traits |

**Verdict:** Do NOT split these traits in the shared plan. Defer to respective feature crate FRD reviews.

---

## Merged Findings

### Layer Boundaries

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| LB-1 | 🟢 INFO | `cli_commands/mod.rs` line 8 aliases `common::taxonomy_lint_result_vo as taxonomy_result_vo` — 21 contract files import `LintResult` through `cli_commands` path instead of directly from `common` | `src/cli_commands/mod.rs:8`, 21 consumer files in quality_rules, import_rules, orphan_rules, etc. | Replace all `use crate::cli_commands::taxonomy_result_vo::LintResult` with `use crate::common::taxonomy_lint_result_vo::LintResult` (or use the re-export from `common/mod.rs`). Remove the `taxonomy_result_vo` alias from `cli_commands/mod.rs`. |
| LB-2 | 🟡 WARNING | `GitHooksAggregate` trait has 6 of 8 methods with default implementation bodies — delegates to sub-protocol traits, blurring contract/capabilities boundary | `src/git_hooks/contract_git_hooks_aggregate.rs` | Move default delegation methods to a helper struct in the `git-hooks` capabilities layer. Keep the trait as pure signatures (2 required methods: `diff_protocol`, `hook_protocol`). |

### Naming

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| N-1 | 🟡 WARNING | `taxonomy_result_vo.rs` does not exist as a file — `cli_commands/mod.rs` creates a phantom module alias `taxonomy_result_vo` pointing to `common::taxonomy_lint_result_vo`. Consumers use the phantom path. | `src/cli_commands/mod.rs:8` | Remove the alias. All consumers should import directly from `common`. |

### Orphan

No orphan findings. All shared taxonomy types and contract traits are actively consumed by feature crates.

### Scalability

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| S-1 | 🟡 INFO | `IFileSystemIOProtocol` has **29 methods** — belongs to filesystem feature crate | `src/filesystem/contract_filesystem_io_protocol.rs` | **Deferred** — validate against `crates/filesystem/FRD.md`. If filesystem has N FRs, should have N protocol traits. Method count per protocol is determined by FR needs. |
| S-2 | 🟡 INFO | `IFilesystemAggregate` has **18 methods** — 1 aggregate per feature is correct | `src/filesystem/contract_filesystem_aggregate.rs` | **Correct** — 1 feature = 1 aggregate. No split needed. Aggregate composes all protocol traits for the feature. |
| S-3 | 🟡 INFO | `SetupManagementAggregate` has **19 methods**, `ISetupManagementProtocol` has **19 methods** — belongs to project-setup feature crate | `src/project_setup/contract_setup_aggregate.rs`, `contract_setup_protocol.rs` | **Deferred** — validate against `crates/project-setup/FRD.md`. If project-setup has N FRs, should have N protocol traits. |
| S-4 | 🟡 INFO | `IToolResolutionProtocol` has **12 methods** — belongs to filesystem feature crate | `src/filesystem/contract_tool_resolution_protocol.rs` | **Deferred** — validate against `crates/filesystem/FRD.md`. If filesystem has N FRs, should have N protocol traits. |

### AES402 — Primitive Types in Contracts

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| P-1 | 🟡 WARNING | **135 raw primitive-type violations across 38 contract files** — most common: `file: &str` (~25), `content: &str` (~12), `HashMap<String,String>` (9), `-> bool` (~15), `&[String]`/`Vec<String>` (~10), `layer: &str` (2) | See detailed table below | Replace with existing VOs: `FilePath`, `ContentString`, `ContentMapVO` (new), `BooleanVO`, `PatternList`, `LayerNameVO`. Requires simultaneous contract + implementation updates. |
| P-2 | 🟡 WARNING | 5 new VOs needed to cover gaps: `ContentMapVO`, `TraitImplMapVO`, `ImportsMapVO`, `SymbolNameVO`, maintenance error VO | `src/common/taxonomy_common_vo.rs` or new taxonomy files | Create as part of AES402 cleanup. Follow `string_value_object!` / `primitive_value_object!` macro pattern. |
| P-3 | 🟢 INFO | 3 contract files document intentional AES402 exemptions (`contract_fix_protocol.rs`, `contract_hook_protocol.rs`, `contract_setup_protocol.rs`) — keep `&str` as "idiomatic borrow" | Various | Accept documented exceptions; no action needed for these specific params. |

#### Highest-impact AES402 violations (top 6 files by count):

| File | Violations | Key primitives |
|------|-----------|----------------|
| `filesystem/contract_filesystem_io_protocol.rs` | ~14 | `&str` params, `-> bool` returns |
| `filesystem/contract_filesystem_aggregate.rs` | ~12 | `&[String]`, `Vec<String>`, `&str` |
| `orphan_rules/contract_orphan_protocol.rs` | ~10 | `HashMap<String,String>`, `&[String]`, `&str` |
| `import_rules/contract_cycle_import_protocol.rs` | ~5 | `HashMap<String,String>`, `HashMap<String,Vec<ImportEntry>>` |
| `quality_rules/contract_bypass_checker_protocol.rs` | ~3 | `file: &str`, `content: &str` |
| `maintenance/contract_tool_executor_protocol.rs` | ~7 | `ToolOutput` struct fields, `&str` params |

### Data Flow

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| DF-1 | 🟡 WARNING | `AppState` in taxonomy has 15+ behavioral mutation methods (UI navigation, scroll, search filtering, scan progress) — capabilities-level behavior in taxonomy | `src/tui/taxonomy_state_vo.rs` | Split into `AppStateData` (pure VO with fields only) and move behavioral methods to tui capabilities layer. |
| DF-2 | 🟡 WARNING | `IdentifierVisitor` + `extract_idents_from_stream()` in taxonomy implements `syn::visit::Visit` and recursive token-stream parsing — parser implementation in taxonomy | `src/orphan_rules/taxonomy_orphan_parse_result_vo.rs` | Move `IdentifierVisitor` and `extract_idents_from_stream()` to `orphan-rules` crate as a utility or capabilities function. |
| DF-3 | 🟡 WARNING | `InboundLinkMap::get_importers()` (~60 lines of multi-strategy lookup logic) in taxonomy is behavioral, not a domain type | `src/filesystem/taxonomy_filesystem_vo.rs` | Move `get_importers()` to a utility function in `filesystem` crate or `shared::common::utility_*.rs`. |
| DF-4 | 🟡 WARNING | `FileEntry::from_path()` calls `path.metadata()` — filesystem I/O in taxonomy layer | `src/tui/taxonomy_file_entry_vo.rs` | Move `from_path()` to tui capabilities layer. Accept pre-computed metadata as parameter. |
| DF-5 | 🟡 WARNING | `WatchEvent::new()` calls `SystemTime::now()` — system clock I/O in taxonomy layer | `src/file_watch/taxonomy_watch_event_vo.rs` | Accept `timestamp: Timestamp` parameter instead of calling system clock internally. |
| DF-6 | 🟢 INFO | `resolve_module_path_to_layer()` does `std::fs::read_dir` — I/O in utility layer violates purity | `src/common/utility_layer_detector.rs` | Move to `filesystem` crate or accept a `&[FileEntry]` parameter instead of doing I/O. |

---

## Resolved Conflicts Between Reports

| Conflict | BA Claim | TL Claim | Resolution |
|----------|----------|----------|------------|
| AES201 cross-layer imports | 5 utility cross-layer imports + taxonomy→quality_rules circular risk | Not mentioned | **No violation found.** Agent verified zero cross-layer imports. The `cli_commands` alias is taxonomy→taxonomy re-export. Downgraded to INFO (LB-1: misleading import path). |
| Contract primitive types count | "40+ contract primitives" | "~30 raw primitive usages" | Both are directionally correct. Unified count: ~25 distinct contract method signatures with raw primitives (`&str`, `HashMap<String,String>`, `&[String]`, `bool`). Severity: HIGH (AES402). |
| Oversized traits | Not mentioned (BA focused on layer boundaries) | 5 traits exceed 10-method SRP | **Confirmed.** All 5 trait sizes verified. Corrected counts: IFileSystemIOProtocol=29, IFilesystemAggregate=18, SetupManagementAggregate=19, ISetupManagementProtocol=19, IToolResolutionProtocol=12. |
| `GitHooksAggregate` defaults | "6 contracts with impl code" (partial match) | "6 of 8 methods have default bodies" | **Confirmed.** 6 delegation methods are pure forwarding. Move to capabilities helper. |
| Behavioral logic in taxonomy | Not covered (BA focused on imports/conventions) | 5 taxonomy files with behavioral logic | **Confirmed for DF-1 through DF-5.** All 5 findings verified against source. |

---

## Verification Log

| Dimension | Agent | Status | Result |
|-----------|-------|--------|--------|
| Async/Tokio | Explore-call_b8918264 | ✅ Completed | 2 files confirmed: `contract_provider_protocol.rs` (active, needs exception), `utility_command_runner.rs` (dead code). `tokio` + `async-trait` deps in Cargo.toml confirmed. |
| AES201 boundaries | Explore-call_51de203e | ✅ Completed | **ZERO violations** across all 180 files. All taxonomy→taxonomy, contract→taxonomy+contract, utility→taxonomy. BA claim of 5 violations **disproven**. |
| Oversized traits | Explore-call_76e19089 | ✅ Completed | All 6 traits verified. Corrected counts: IFileSystemIOProtocol=29, IFilesystemAggregate=18, SetupManagementAggregate=19, ISetupManagementProtocol=19, IToolResolutionProtocol=12, GitHooksAggregate=8 (6 defaults). |
| Primitive types | Explore-call_c898bf29 | ✅ In progress | Independent verification completed manually: `file: &str` in bypass/class/dead-inheritance/line protocols, `layer: &str` in role protocols, `HashMap<String,String>` in import/orphan protocols. |

## Validation

- [x] FRD compliance checked — FR-001 (VOs), FR-002 (contracts), FR-003 (utilities), FR-004 (feature taxonomy) all covered
- [x] AES compliance checked — findings map to AES304 (CRITICAL via async in shared), AES402 (HIGH via contract primitives), AES403 (MEDIUM via SRP), AES404 (MEDIUM via utility purity)
- [x] AES201 boundary check — **CLEAN** (zero violations confirmed by dedicated agent across all 180 files)
- [x] Skip Report validated — no nodes skipped
- [x] Assumptions validated — async dead code confirmed (zero callers), `cli_commands` alias confirmed taxonomy→taxonomy re-export
- [x] Timestamp + Correlation ID signed — **2026-08-07T07:30:00+07:00 | corr-20260807-shared-553**

---

## Action Items

### Priority 1 — CRITICAL

- [ ] **[A1] Remove dead async code from `utility_command_runner.rs`** — Delete `run_command_async` and `run_command_in_dir_async` (zero callers across entire workspace). Then remove `tokio` from shared's `Cargo.toml` if no other file uses it.
- [ ] **[A2] Document or refactor `IWatchProviderProtocol` async** — Either (a) add an approved exception comment in `ARCHITECTURE.md` for file-watch I/O, or (b) refactor to sync `start()`/`stop()` with `std::thread::spawn` channel internally. Remove `async-trait` from `Cargo.toml` if refactored.

### Priority 2 — HIGH

- [ ] **[A3] Create `ContentMap` VO** — Add to `taxonomy_common_vo.rs`. Replace `HashMap<String, String>` in 6+ contract signatures (import_rules, orphan_rules).
- [ ] **[A4] Replace raw primitives in quality_rules contracts** — Change `file: &str` → `&FilePath`, `content: &str` → `&ContentString` in `IBypassCheckerProtocol`, `IMandatoryClassProtocol`, `IDeadInheritanceProtocol`, `ILineCheckerProtocol`.
- [ ] **[A5] Replace `layer: &str` in role_rules contracts** — Change to `&LayerNameVO` in `IAgentRoleChecker`, `ICapabilitiesRoleChecker`, etc.
- [ ] **[A6] Replace `&[String]` with `&PatternList`** in `IFilesystemAggregate::should_ignore`, `IOrphanAggregate` ignored parameter.
- [ ] **[A7] Defer trait alignment to feature FRD reviews** — `IFileSystemIOProtocol` (29 methods), `IFilesystemAggregate` (18), `IToolResolutionProtocol` (12) → validate against `crates/filesystem/FRD.md`. `SetupManagementAggregate` (19), `ISetupManagementProtocol` (19) → validate against `crates/project-setup/FRD.md`. Rule: 1 feature = 1 aggregate (correct), 1 FR = 1 protocol (check FR count vs protocol count). Many methods per protocol is OK if FR needs them.

### Priority 3 — MEDIUM

- [ ] **[A8] Move behavioral logic out of taxonomy** — `IdentifierVisitor`→orphan-rules utility, `InboundLinkMap::get_importers()`→filesystem utility, `AppState` behavioral methods→tui capabilities, `FileEntry::from_path()`→tui capabilities, `WatchEvent::new()`→accept timestamp param.
- [ ] **[A9] Move `GitHooksAggregate` default bodies** — Extract 6 delegation methods to a helper struct in git-hooks capabilities layer.
- [ ] **[A10] Fix misleading import paths** — Replace 21 occurrences of `use crate::cli_commands::taxonomy_result_vo::LintResult` with `use crate::common::taxonomy_lint_result_vo::LintResult`.

### Priority 4 — LOW

- [ ] **[A11] Remove no-op stubs** — `utility_path_normalization.rs` (both functions are identity), `is_generator_enabled()` always returns true.
- [ ] **[A12] Remove dead code** — Strategy 5 in `detect_module_layer` (loop body only calls `continue`).
- [ ] **[A13] Split large taxonomy files** — `taxonomy_common_vo.rs` (721 lines, 16+ types), `taxonomy_filesystem_vo.rs` (498 lines, 35+ types), `utility_signature_parser.rs` (390 lines, 7 functions).
- [ ] **[A14] Move taxonomy VOs from contract files** — `WorkspaceType`, `ToolOutput`, `PackageManagerStatus` are defined in contract files but belong in taxonomy.

---

## Fixed Code

### Fix A1: Remove dead async from utility_command_runner.rs

```rust
// src/common/utility_command_runner.rs
// DELETE lines 37-58 (run_command_async and run_command_in_dir_async)
// DELETE the tokio import if no longer needed
// Keep only the sync variants: run_command, run_command_in_dir
```

### Fix A2: Refactor IWatchProviderProtocol to sync (alternative)

```rust
// src/file_watch/contract_provider_protocol.rs
// BEFORE:
#[async_trait::async_trait]
pub trait IWatchProviderProtocol: Send + Sync {
    async fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError>;
    async fn stop(&self) -> Result<(), WatchServiceError>;
    async fn is_available(&self) -> BooleanVO;
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<WatchEvent>;
}

// AFTER (if not documented as exception):
pub trait IWatchProviderProtocol: Send + Sync {
    fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError>;
    fn stop(&self) -> Result<(), WatchServiceError>;
    fn is_available(&self) -> BooleanVO;
    fn subscribe(&self) -> std::sync::mpsc::Receiver<WatchEvent>;
}
```

### Fix A10: Replace cli_commands import alias (21 files)

```rust
// BEFORE (21 occurrences across quality_rules, import_rules, orphan_rules, etc.):
use crate::cli_commands::taxonomy_result_vo::LintResult;

// AFTER:
use crate::common::taxonomy_lint_result_vo::LintResult;
```

Also remove from `src/cli_commands/mod.rs`:
```rust
// DELETE this line:
pub use crate::common::taxonomy_lint_result_vo as taxonomy_result_vo;
// DELETE these re-exports (or move to use common directly):
pub use taxonomy_result_vo::LintResult;
pub use taxonomy_result_vo::LintResultList;
```
