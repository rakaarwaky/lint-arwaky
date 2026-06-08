# Feature Requirements Document (FRD)
**Feature Name:** Surface Hierarchy Violation Detector (AES018)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 08/06/2026  
**Version:** v2.0  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v2.0 | 08/06/2026 | Raka | Added full 3-tier hierarchy (Smart, Utility, Passive) | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES018 rule that enforces **surface hierarchy** constraints. The surface layer has three tiers — Smart, Utility, Passive — each with specific import rules and behavioral mandates. AES018 covers: (1) **barrel wiring** — every non-init surface file must be declared in its layer barrel, (2) **cross-tier import restrictions** — each tier may only import from specific allowed layers and tiers. The barrel wiring check is implemented in `SurfaceHierarchyChecker::check_surface_hierarchy()`.

### 2.2 Surface Hierarchy (3 Tiers)

| Tier | Suffixes | Role | Allowed Imports |
|------|----------|------|----------------|
| **Smart** (top) | `_command`, `_handler`, `_controller`, `_entry` | Entry points — parse I/O, delegate via `ServiceContainerAggregate` | taxonomy, contract(aggregate), surfaces(smart, utility, passive) |
| **Utility** (middle) | `_hook`, `_store`, `_provider`, `_router` | Helpers — stateless, reusable utilities | taxonomy, contract(aggregate), surfaces(passive) |
| **Passive** (bottom) | `_component`, `_layout`, `_view` | Dumb views — present data only | taxonomy only |

**Hierarchy Rule:** A tier may NOT import from a tier above it:
- Utility → Smart ❌ (forbidden)
- Passive → Smart ❌ (forbidden)
- Passive → Utility ❌ (forbidden)

**In-Scope:**
- Barrel wiring: verifying every surface file is re-exported via `__init__.py`/`mod.rs`/`index.ts`
- Smart surface import validation
- Utility surface import restriction (must NOT import Smart)
- Passive surface import restriction (must import taxonomy ONLY)
- HIGH severity reporting

**Out-of-Scope:**
- Import-rule enforcement currently emitted as AES001 (config defines AES018 messages)
- Auto-fixing missing barrel declarations
- Cross-language barrel format validation
- Method count / body length / nesting checks (AES019 — separate FRD)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES018** | Rule code for surface hierarchy violation |
| **check_surface_hierarchy()** | Main detection method in `SurfaceHierarchyChecker` |
| **Barrel wiring** | Requirement that every surface file is imported in its layer's barrel module |
| **Smart surface** | Top-tier surface: `_command`, `_handler`, `_controller`, `_entry` |
| **Utility surface** | Mid-tier surface: `_hook`, `_store`, `_provider`, `_router` |
| **Passive surface** | Bottom-tier surface: `_component`, `_layout`, `_view` |

## 3. Feature Overview
### 3.1 Background & Problem
Surface files that are not wired into their layer barrel become invisible to consumers and can drift out of the architecture. Without tier-based import restrictions, Smart surfaces can be imported by Utility surfaces (creating circular dependency risk) and Passive surfaces can import business logic (breaking the "dumb view" contract).

### 3.2 Business Goals
- Ensure every surface file is discoverable through its layer barrel
- Enforce the 3-tier hierarchy: Smart → Utility → Passive (top-down only)
- Prevent tier violations: Utility must NOT import Smart, Passive must import taxonomy only
- Provide clear, actionable violation messages

### 3.3 Target Users
- **Developers**: Get notified when a surface file is unwired or violates tier import rules
- **Architects**: Configure barrel and import rules per project via YAML

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when my surface file is not listed in the barrel.
- **US-002:** As a developer, I want to be warned when my Utility surface imports a Smart surface.
- **US-003:** As a developer, I want to be warned when my Passive surface imports from agent/infrastructure/capabilities.
- **US-004:** As an architect, I want each tier's allowed imports configurable per project via YAML.

### 4.2 Use Cases & Workflow
**Barrel Wiring Pipeline:**
```
1. Collect all surface layer files
2. For each file (skip __init__.py, mod.rs, index.ts, index.js):
   a. Is file imported in barrel (__init__.py/mod.rs/index.ts)?
   b. If not → AES018 HIGH
```

**Cross-Tier Import Pipeline (config-defined):**
```
File: surfaces/utility_import_store.rs  → Utility tier

1. Detect surface tier by suffix:
   - _command/_handler/_controller/_entry → Smart
   - _hook/_store/_provider/_router       → Utility
   - _component/_layout/_view             → Passive
2. Check allowed_imports for the tier:
   - If Utility tries to import Smart → AES018
   - If Passive tries to import Smart/Utility/agent/infra/capabilities → AES019
```

### 4.3 Business Rules
- Severity: HIGH
- Barrel wiring checked for all non-init surface files
- Barrel files checked: `__init__.py` (Python), `mod.rs` (Rust), `index.ts`/`index.js` (JS/TS)
- 3-tier hierarchy: Smart (top) → Utility (middle) → Passive (bottom)
- Smart surfaces may import: taxonomy, contract(aggregate), any surface tier
- Utility surfaces may import: taxonomy, contract(aggregate), passive surfaces ONLY
- Passive surfaces may import: taxonomy ONLY
- Import violations currently emitted as AES001 (config references AES018/AES019)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per project | < 50ms total |
| NFR-002 | False positive rate | 0% for properly wired files |
| NFR-003 | False negative rate | 0% for unwired files |

