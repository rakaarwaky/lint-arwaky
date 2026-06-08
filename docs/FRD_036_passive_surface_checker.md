# Feature Requirements Document (FRD)
**Feature Name:** Passive Surface Violation Detector (AES019)  
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
This document defines the AES019 rule that enforces passive surface constraints: passive surfaces (`_component`, `_layout`, `_view`) must only import from the taxonomy layer, must have limited public methods (≤10), must keep method bodies short (≤80 lines), and must limit if-nesting depth (≤3). The rule is implemented in `SurfaceHierarchyChecker::_check_passive()`.

### 2.2 Scope
**In-Scope:**
- Import restriction: passive surfaces must import taxonomy only
- Method count check: ≤10 public methods per file
- Method body length: ≤80 lines per method
- Method nesting depth: ≤3 levels of if-nesting
- HIGH severity reporting

**Out-of-Scope:**
- Auto-fixing violations
- Configuration of method/branch thresholds (currently hardcoded)
- Cross-language support beyond Python

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES019** | Rule code for passive surface violation |
| **_check_passive()** | Entry point in `SurfaceHierarchyChecker` |
| **Passive surface** | Surface file with suffix `_component`, `_layout`, or `_view` |
| **PY_METHOD_RE** | Python regex for detecting method definitions |

## 3. Feature Overview
### 3.1 Background & Problem
Passive surfaces (components, layouts, views) are "dumb" UI elements that should only know about data structures (taxonomy). When they import business logic, contain too many methods, or have deeply nested control flow, they violate the passive surface contract and blur architectural boundaries.

### 3.2 Business Goals
- Enforce strict import isolation for passive surfaces
- Limit complexity in passive surfaces (methods, body length, nesting)
- Ensure passive surfaces remain "dumb" view components

### 3.3 Target Users
- **Developers**: Get notified when a passive surface is too complex or imports from forbidden layers
- **Architects**: Ensure UI components remain pure presentation layers

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when my passive surface imports from agent/capabilities/infrastructure.
- **US-002:** As a developer, I want to be warned when my component has too many methods or deep nesting.

### 4.2 Use Cases & Workflow
**Detection Pipeline:**
```
File: cli-commands/surface_passive_view.rs

1. Detect layer: surfaces(component|layout|view)
2. Is file a passive surface (suffix check)?
3. For Python files:
   a. Count public methods (>10 → AES019)
   b. Check method body lengths (>80 lines → AES019)
   c. Check if-nesting depth (>3 → AES019)
4. Check imports: only taxonomy allowed (config-defined, emitted as AES001)
```

### 4.3 Business Rules
- Severity: HIGH
- Scopes checked: `surfaces(component)`, `surfaces(layout)`, `surfaces(view)`
- Maximum public methods: 10
- Maximum method body: 80 lines
- Maximum if-nesting depth: 3 levels
- Import restriction: taxonomy only (config, emitted as AES001)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 20ms |
| NFR-002 | False positive rate (passive surfaces) | 0% |
| NFR-003 | False negative rate (non-passive surfaces) | 0% |

## 6. UI/UX Requirements
```
AES019 HIGH - src-python/cli-commands/surface_busy_handler.py
  AES019 PASSIVE_METHOD_COUNT: Passive surface has >10 public methods (found: 12).
  WHY? Passive surfaces are "dumb" components and must remain simple.
  FIX: Split complex presentation logic into sub-components.

AES019 HIGH - src-python/cli-commands/surface_nested_view.py
  AES019 PASSIVE_NESTING: Passive surface has if-nesting depth >3.
  WHY? Deep control flow in passive surfaces indicates logic contamination.
  FIX: Extract nested branches into dedicated sub-components.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Python passive surface with 12 methods | `_check_passive()` runs | AES019 flagged (method count) | Pending Review |
| AC-002 | Python passive surface with 80-line method | `_check_passive()` runs | AES019 flagged (body length) | Pending Review |
| AC-003 | Python passive surface with nesting depth 4 | `_check_passive()` runs | AES019 flagged (nesting) | Pending Review |
| AC-004 | Rust passive surface with 12 methods | `_check_passive()` runs | NOT checked | Pending Review Rust not supported |
| AC-005 | Passive surface imports agent | Import check runs | AES019 flagged | Pending Review Emitted as AES001 |
| AC-006 | Non-passive surface file | `_check_passive()` runs | Skipped | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location**: `src-rust/layer-rules/capabilities_hierarchy_checker.rs:87-254`
- **Status**: **FULLY IMPLEMENTED** — Python only
- Invoked from `lint_checking_coordinator.rs:154` via `check_surface_hierarchy()`

### 8.2 Bugs Found

1. **Python-only analysis** (`src-rust/layer-rules/capabilities_hierarchy_checker.rs:98-120`)
   - `PY_METHOD_RE`, `PY_CLASS_RE`, `IF_RE` are Python-specific regex
   - Rust `.rs` and JS/TS surface files are never analyzed for method count/body/nesting
   - **Impact**: Rust/JS passive surfaces bypass AES019 method checks
   - **Fix**: add Rust (`fn `) and JS/TS method detection

2. **Error code mismatch** (`src-rust/layer-rules/capabilities_import_checker.rs:339`)
   - Config defines AES019 for passive surface import restriction
   - `ArchImportRuleChecker` emits AES001 instead
   - **Impact**: import-based AES019 violations are labeled AES001

### 8.3 What Needs to Be Added
- **Rust passive surface analysis**: extend `_check_passive()` to detect `fn ` declarations and count methods for `.rs` files
- **JS/TS passive surface analysis**: add `method|function` detection for `.ts`/`.tsx`/`.js`/`.jsx` files
- **Error code routing**: make import checker emit config-defined error codes

### 8.4 What to Keep
- **Python passive surface checks** Pending Review (method count, body length, nesting)
- **Import restriction config** Pending Review (`Passive_Surface_Relations` in YAML)
- **Coordinator pipeline integration** Pending Review (`src-rust/pipeline-jobs/agent_checking_coordinator.rs:154`)

### 8.5 Empirical Evidence from Test Projects
- `test-project-rust/src-rust/cli-commands/surface_busy_handler.py` — 12 methods → flagged AES019 Pending Review
- `test-project-rust/src-rust/cli-commands/surface_passive_view.rs` — imports agent → flagged AES001 Pending Review (should be AES019)
- `test-project-rust/src-rust/cli-commands/surface_view_handler.rs` — 6 methods + deep nesting → NOT flagged Pending Review (Rust)

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-001 (Layer detection) | Surface suffix detection | File misclassified | Layer detection tests |
| FR-003 (AST Parsing) | Method/body/nesting analysis | Python regex only | Extend to Rust/JS |
| Config YAML | Import rules per tier | Error code mismatch | Wire config codes |

## 10. Appendices
- `src-rust/layer-rules/capabilities_hierarchy_checker.rs:87` — `_check_passive()`
- `src-rust/layer-rules/capabilities_hierarchy_checker.rs:153` — `_check_methods_too_public()`
- `src-rust/layer-rules/capabilities_hierarchy_checker.rs:170` — `_check_method_lengths()`
- `src-rust/layer-rules/capabilities_hierarchy_checker.rs:191` — `_check_method_nesting()`
- `lint_arwaky.config.rust.yaml:610` — Passive_Surface_Relations config
