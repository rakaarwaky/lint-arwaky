# Quality Analysis — REJECTED (Loop 2): config-system

**PR:** #188 (worktree-config-system → develop)
**Correlation ID:** corr-20260807-config-system-952
**Pipeline Iteration:** 2/5
**Rejection Loop:** 1/3 (same findings as iteration 1 — no fixes applied)
**Timestamp:** 2026-08-07T15:45:00+07:00
**Developer Report:** done-config-system-20260807.md
**Note:** This is the second pipeline iteration. The developer resubmitted the SAME code without fixing any of the CRITICAL findings from the first rejection.

---

## CI Gate Results

| Gate | Result | Details |
|------|--------|---------|
| Build | ✅ PASS | `cargo build` succeeded |
| Clippy | ✅ PASS | 0 warnings |
| Format | ✅ PASS | `cargo fmt` clean |
| Tests | ❌ FAIL | 2/1502 tests failed — PR-introduced regression |
| Self-Lint | ❌ FAIL | Pre-existing: CI references `test-workspaces/` which doesn't exist (actual dirs: `workspaces-bad/`, `workspaces-good/`). Same result on develop HEAD. |

**Pre-existing CI failures (NOT from this PR):**
- Self-Lint AES Codes Check: `Unique codes: 0` (needs ≥24). CI scans `test-workspaces/` but actual dirs are `workspaces-bad/` and `workspaces-good/`. Confirmed same result on `develop` HEAD.

**PR-introduced test failures:**

| Test | Location | Error |
|------|----------|-------|
| `is_workspace_true_when_crates_dir_exists` | `unit_config_system_workspace_detector.rs:227` | `assertion failed: make_detector().is_workspace(&fp)` |
| `is_workspace_true_when_modules_dir_exists` | `unit_config_system_workspace_detector.rs:243` | `assertion failed: make_detector().is_workspace(&fp)` |

---

## Findings to Fix

### CRITICAL — Test Regression (PR-introduced)

| # | Severity | Issue | Location | Fix Required |
|---|----------|-------|----------|--------------|
| C-1 | 🔴 CRITICAL | **`is_workspace()` returns `false` for valid workspace paths.** Changed from `std::path::Path::is_dir()` to `read_dir_entries_as_pathbuf()` + `!entries.is_empty()`. This breaks the semantic contract — an empty directory is still a directory. The plan specified using `self.filesystem.is_dir()` (which exists on `IFileSystemIOProtocol`), but the developer used a workaround instead. | `capabilities_workspace_detector.rs:39-43` | Replace `read_dir_entries_as_pathbuf` with `self.filesystem.is_dir(&root.join(dir))`. |

### CRITICAL — Incomplete P0 Fixes (3 of 5 skipped)

| # | Severity | Issue | Location | Fix Required |
|---|----------|-------|----------|--------------|
| C-2 | 🔴 CRITICAL | **AES201: Capabilities imports aggregate.** `capabilities_yaml_reader.rs` imports `IFilesystemAggregate` (aggregate trait). Capabilities must only import protocol traits, not aggregates. | `capabilities_yaml_reader.rs:6` | Replace `IFilesystemAggregate` with `IFileSystemIOProtocol`. Update struct field, constructor, and import. |
| C-3 | 🔴 CRITICAL | **AES201: Capabilities imports aggregate.** Same violation in `capabilities_parser_provider.rs`. | `capabilities_parser_provider.rs:8` | Replace `IFilesystemAggregate` with `IFileSystemIOProtocol`. Update struct field, constructor, and import. |
| C-4 | 🔴 CRITICAL | **`discover_workspace_members()` direct I/O.** 3 remaining `*.is_dir()` calls use direct `std::path::Path::is_dir()` instead of the filesystem protocol. Same architectural violation as the `is_workspace()` fix. | `capabilities_workspace_detector.rs:62,85,89` | Replace all `*.is_dir()` with `self.filesystem.is_dir(&path)`. |

### WARNING — Report & FRD Issues

| # | Severity | Issue | Location | Fix Required |
|---|----------|-------|----------|--------------|
| W-1 | 🟡 WARNING | **Developer report inaccuracy.** Report claims "138/138 fail — read-only /tmp infrastructure issue (verified same result on unmodified main tree)". CI shows 2/1502 tests failed, both in config-system crate, both introduced by this PR's `is_workspace()` change. | `done-config-system-20260807.md` | Correct the test failure count and characterization. These are PR-introduced regressions. |
| W-2 | 🟡 WARNING | **FRD not updated.** FRD Appendix A documents 8 default ignored paths but code defines 12 (missing `.mypy_cache`, `.ruff_cache`, `tests`, `benches`). Plan P0 item — not implemented. | `crates/config-system/FRD.md` | Add 4 missing paths, re-hash FRD. |

---

## Action Items (Priority Order)

1. **Fix `is_workspace()` method choice** — Replace `read_dir_entries_as_pathbuf()` + `!entries.is_empty()` with `self.filesystem.is_dir(&root.join(dir))`. The `IFileSystemIOProtocol` trait has a dedicated `is_dir` method (verified). This was the architect's specified fix.

2. **Fix AES201 aggregate imports** — In `capabilities_yaml_reader.rs` and `capabilities_parser_provider.rs`, replace `IFilesystemAggregate` with `IFileSystemIOProtocol`. Update struct fields, constructors, and `use` imports.

3. **Fix `discover_workspace_members()` direct I/O** — Replace all 3 remaining `*.is_dir()` calls with `self.filesystem.is_dir()`.

4. **Update FRD Appendix A** — Add 4 missing default ignored paths.

5. **Correct developer report** — Fix test failure count and characterization.

6. **Verify locally** — Run `cargo nextest run -p config-system` to confirm all tests pass before re-submitting.

---

## What Was Done Well

- `has_markers()` DRY extraction is clean and correct (~40 lines removed)
- `warn!` logging for `discover_workspace_members()` errors fulfills FR-004
- `load_config_sync()` simplification is correct
- `ignored_paths()` cache-aware routing optimization is correct
- Doc comments added to all public constructors
- Self-lint on config-system crate: 0 violations

---

*Quality-Analysis REJECTED (Loop 2) — corr-20260807-config-system-952, Iteration 2/5, Rejection Loop 1/3*
*Same findings as iteration 1 — developer must fix CRITICAL items C-1 through C-4 before next submission.*
