# TUI Crate Audit Report

**Date**: 2026-06-27
**Scope**: `crates/tui/src/` + `crates/tui/tests/`
**Baseline**: Self-lint (`lint-arwaky-cli check`) + Clippy + manual review

---

## Executive Summary

| Category | Status |
|----------|--------|
| Build | ✅ Clean (`cargo check`, `cargo test` 75/75 pass) |
| Clippy warnings | ⚠️ 17 warnings (1 lib, 16 tests) |
| Self-lint violations | ⚠️ 7 violations (2 files) |
| Dead code | ⚠️ 1 file orphaned, 1 struct unused |
| Import compliance | ⚠️ 2 AES201 violations in orphaned file |

---

## File-by-File Audit (15 src files)

### ✅ CLEAN (13 files)

| # | File | Layer | Status |
|---|------|-------|--------|
| 1 | `agent_tui_orchestrator.rs` | agent | ✅ Implements `ITuiAggregate` |
| 2 | `capabilities_action_handler.rs` | capabilities | ✅ Implements `IActionHandlerProtocol` |
| 3 | `capabilities_lint_executor.rs` | capabilities | ⚠️ 1 clippy warning + 1 AES403 |
| 4 | `infrastructure_file_system_adapter.rs` | infrastructure | ✅ Implements `IFileSystemPort` |
| 5 | `surface_tui_command.rs` | surface | ✅ Main loop + 3-panel layout |
| 6 | `surface_file_list_view.rs` | surface | ✅ Passive render |
| 7 | `surface_tree_view.rs` | surface | ✅ Passive render |
| 8 | `surface_preview_view.rs` | surface | ✅ Passive render |
| 9 | `surface_path_screen.rs` | surface | ✅ Passive render |
| 10 | `surface_help_screen.rs` | surface | ✅ Passive render |
| 11 | `surface_shortcut_component.rs` | surface | ✅ Passive render |
| 12 | `surface_status_component.rs` | surface | ✅ Passive render |
| 13 | `root_tui_container.rs` | root | ✅ DI wiring (now with `init_global_checker`) |

### ⚠️ ISSUES (2 files)

#### 1. `capabilities_watch_manager.rs` — ORPHANED, SHOULD BE REMOVED

**Problem**: File exists on disk but is NOT declared in `lib.rs`. No other file imports it.

**Self-lint violations**:
- `AES201 FORBIDDEN_IMPORT` — imports from `infrastructure` layer (forbidden for capabilities)
- `AES204 IMPORT_INTENT` — `Mutex` import used only in dummy functions
- `AES403 CAPABILITY_ROLE` — `WatchChange` struct has no trait implementation

**Recommendation**: DELETE this file. It's dead code implementing a feature (live file watch) that isn't wired into the TUI. If watch mode is needed later, it should be re-implemented with proper AES compliance.

#### 2. `capabilities_lint_executor.rs` — MINOR ISSUES

**Clippy warning** (line 432):
```rust
// Current:
.filter(|r| r.source.as_ref().map_or(false, |s| security_names.iter().any(|n| s.contains(n))))
// Should be:
.filter(|r| r.source.as_ref().is_some_and(|s| security_names.iter().any(|n| s.contains(n))))
```

**AES403 violation**:
- `AdapterInfo` struct (in `discover_adapters`) has no trait implementation
- This is a data-only struct used for display, not a capability. Consider moving to taxonomy or renaming to avoid the AES403 check.

---

## Test Audit (4 test files)

| # | File | Tests | Status |
|---|------|-------|--------|
| 1 | `lint_executor_tests.rs` | 32 | ⚠️ 9 clippy warnings |
| 2 | `config_show_tests.rs` | 4 | ✅ Clean |
| 3 | `adapter_discovery_tests.rs` | 5 | ⚠️ 7 unused imports |
| 4 | `capabilities_action_handler_tests.rs` | 32 | ✅ Clean |

### `lint_executor_tests.rs` Issues:
- **Unused variable** `files` (line 722)
- **Dead fields**: `success` field never read, `failure()` function never used
- **6x** `field assignment outside of initializer` — tests use `Default::default()` then set fields individually instead of struct literal

### `adapter_discovery_tests.rs` Issues:
- **7 unused imports** — leftover from when tests were more comprehensive

---

## Self-Lint Violations Summary

| Code | File | Issue | Severity |
|------|------|-------|----------|
| AES201 | `capabilities_watch_manager.rs` | FORBIDDEN_IMPORT (infra in capabilities) | HIGH |
| AES204 | `capabilities_watch_manager.rs` | IMPORT_INTENT (dummy Mutex import) | MEDIUM |
| AES403 | `capabilities_watch_manager.rs` | CAPABILITY_ROLE (WatchChange no trait) | HIGH |
| AES403 | `capabilities_lint_executor.rs` | CAPABILITY_ROLE (AdapterInfo no trait) | MEDIUM |

**Note**: 3 of 4 violations are in the orphaned `capabilities_watch_manager.rs`. Removing it eliminates 75% of violations.

---

## Recommended Actions (Priority Order)

| # | Action | Impact | Effort |
|---|--------|--------|--------|
| 1 | Delete `capabilities_watch_manager.rs` | Eliminates 3 violations + orphaned code | 2 min |
| 2 | Fix `map_or` → `is_some_and` in `capabilities_lint_executor.rs:432` | Eliminates 1 clippy warning | 1 min |
| 3 | Clean unused imports in `adapter_discovery_tests.rs` | Eliminates 7 clippy warnings | 2 min |
| 4 | Clean `lint_executor_tests.rs` warnings (unused vars, dead fields) | Eliminates 9 clippy warnings | 5 min |
| 5 | Address `AdapterInfo` AES403 — either implement trait or move to taxonomy | Eliminates 1 violation | 10 min |

**Total effort**: ~20 minutes to reach 0 violations + 0 clippy warnings.

---

## Architecture Compliance

| Layer | Files | Correct? |
|-------|-------|----------|
| Taxonomy | 0 in tui (5 in shared) | ✅ Correct location |
| Contract | 0 in tui (6 in shared) | ✅ Correct location |
| Capabilities | 2 (action_handler, lint_executor) + 1 orphan | ⚠️ Remove orphan |
| Infrastructure | 1 (file_system_adapter) | ✅ |
| Agent | 1 (tui_orchestrator) | ✅ |
| Surfaces | 8 (command + 7 views/components) | ✅ |
| Root | 1 (tui_container) | ✅ |

**Total**: 14 active files (15 on disk, 1 orphaned)
