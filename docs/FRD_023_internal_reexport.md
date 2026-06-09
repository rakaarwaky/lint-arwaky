# 📄 Feature Requirements Document (FRD)
**Feature Name:** Internal Re-export Forbidden Checker (AES013)
**Product:** Lint Arwaky v1.10.2
**Author:** Raka
**Date:** 09/06/2026  
**Version:** v1.1

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v1.1 | 09/06/2026 | Raka | Updated to prefix-based architecture: layers are filename prefixes, not directories; updated file paths for 26 feature folders | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES013 rule that forbids non-barrel sub-modules from having public re-export lists (`pub use`, `__all__`, `export *`). Only layer barrel files (mod.rs/__init__.py/index.ts) are allowed to define the layer's public API surface.

### 2.2 Scope
**In-Scope:**
- Detecting all-export patterns in non-barrel files
- Rust: `pub use` in non-mod.rs files
- Python: `__all__` in non-__init__.py files
- JS/TS: exempted (export is standard module practice)
- MEDIUM severity violations

**Out-of-Scope:**
- Barrel completeness rules (AES012 — separate FRD)
- Content validation beyond export patterns

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES013** | Rule code for internal re-export violation |
| **check_forbid_internal_all()** | Main detection method |
| **file_has_all_export()** | Content scan for export patterns |

## 3. Feature Overview
### 3.1 Background & Problem
Sub-modules contained `pub use` and re-export statements, bypassing the barrel's role as the single API surface. This made it unclear which types were part of the public API vs internal implementation details.

### 3.2 Business Goals
- Only barrel files define the layer's public API
- Prevent API surface fragmentation across sub-modules
- Centralize type visibility in mod.rs

### 3.3 Target Users
- **Developers**: Told when using pub use in sub-modules
- **Architects**: Centralize API surface in barrel files

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned if I use pub use in a non-barrel file, so I keep the API surface centralized.

### 4.2 Use Cases & Workflow
**Detection:**
```
File: src-rust/layer-rules/capabilities_import_checker.rs (non-barrel)
  → Contains "pub use" somewhere in the file
  → NOT barrel file → AES013 MEDIUM violation

File: src-rust/layer-rules/mod.rs (barrel)
  → Contains "pub use" (allowed)
  → IS barrel file → No violation
```

**JS/TS exemption:** `export` in JS/TS is standard module practice, not a barrel bypass — exempted from AES013.

### 4.3 Business Rules
- Severity: MEDIUM
- Applies to Rust and Python only (JS/TS exempted)
- Configurable via YAML `forbid_internal_all` flag per layer

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |

