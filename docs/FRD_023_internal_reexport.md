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

N/A — Pending review after vertical slicing refactoring.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `forbid_internal_all` per layer | Missing for a layer = no check | Default enabled for all layers except root |

## 10. Appendices
- `src-rust/layer-rules/capabilities_internal_checker.rs:83` — `check_forbid_internal_all()`
- `lint_arwaky.config.rust.yaml` — Per-layer standards with `forbid_internal_all: true`
