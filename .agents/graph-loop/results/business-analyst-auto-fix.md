# Plan: auto-fix — Business-Analyst

## Summary

The auto-fix crate provides mechanical fixes (remove, replace, rename) for three AES violation codes: AES101 (naming), AES203 (unused imports), AES304 (bypass comments). The implementation is solid — clean 4-file architecture (processor, adapter, orchestrator, container), comprehensive reason-coded outcomes, and good test coverage (8 test files, 50+ test cases). However, there are **2 critical issues** in the FRD (dry-run return value contradiction, FRD file naming doesn't match PRD convention), **3 warnings** (dead `with_dry_run` constructor, missing idempotency test, FRD lists method signatures which violates project convention), and **2 info-level findings**. No AES violations in the code structure itself.

## Findings

### Requirements Clarity

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| RC-1 | 🔴 CRITICAL | **FR-003 dry-run return value contradicts FixOutcome type.** FR-003 says: "In dry-run mode, returns `Applied` with change count without modifying the file." But `FixOutcome::applied(changes)` requires a `changes: usize`. The code at `rename_symbol_impl` line ~188 correctly computes `change_count` before the dry-run check and passes it to `FixOutcome::applied(change_count)`. However, FR-001 dry-run says "returns `Applied` (would apply) without modifying the file" — implying change count is 0, while FR-003 says count is non-zero. The FRD is internally inconsistent about what dry-run `Applied` carries. | FRD.md FR-001 vs FR-003 | Align FR-001 to match FR-003: dry-run `Applied` carries the **would-apply change count**, not 0. Update FR-001 edge case to say "returns `Applied(1)` (would apply) without modifying the file." |
| RC-2 | 🟡 WARNING | **FRD contains explicit method signatures and file paths — violates project convention.** Memory `frd-conventions.md` states: "FRD should not contain explicit method signatures, parameter types, or file names." The current FRD references `IFixProtocol`, `LintFixOrchestratorAggregate`, `FileAdapter`, `AutoFixContainer` by name and describes their wiring. | FRD.md Integration Points section, API Contract table | Remove file names and trait names from FRD body. The API Contract table should use operation/input/output descriptions only. Integration Points should describe data flow, not concrete type names. |
| RC-3 | 🟡 WARNING | **FRD test scenarios don't fully match PRD acceptance test naming.** PRD states: "Acceptance tests standardized to `acceptance_FR_00N.rs`". Current test files use `acceptance_AES201_fix.rs` and `acceptance_AES304_fix.rs`. No `acceptance_FR_001.rs` through `acceptance_FR_005.rs` exist. | tests/ directory | Rename acceptance tests to `acceptance_FR_001.rs` (unused import), `acceptance_FR_002.rs` (bypass fix), `acceptance_FR_003.rs` (symbol rename), `acceptance_FR_004.rs` (dry-run), `acceptance_FR_005.rs` (non-fixable report). Add missing acceptance tests for FR-003, FR-004, FR-005. |

### Business Flow

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| BF-1 | 🟡 WARNING | **No idempotency enforcement — FRD NFR requires it.** FRD NFR states: "Running auto-fix repeatedly on the same file produces no further changes (`Skipped` after first `Applied`)." The code relies on the linter returning 0 violations on re-scan, which is correct in principle, but there's no explicit guard. If the linter is stale or returns false positives, the same fix could be applied twice (e.g., double-removing an import line that was already removed). | capabilities_fix_processor.rs execute() | Add an idempotency guard: after applying a fix, re-read the file and verify the target pattern no longer exists before counting it as `Applied`. Alternatively, document this as a known dependency on linter correctness (acceptable if verified by tests). |
| BF-2 | 🟢 INFO | **`with_dry_run` deprecated constructor still present.** The `#[deprecated]` annotation is correct, but it adds noise. Since this is a pre-1.0 crate (v1.11.0 but no external consumers documented), it could be removed entirely. | capabilities_fix_processor.rs line ~70 | Remove `with_dry_run` entirely — the only caller pattern is `new() + execute(path, dry_run)`. |

### Logic Implementation

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| LI-1 | 🟢 INFO | **AES101 rename logic has a subtle behavioral note.** The rename logic in `execute()` extracts old_name by finding the first word containing `_` and longer than 3 chars. If the message has multiple such words, it picks the first one. This works for typical linter messages but is fragile if message format changes. | capabilities_fix_processor.rs execute() ~line 97 | Acceptable — this is inherent to the mechanical rename approach. The FRD explicitly states rename is "mechanical" and "does not produce semantically correct snake_case names." No change needed, but worth noting. |
| LI-2 | 🟢 INFO | **Multi-line import detection heuristic is reasonable but not exhaustive.** The current detection handles: unclosed `{`, trailing comma, and previous-line continuation. It correctly skips Python `\` continuation (not implemented — acceptable for v1). The heuristic may miss edge cases like `use foo::{\n    bar, // comment\n}` but these are unlikely in practice. | capabilities_fix_processor.rs fix_unused_import_impl ~line 150 | Acceptable for v1. FRD states "Multi-line import blocks are not auto-fixed." The heuristic correctly errs on the side of skipping. |
| LI-3 | 🔴 CRITICAL | **No acceptance test for FR-003 (symbol rename) or FR-005 (non-fixable report).** The unit tests in `unit_auto_fix_fix_processor.rs` cover rename logic well, but there is no `acceptance_FR_003.rs` test that verifies the end-to-end rename pipeline through the orchestrator, and no `acceptance_FR_005.rs` that validates non-fixable violations are properly reported. PRD mandates `acceptance_FR_00N.rs` naming. | tests/ directory | Add `acceptance_FR_003.rs` (rename end-to-end) and `acceptance_FR_005.rs` (non-fixable report). |

### Testability & Acceptance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| TA-1 | 🟡 WARNING | **Missing acceptance tests for FR-004 (dry-run) and FR-005 (non-fixable).** While `e2e_auto_fix_flow.rs` has dry-run tests, there's no `acceptance_FR_004.rs` or `acceptance_FR_005.rs`. The PRD requires standardized naming. | tests/ directory | Create `acceptance_FR_004.rs` and `acceptance_FR_005.rs` per PRD convention. |
| TA-2 | 🟡 WARNING | **No idempotency test.** FRD NFR requires: "Running auto-fix repeatedly on the same file produces no further changes." This is not tested anywhere in the test suite. | tests/ directory | Add `acceptance_FR_004.rs` test case: apply fix, then re-run — verify second run produces no `Applied` outcomes. |

### Traceability (FRD → Code)

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| TR-1 | 🟢 INFO | **FR-001 → code traceability is complete.** `fix_unused_import_impl` handles all edge cases listed in FRD: file not found, line out of bounds, non-import line, multi-line import, read/write errors. | capabilities_fix_processor.rs | No action needed. |
| TR-2 | 🟢 INFO | **FR-002 → code traceability is complete.** `fix_bypass_comments_impl` handles all FRD patterns: `#[allow]` removed entirely, inline comments stripped (not deleted), `unwrap()` → `expect("safe")`, `panic!`/`todo!`/`unimplemented!`/`unreachable!` → `Skipped(unsafe_removal)`, `expect()` → `Skipped(already_has_context)`. | capabilities_fix_processor.rs | No action needed. |
| TR-3 | 🟢 INFO | **FR-003 → code traceability is complete.** `rename_symbol_impl` implements word-boundary replacement, keyword conflict detection, symbol-not-found detection, and dry-run with change count. | capabilities_fix_processor.rs | No action needed. |
| TR-4 | 🟢 INFO | **FR-004 → code traceability is complete.** `execute(path, dry_run)` parameter passes through to all individual fix methods. Dry-run results are identical to non-dry-run (Applied/Skipped/Failed with same reasons). | capabilities_fix_processor.rs execute() | No action needed. |
| TR-5 | 🟢 INFO | **FR-005 → code traceability is complete.** `report_non_fixable` filters by FIXABLE_CODES (exact equality). `manual_skipped` collects AES304 UnsafeRemoval and AlreadyHasContext results. Both are merged into the output. | capabilities_fix_processor.rs execute() | No action needed. |

## Violations

None. The auto-fix crate's code structure is AES-compliant:

| File | Layer | Naming | AES201 | Role |
|------|-------|--------|--------|------|
| `capabilities_fix_processor.rs` | Capabilities | ✅ `capabilities_` prefix | ✅ imports taxonomy + contract + utility only | ✅ implements IFixProtocol |
| `capabilities_file_adapter.rs` | Capabilities | ✅ `capabilities_` prefix | ✅ imports taxonomy + contract + filesystem | ✅ implements IFileAdapterProtocol |
| `agent_fix_orchestrator.rs` | Agent | ✅ `agent_` prefix + `_orchestrator` suffix | ✅ imports taxonomy + contract only | ✅ pure delegation |
| `root_auto_fix_container.rs` | Root | ✅ `root_` prefix + `_container` suffix | ✅ imports all layers (root permitted) | ✅ DI wiring only |

## Action Items

- [ ] **CRITICAL** RC-1: Fix FRD dry-run return value contradiction — FR-001 says change count 0, FR-003 says non-zero. Align to non-zero (matches actual code behavior).
- [ ] **CRITICAL** LI-3 + TA-1: Add `acceptance_FR_003.rs`, `acceptance_FR_004.rs`, `acceptance_FR_005.rs` tests per PRD naming convention.
- [ ] **WARNING** RC-2: Remove explicit trait/file names from FRD Integration Points and API Contract table.
- [ ] **WARNING** RC-3: Rename existing `acceptance_AES201_fix.rs` → `acceptance_FR_001.rs` and `acceptance_AES304_fix.rs` → `acceptance_FR_002.rs`.
- [ ] **WARNING** TA-2: Add idempotency test case (fix → re-run → no Applied outcomes).
- [ ] **WARNING** BF-1: Either add idempotency guard in code or document linter dependency in FRD NFR.
- [ ] **INFO** BF-2: Remove deprecated `with_dry_run` constructor (no external consumers).

## Fixed Code

No code changes required for this iteration — all findings are FRD/documentation level. The actual Rust implementation is correct and AES-compliant. Code fixes are deferred to the developer agent after this plan is approved.
