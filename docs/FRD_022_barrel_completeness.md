# 📄 Feature Requirements Document (FRD)
**Feature Name:** Barrel Completeness Checker (AES012)
**Product:** Lint Arwaky v1.10.2
**Author:** Raka
**Date:** 08/06/2026
**Version:** v1.0

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |

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
File: src-rust/capabilities/mod.rs (barrel file)
  → Check for "pub use" in content
  → NOT found → AES012 MEDIUM violation

File: src-rust/contract/mod.rs (barrel file)
  → Check for "pub use" in content
  → Found: pub use ... → No violation
```

**Language-specific patterns:**
| Language | Pattern | Example |
|----------|---------|---------|
| Rust | `pub use` | `pub use architecture_import_checker::ArchImportRuleChecker;` |
| Python | `__all__` | `__all__ = ["ArchImportRuleChecker"]` |
| JS/TS | `export *` | `export * from './architecture_import_checker'` |

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
AES012 MEDIUM - src-rust/capabilities/mod.rs
  AES012 BARREL_COMPLETENESS: mod.rs is missing public modules or items exports.
  WHY? Layer boundaries must explicitly define their public API.
  FIX: Add explicit pub use declarations to mod.rs exposing public symbols.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Barrel file without all-export | `check_barrel_completeness()` runs | AES012 MEDIUM flagged | ✅ |
| AC-002 | Barrel file with all-export | `check_barrel_completeness()` runs | No violation | ✅ |
| AC-003 | Non-barrel file | `check_barrel_completeness()` runs | Skipped (not barrel) | ✅ |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `barrel_completeness` per layer | Missing for a layer = no check | Default enabled for all layers except root |

## 9. Appendices
- `src-rust/capabilities/architecture_internal_checker.rs:53` — `check_barrel_completeness()`
- `lint_arwaky.config.rust.yaml` — Per-layer `Taxonomy_Standards`, `Contract_Standards`, etc.