## 6. UI/UX Requirements
```
AES018 HIGH - src-rust/surfaces/complex_view_handler.rs
  AES018 MISSING_BARREL: Surface file not exported from mod.rs.
  WHY? Each surface must be explicitly wired via the layer barrel to be discoverable.
  FIX: Add `pub mod complex_view_handler;` to mod.rs.

AES018 HIGH - src-rust/surfaces/utility_import_store.rs
  AES018 SURFACE_HIERARCHY_VIOLATION: Utility surface imports Smart surface.
  WHY? Utility surfaces must be independent of Smart surfaces.
  FIX: Remove the Smart surface import; use dependency injection instead.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Surface file NOT in barrel | `check_surface_hierarchy()` runs | AES018 HIGH flagged | ✅ |
| AC-002 | Surface file properly barrelled | `check_surface_hierarchy()` runs | No AES018 | ✅ |
| AC-003 | Barrel file itself (`__init__.py`) | `check_surface_hierarchy()` runs | Skipped | ✅ |
| AC-004 | Utility surface imports Smart surface | Import check runs | AES018 flagged | ❌ Emitted as AES001 |
| AC-005 | Passive surface imports agent | Import check runs | AES019 flagged | ❌ Emitted as AES001 |
| AC-006 | Smart surface imports from any layer | Import check runs | Allowed | ✅ |
| AC-007 | Non-surface file outside /surfaces/ | `check_surface_hierarchy()` runs | Skipped | ✅ |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location**: `src-rust/capabilities/surface_hierarchy_checker.rs:43-79`
- **Status**: **FULLY IMPLEMENTED** — barrel wiring check only
- Invoked from `lint_checking_coordinator.rs:154`

### 8.2 Bugs Found

1. **Error code mismatch for import restriction** (`architecture_import_checker.rs:339`)
   - Config defines AES018/AES019 violation messages for cross-tier surface imports
   - `ArchImportRuleChecker::check_forbidden_imports` hardcodes `AES001` as error code
   - **Impact**: all import-based surface hierarchy violations are labeled AES001
   - **Fix**: wire error code from config or accept AES001 as the correct code for import rules

2. **Barrel wiring primarily supports Python**
   - The `is_wired` helper (`surface_hierarchy_checker.rs:296-316`) reads `__init__.py` content
   - Rust `mod.rs` support relies on file existence checks, not content analysis
   - **Impact**: Rust surface files may get false negatives
   - **Fix**: implement content-aware `mod.rs` parsing

3. **No explicit 3-tier detection in barrel wiring**
   - Barrel wiring treats all surface files uniformly
   - Tier-specific constraints (Smart vs Utility vs Passive) are only in config import rules
   - **Impact**: no active code validates tier-based import rules independently

### 8.3 What Needs to Be Added
- **Error code routing**: make `ArchImportRuleChecker` emit the config-defined error code (AES018/AES019) instead of hardcoded AES001
- **Rust barrel validation**: implement full `mod.rs` content scanning for surface declarations
- **JS/TS barrel support**: verify `index.ts` scanning works for all surface tiers
- **Tier-aware import checker**: add explicit Smart/Utility/Passive tier detection with separate code paths

### 8.4 What to Keep
- **Barrel wiring logic** ✅ (`surface_hierarchy_checker.rs:43-79`)
- **Per-surface-tier forbidden imports** ✅ (config sections for Smart, Utility, Passive)
- **Coordinator pipeline integration** ✅ (`lint_checking_coordinator.rs:154`)

### 8.5 Empirical Evidence from Test Projects
- `test-project-rust/src-rust/surfaces/utility_import_store.rs` — Utility imports Smart → flagged as AES001 ❌
- `test-project-rust/src-rust/surfaces/passive_bad_view.rs` — Passive imports agent → flagged as AES001 ❌
- `test-project-rust/src-rust/surfaces/complex_view_handler.rs` — Smart surface → no tier violation
- No dedicated AES018 barrel-wiring fixture exists

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-001 (Layer detection) | Surface tier detection by suffix | File misclassified | Existing layer tests |
| Config YAML | Import rules per surface tier | Error code mismatch | Wire config error codes |
| File I/O | Reading barrel content | Unreadable barrel → false positive | Skip on read error |

## 10. Appendices
- `src-rust/capabilities/surface_hierarchy_checker.rs:43` — `check_surface_hierarchy()`
- `src-rust/agent/lint_checking_coordinator.rs:154` — Invocation
- `src-rust/capabilities/architecture_import_checker.rs:339` — Hardcoded AES001
- `lint_arwaky.config.rust.yaml:582` — Smart_Surface_Relations config
- `lint_arwaky.config.rust.yaml:593` — Utility_Surface_Relations config
- `lint_arwaky.config.rust.yaml:610` — Passive_Surface_Relations config
