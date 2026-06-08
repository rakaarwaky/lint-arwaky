# 📄 Feature Requirements Document (FRD)
**Feature Name:** Barrel Completeness Checker (AES012)
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
This document defines the AES012 rule that ensures every layer's barrel file (`mod.rs`, `__init__.py`, `index.ts`) properly re-exports all public modules. Barrel files without `pub use`/`__all__`/`export *` are flagged.

### 2.2 Scope
**In-Scope:**
- Detecting barrel files missing all-export patterns
- Rust: `pub use` in `mod.rs` or `lib.rs`
- Python: `__all__` in `__init__.py`
- JS/TS: `export *` or `export {` in `index.ts`/`index.js`
- MEDIUM severity violations

**Out-of-Scope:**
- Internal re-export rules (AES013 — separate FRD)
- Content validation of exports

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES012** | Rule code for barrel completeness violation |
| **check_barrel_completeness()** | Main detection method |
| **file_has_all_export()** | Content scan for all-export patterns |

## 3. Feature Overview
### 3.1 Background & Problem
Layer barrel files existed without exporting all modules — `mod.rs` declared modules but didn't re-export them, making internal types inaccessible from outside the layer.

### 3.2 Business Goals
- Every barrel file must explicitly export all public symbols
- Standardize export patterns per language
- Prevent inaccessible types within layers

### 3.3 Target Users
- **Developers**: Reminded to add `pub use`/`__all__` to barrel files
- **Architects**: Ensure layer API surface is complete

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want barrel files without all-exports to be flagged, so my module's types are accessible.

### 4.2 Use Cases & Workflow
**Detection:**
```
File: src-rust/layer-rules/mod.rs (barrel file)
  → Check for "pub use" in content
  → NOT found → AES012 MEDIUM violation

File: src-rust/di-containers/contract_service_aggregate.rs
  → Check for "pub use" in content
  → Found: pub use ... → No violation
```

**Language-specific patterns:**
| Language | Pattern | Example |
|----------|---------|---------|
| Rust | `pub use` | `pub use capabilities_import_checker::ArchImportRuleChecker;` |
| Python | `__all__` | `__all__ = ["ArchImportRuleChecker"]` |
| JS/TS | `export *` | `export * from './capabilities_import_checker'` |

### 4.3 Business Rules
- Severity: MEDIUM
- Only applies to barrel files (mod.rs/__init__.py/index.ts/index.js)
- Configurable via YAML `barrel_completeness` flag per layer

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |

