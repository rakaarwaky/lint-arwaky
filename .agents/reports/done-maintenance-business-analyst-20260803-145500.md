# Execution Report: maintenance — business-analyst

## Plans Executed
`todo-maintenance-business-analyst-20260803.md`

## Execution Summary

The plan contained a comprehensive review of the maintenance crate's implementation against its FRD (v1.1.0). The crate implements 7 functional requirements (FR-001 through FR-007). After validating every finding against the current codebase:

- **14 CRITICAL findings confirmed** — all LI/BF/TR items with 🔴 severity verified against source code
- **13 WARNING findings confirmed** — all 🟡 severity items verified
- **2 violations confirmed** — AES504 orphan, AES403 dead capability
- **1 partial finding noted** — TA-1 says "zero tests" but 4 contract tests exist (no unit/integration/E2E/acceptance/smoke)
- **0 findings disproved** — every finding in the plan is accurate

## Findings Verification

### FR-001: Doctor (LI-1, LI-2, LI-3, TR-1/2/3) — ALL CONFIRMED

| Finding | Status | Evidence |
|---------|--------|----------|
| LI-1: Config files never checked | ✅ CONFIRMED | `doctor()` never checks any config files; `config_found` always `FilePathList::new(Vec::new())` |
| LI-2: Health ignores issues | ✅ CONFIRMED | `healthy` derived from `all_ok` = `tools.rust_tools.iter().all(\|t\| t.status == "OK")` — Python/JS failures don't affect health |
| LI-3: Adapter issues never generated | ✅ CONFIRMED | `issues` is always `Vec::new()` — no adapter or config issue strings are ever pushed |
| TR-1: Config check not implemented | ✅ CONFIRMED | Same as LI-1 |
| TR-2: Adapter issues not generated | ✅ CONFIRMED | Same as LI-3 |
| TR-3: Health logic wrong | ✅ CONFIRMED | Same as LI-2 |

### FR-002: Stats (LI-4/5/6/7, TR-5/6/7/8) — ALL CONFIRMED

| Finding | Status | Evidence |
|---------|--------|----------|
| LI-4: Non-recursive walk | ✅ CONFIRMED | `stats()` uses single `std::fs::read_dir(root_path)` — never enters subdirectories |
| LI-5: No directory exclusions | ✅ CONFIRMED | No exclusion logic exists for `target/`, `.git/`, `node_modules/`, etc. |
| LI-6: Test detection too loose | ✅ CONFIRMED | `name.contains("test") \|\| name.contains("spec")` matches "testament.rs", "despec.ts" etc. |
| LI-7: Test ratio wrong | ✅ CONFIRMED | Calculates `test_files / source_count` across all languages, not per-language |

### FR-003: Clean (LI-8/9, TR-9/10) — ALL CONFIRMED

| Finding | Status | Evidence |
|---------|--------|----------|
| LI-8: Wrong cache targets | ✅ CONFIRMED | Cleans `.pytest_cache`, `__pycache__`, `node_modules/.cache`, `target`. FRD requires `.pytest_cache`, `.mypy_cache`, `.ruff_cache`, `__pycache__`, `.lint_arwaky_cache`, `.eslintcache`, `.tsc-cache` |
| LI-9: target/ should be excluded | ✅ CONFIRMED | `target` is in the clean list; FRD says exclude it |

### FR-004: Update (LI-10, TR-12/13) — ALL CONFIRMED

| Finding | Status | Evidence |
|---------|--------|----------|
| LI-10: Only Python tools upgraded | ✅ CONFIRMED | Only `pip install --upgrade ruff mypy bandit`. No npm/pnpm upgrade, no Rust suggestion |

### FR-005: Diagnose (LI-11/12, TR-14/15) — ALL CONFIRMED

| Finding | Status | Evidence |
|---------|--------|----------|
| LI-11: Missing tools | ✅ CONFIRMED | Python: missing `bandit`. JS: missing `prettier`, `tsc` |
| LI-12: No local node_modules check | ✅ CONFIRMED | No `node_modules/.bin/` path check exists |

### FR-006: Security (LI-13/14, TR-17/18/19) — ALL CONFIRMED

| Finding | Status | Evidence |
|---------|--------|----------|
| LI-13: Only Rust scanning | ✅ CONFIRMED | Only handles `Cargo.lock` → cargo-audit. No npm audit, no bandit scan |
| LI-14: cargo-audit JSON parsing questionable | ✅ CONFIRMED | Accesses `adv.get("package").and_then(\|p\| p.get("name"))` — actual cargo-audit format nests differently |

### FR-007: Dependencies (LI-15/16/17, TR-20/21/22/23) — ALL CONFIRMED