## 6. UI/UX Requirements
```
AES013 MEDIUM - src-rust/layer-rules/capabilities_import_checker.rs
  AES013 INTERNAL_ALL_FORBIDDEN: Public re-exports list detected in non-barrel sub-module.
  WHY? Only mod.rs barrel files should define the layer's public API surface.
  FIX: Remove pub use from this file and export via mod.rs instead.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Non-barrel file with pub use | `check_forbid_internal_all()` runs | AES013 MEDIUM flagged | Pending Review |
| AC-002 | Barrel file with pub use | `check_forbid_internal_all()` runs | No violation (barrel) | Pending Review |
| AC-003 | JS/TS file with export | `check_forbid_internal_all()` runs | Skipped (exempted) | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `check_forbid_internal_all()` | `layer-rules/capabilities_internal_checker.rs:98` | 33 lines | Active — called via `check_internal_rules()` at line 150 for non-barrel files |
| `file_has_all_export()` (shared) | `layer-rules/capabilities_internal_checker.rs:44` | 18 lines | Shared helper with AES012 — checks `pub use` / `__all__` / `export *` |
| `check_internal_rules()` (dispatcher) | `layer-rules/capabilities_internal_checker.rs:133` | 22 lines | Entry point — AES012 for barrels, AES013 for non-barrels |
| YAML config | `lint_arwaky.config.rust.yaml` | — | `forbid_internal_all: true` for all layers except root |
| JS/TS exemption | `capabilities_internal_checker.rs:108-113` | 6 lines | JS/TS files are always skipped |
| Test fixture | `test-project-rust/` | — | **None present** |

The implementation:
1. Only runs for non-barrel files — gated by `check_internal_rules()` at line 149 (note: barrel files handled at line 145-147, AES013 at line 150)
2. Checks `definition.forbid_internal_all.value` config flag — line 104
3. JS/TS exemption: if file ends with `.js`, `.ts`, `.jsx`, or `.tsx`, returns early — lines 108-113
4. Calls `file_has_all_export()` — same helper as AES012
5. If export found → AES013 MEDIUM (line 128)
6. Custom violation message from YAML used if configured — lines 116-127

### 8.2 Bugs Found

| # | Bug | Location | Impact | Fix |
|---|-----|----------|--------|-----|
| B1 | **Default violation message is Python-specific** | `check_forbid_internal_all()` line 126 | Default message says `"__all__ is forbidden in non-barrel files."` but AES013 also catches Rust `pub use` (via shared `file_has_all_export()`). A Rust developer seeing `__all__` will be confused — Rust doesn't use `__all__`. | Change message to: `"pub use / __all__ / export * is forbidden in non-barrel files. Only mod.rs barrel files should define the layer's public API surface."` Or use the YAML config message which is correct: `"AES013 INTERNAL_ALL_FORBIDDEN: Public re-exports list detected in a non-barrel sub-module."` |
| B2 | **Zero line number** | `capabilities_internal_checker.rs:27` — `make_result` | AES013 violations report line 0. | Use `LineNumber::new(1)`. |
| B3 | **No AES013 test fixture** | `test-project-rust/` | Unlike AES007, AES010, AES011 which have fixtures, AES013 has zero test coverage. | Add a non-barrel `.rs` file with `pub use` statement and expect AES013. |
| B4 | **FRD appendix path outdated** | `FRD_023.md:103` | References `layer-rules/capabilities_internal_checker.rs:83` — actual method is at line 98. | Update appendix path. |

### 8.3 What Needs to Be Added

1. **Better default message** — mention Rust `pub use` and JS `export *` alongside Python `__all__` in the fallback message.
2. **Unit tests** for `check_forbid_internal_all()`:
   - Non-barrel file with `pub use` → AES013 MEDIUM
   - Non-barrel file without `pub use` → no violation
   - Barrel file (shouldn't reach this method) — no violation
   - JS/TS file with `export *` → skipped (exemption)
   - Config flag `false` → skipped
   - Custom violation message from YAML
3. **Integration test fixture** — a non-barrel file with `pub use` in test-project-rust.
4. **Consider `pub mod` in non-barrel files** — `pub mod` also creates a public API surface. Should it also be flagged under AES013?

### 8.4 What to Keep

1. **JS/TS exemption** — correctly exempts JavaScript/TypeScript files where `export` is standard module practice, not a barrel bypass (lines 108-113).
2. **Shared helper with AES012** — `file_has_all_export()` is reused cleanly between both rules.
3. **Config-driven enforcement** — `forbid_internal_all` flag per layer allows selective disable (root layer has `false`).
4. **Custom message support** — YAML-configured `forbid_internal_all_violation_message` takes precedence over the Python-centric default.
5. **Correct gating** — only non-barrel files reach this check (barrel files are handled by AES012 at line 146).

### 8.5 Empirical Evidence from Test Projects

| Project | File | Expected | Actual (current) | Notes |
|---------|------|----------|------------------|-------|
| `test-project-rust` | — | AES013 MEDIUM | ❌ **No test fixture exists** | No non-barrel file with `pub use` exists. |
| `self-lint` | `layer-rules/capabilities_internal_checker.rs` | No violation | ✅ | This file has `pub` methods but no `pub use` re-exports. |
| `self-lint` | `layer-rules/capabilities_compliance_analyzer.rs` | No violation | ✅ | No `pub use` statements. |
| `self-lint` | `code-analysis/agent_checking_coordinator.rs` | No violation | ✅ | Uses `use crate::...` but no `pub use`. |
| `self-lint` | Any `.ts` file with `export` | Skipped | ✅ | JS/TS exemption at lines 108-113. |
| `self-lint` | Any `mod.rs` with `pub use` | Skipped (AES012, not AES013) | ✅ | Barrel files handled separately. |
| `self-lint` | Non-barrel Rust file with `pub use` | AES013 MEDIUM | ✅ Would fire but with `__all__` message (B1) | Misleading message for Rust developers. |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `forbid_internal_all` per layer | Missing for a layer = no check | Default enabled for all layers except root |

## 10. Appendices
- `src-rust/layer-rules/capabilities_internal_checker.rs:83` — `check_forbid_internal_all()`
- `lint_arwaky.config.rust.yaml` — Per-layer standards with `forbid_internal_all: true`