## 6. UI/UX Requirements
```
AES012 MEDIUM - src-rust/layer-rules/mod.rs
  AES012 BARREL_COMPLETENESS: mod.rs is missing public modules or items exports.
  WHY? Layer boundaries must explicitly define their public API.
  FIX: Add explicit pub use declarations to mod.rs exposing public symbols.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Barrel file without all-export | `check_barrel_completeness()` runs | AES012 MEDIUM flagged | Pending Review |
| AC-002 | Barrel file with all-export | `check_barrel_completeness()` runs | No violation | Pending Review |
| AC-003 | Non-barrel file | `check_barrel_completeness()` runs | Skipped (not barrel) | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `check_barrel_completeness()` | `layer-rules/capabilities_internal_checker.rs:68` | 26 lines | Active — called via `check_internal_rules()` at line 146 for barrel files only |
| `file_has_all_export()` | `layer-rules/capabilities_internal_checker.rs:44` | 18 lines | Helper — checks `pub use` / `__all__` / `export *` |
| `check_internal_rules()` (dispatcher) | `layer-rules/capabilities_internal_checker.rs:133` | 22 lines | Entry point — delegates to AES012 for barrels, AES013 for non-barrels |
| YAML config | `lint_arwaky.config.rust.yaml` | — | `barrel_completeness: true` for all layers except root (line 167: `false`) |
| Test fixture | `test-project-rust/` | — | **None present** |

The implementation:
1. Only runs for barrel files (`mod.rs`, `__init__.py`, `index.ts`, `index.js`) — gated by `check_internal_rules()` at line 145
2. Checks `definition.barrel_completeness.value` config flag — line 75 (false for root layer)
3. Calls `file_has_all_export()` — detects:
   - Rust: `pub use `, `pub use{`, `pub use(` — line 48-50
   - Python: `__all__` (not in comments) — lines 52-55
   - JS/TS: `export *` — line 57
4. If no export found → AES012 MEDIUM (line 91)
5. Custom violation message from YAML used if configured — lines 79-88

### 8.2 Bugs Found

| # | Bug | Location | Impact | Fix |
|---|-----|----------|--------|-----|
| B1 | **`pub use` pattern misses multi-line and grouped exports** | `file_has_all_export()` lines 48-50 | Only checks `pub use ` (with trailing space), `pub use{`, `pub use(`. Misses `pub use { ... }` (space before `{`), `pub(crate) use`, and `pub(super) use` with visibility qualifiers. Misses `pub mod` re-exports (which also serve as barrel visibility). | Use regex `r"pub\s+(use|mod)\s"` for Rust export detection, or parse Rust module declarations. |
| B2 | **Zero line number** | `capabilities_internal_checker.rs:27` — `make_result` | AES012 violations report line 0. | Use `LineNumber::new(1)`. |
| B3 | **JS/TS detection is incomplete** | `file_has_all_export()` line 57 | Only checks `export *`. Does NOT check `export {` (named exports) or `export default`, which are valid barrel patterns in JS/TS. | Also check for `export {` and `export default`. |
| B4 | **Python `__all__` in comments** | Lines 54-55 | Checks `// __all__` and `# __all__` as commented-out markers but misses `#  __all__` (double space) or `//  __all__`. | Trim whitespace when checking comment exclusion. |
| B5 | **No AES012 test fixture** | `test-project-rust/` | No barrel file missing exports exists in the test project. | Create a `mod.rs` without `pub use` statements. |
| B6 | **FRD appendix path outdated** | `FRD_022.md:108` | References `layer-rules/capabilities_internal_checker.rs:53` — actual method is at line 68. | Update appendix path. |

### 8.3 What Needs to Be Added

1. **Robust Rust export detection** — handle `pub use { A, B }`, `pub(crate) use`, `pub mod` re-exports, and `pub use crate::...`.
2. **JS/TS `export {` detection** — named exports are a valid barrel pattern.
3. **Unit tests** — `file_has_all_export()` needs tests for all three languages:
   - Rust `pub use Foo::bar;` → true
   - Rust empty file → false
   - Python `__all__ = ["Foo"]` → true
   - Python `# __all__` (commented out) → false
   - JS `export * from './foo'` → true
   - JS `export { Foo }` → true (currently false — B3)
4. **Integration test fixture** — a `mod.rs` without exports.
5. **Config parity for Python/JS** — ensure Python and JS configs have `barrel_completeness` for applicable layers.

### 8.4 What to Keep

1. **Proper barrel-file gating** — AES012 only runs for barrel files, never for regular source files (line 145 check in `check_internal_rules`).
2. **Config flag** — root layer correctly exempted via `barrel_completeness: false`.
3. **Custom message support** — YAML-configured `barrel_completeness_violation_message` takes precedence over default.
4. **Comment-exclusion for Python** — `// __all__` and `# __all__` in comments are excluded from false-positive detection.
5. **Shared with AES013** — both rules share `file_has_all_export()` and `make_result()`, reducing code duplication.

### 8.5 Empirical Evidence from Test Projects

| Project | File | Expected | Actual (current) | Notes |
|---------|------|----------|------------------|-------|
| `test-project-rust` | — | AES012 MEDIUM | ❌ **No test fixture exists** | No barrel file without exports. |
| `self-lint` | `layer-rules/mod.rs` | No violation | ✅ | Contains `pub use` declarations. |
| `self-lint` | `naming-rules/mod.rs` | No violation | ✅ | Contains `pub mod` declarations. |
| `self-lint` | `di-containers/mod.rs` | No violation | ✅ | Contains `pub use` declarations. |
| `self-lint` | Root `src-rust/mod.rs` (if exists) | Skipped | ✅ | Root layer has `barrel_completeness: false`. |
| `self-lint` | Barrel with `pub use { Foo, Bar }` | No violation | ❌ **False positive** (B1) | Multi-line grouped `pub use { ... }` doesn't match `pub use ` or `pub use{`. |
| `self-lint` | Python `__init__.py` with `__all__` | No violation | ✅ | `__all__` detected correctly. |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `barrel_completeness` per layer | Missing for a layer = no check | Default enabled for all layers except root |

## 10. Appendices
- `src-rust/layer-rules/capabilities_internal_checker.rs:53` — `check_barrel_completeness()`
- `lint_arwaky.config.rust.yaml` — Per-layer `Taxonomy_Standards`, `Contract_Standards`, etc.