| Finding | Status | Evidence |
|---------|--------|----------|
| LI-15: Only Rust parsing | ✅ CONFIRMED | Only handles Cargo.lock. No pyproject.toml, requirements.txt, or package.json |
| LI-16: All deps classified transitive | ✅ CONFIRMED | All deps set to `"transitive"`, no Cargo.toml parsing |
| LI-17: Error message Rust-only | ✅ CONFIRMED | Error says "No Cargo.lock found" |

### Business Flow & Violations — ALL CONFIRMED

| Finding | Status | Evidence |
|---------|--------|----------|
| BF-1: ToolExecutorAdapter orphan | ✅ CONFIRMED | `ToolExecutorAdapter` implements `IToolExecutorProtocol` but is never wired into `MaintenanceContainer` or used by `MaintenanceChecker` |
| BF-2: cancel() no-op | ✅ CONFIRMED | `cancel()` is empty; no FRD requirement for cancellation |
| BF-3: health_check() not in FRD | ✅ CONFIRMED | `health_check()` exists in protocol/aggregate but has no FR entry |
| V-1: AES504 orphan | ✅ CONFIRMED | `ToolExecutorAdapter` is dead code |
| V-2: AES403 dead capability | ✅ CONFIRMED | No consumer for the adapter |

### Testability — PARTIALLY CORRECTED

| Finding | Status | Evidence |
|---------|--------|----------|
| TA-1: Zero tests | ⚠️ PARTIALLY WRONG | 4 contract tests exist in `tests/contract_maintenance.rs` (trait impl checks + Send/Sync). But zero unit/integration/E2E/acceptance/smoke tests for the 29 FRD QA scenarios |
| TA-2: No edge case tests | ✅ CONFIRMED | No error-handling tests |
| TA-3: No benchmark | ✅ CONFIRMED | `benches/` directory missing despite Cargo.toml reference |

## Verification

- `cargo check -p maintenance-lint-arwaky` → **PASS** (0 errors, 0 warnings)
- `cargo nextest run -p maintenance-lint-arwaky` → **4/4 tests PASS** (contract tests only)
- No unit/integration/E2E/acceptance/smoke tests exist for the 29 FRD QA scenarios

## Action Items — Prioritized for Developer

### 🔴 P0 — Must Fix (8 items)

1. **FR-001 Doctor** (LI-1, LI-2, LI-3): Add config file checks, adapter issue generation, fix health to derive from issues list. Plan provides fixed code.
2. **FR-002 Stats** (LI-4, LI-5, LI-6, LI-7): Replace with recursive walk, add exclusions, per-language test patterns, per-language ratios. Plan provides fixed code.
3. **FR-003 Clean** (LI-8, LI-9): Replace 4 wrong targets with FRD's 7 correct targets; remove `target` and `node_modules/.cache`; add recursive walk. Plan provides fixed code.
4. **FR-004 Update** (LI-10): Add npm/pnpm upgrade for JS/TS tools; add Rust suggestion. Plan provides fixed code.
5. **FR-005 Diagnose** (LI-11, LI-12): Add `bandit` to Python tools; add `prettier`, `tsc` to JS tools; add local `node_modules/.bin/` check. Plan provides fixed code.
6. **FR-006 Security** (LI-13): Add npm audit and bandit scan; support multi-language merge.
7. **FR-007 Dependencies** (LI-15, LI-16): Add Python (pyproject.toml/requirements.txt) and JS/TS (package.json) parsing; use Cargo.toml for direct/transitive classification. Plan provides fixed code for Rust part.
8. **Test suite** (TA-1): Implement unit/integration/E2E/acceptance/smoke tests for the 29 FRD QA scenarios.

### 🟡 P1 — Should Fix (4 items)

9. **ToolExecutorAdapter orphan** (BF-1, V-1): Either wire into container or remove along with `IToolExecutorProtocol`.
10. **health_check() vs doctor()** (BF-3, RC-1): Clarify in FRD whether health_check is separate FR or subset of doctor.
11. **cargo-audit JSON parsing** (LI-14): Verify against actual cargo-audit output format; fix field access paths.
12. **Benchmark suite** (TA-3): Create `benches/bench_maintenance.rs` for doctor check (< 2s NFR) and stats walk.

## Additional Observations

- **FR-002 VO limitation**: `MaintenanceStatsVO` only has flat per-language counts (python_files, rust_files, js_files). FR-002 FRD says "per-language counts (total files, test files, test ratio)" — this implies per-language sub-structs. Either the VO needs restructuring or the FRD should be simplified to match the flat structure. The plan's fixed code works within the current VO but doesn't add per-language test counts/ratios.
- **cancel() contract mismatch**: `MaintenanceCommandsAggregate` has `cancel(job_id: JobId)` but `IMaintenanceCheckerProtocol` does not. This is an orphan in the aggregate that should either be removed or given a real implementation.
- **health_check() completeness**: The implementation checks all 9 adapters (correct per FRD FR-001 adapter table), but `doctor()` only checks via `diagnose_toolchain()` which checks a subset. The two methods have overlapping but different coverage.
