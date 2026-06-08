# 📄 Feature Requirements Document (FRD)
**Feature Name:** Surface Layer Rule Checker (AES022)  
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
This document defines the AES022 rule that enforces surface layer passivity. Surfaces must NOT contain domain logic — they are thin I/O layers that parse input and delegate to capabilities/infrastructure through `ServiceContainerAggregate`.

### 2.2 Scope
**In-Scope:**
- AES018: Surface barrel wiring check (files declared in barrel)
- AES019: Passive surface validation (method count, line count, if-depth)
- Three surface roles: Smart, Utility, Passive
- Threshold enforcement: 10 methods, 80 lines/function, depth 3

**Out-of-Scope:**
- Surface direct import check (AES023 — separate FRD)
- Non-surface layer rules

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES022** | Rule code for surface layer enforcement |
| **AES018** | Barrel wiring — utility surfaces must not import Smart surfaces |
| **AES019** | Passive surface — imports taxonomy only, no domain logic |
| **Smart Surface** | `_command`, `_handler`, `_controller`, `_entry` |
| **Utility Surface** | `_hook`, `_store`, `_provider`, `_router` |
| **Passive Surface** | `_component`, `_layout`, `_view` |
| **SurfaceHierarchyChecker** | Capability that enforces AES022 |

## 3. Feature Overview
### 3.1 Background & Problem
Surfaces contained domain logic — CLI commands implemented business algorithms, MCP handlers performed data transformations. There was no enforcement of the passivity principle. Utility surfaces imported Smart surfaces, creating circular dependencies.

### 3.2 Business Goals
- Surfaces must be passive I/O layers only
- Smart surfaces delegate via ServiceContainerAggregate
- Utility surfaces independent of Smart surfaces
- Passive surfaces know taxonomy only

### 3.3 Target Users
- **Developers**: Keep surfaces thin and focused on I/O
- **Architects**: Enforce clean architecture boundaries

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want surfaces with too much domain logic to be flagged, so I keep my CLI commands thin.
- **US-002:** As an architect, I want passive surfaces restricted to taxonomy imports only, so presentation stays separate from logic.

### 4.2 Use Cases & Workflow
**AES018 — Barrel Wiring Check:**
```
For each file with surface_ prefix:
  Is it declared in the feature folder's mod.rs or __init__.py?
    → NOT declared → AES018 violation
```

**AES019 — Passive Surface Check:**
```
File: cli-commands/surface_dashboard_view.rs
  1. Count public methods: 15
  2. Max allowed: 10
  3. 15 > 10 → VIOLATION

  2. Check function body: user_display() = 120 lines
  3. Max allowed: 80
  4. 120 > 80 → VIOLATION

  3. Check if-depth: nested 4 levels
  4. Max depth: 3
  5. 4 > 3 → VIOLATION
```

### 4.3 Business Rules
- Severity: HIGH
- Smart surfaces: must delegate logic, not implement it
- Utility surfaces: must NOT import Smart surfaces
- Passive surfaces: must import taxonomy ONLY
- Thresholds: 10 methods, 80 lines/function, if-depth 3

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Surface detection per file | < 10ms |

## 6. UI/UX Requirements
```
AES022 HIGH - src-rust/cli-commands/surface_dashboard_view.rs
  AES019 PASSIVE_SURFACE_VIOLATION: Surface contains domain logic.
  Found 15 public methods (max 10), function body 120 lines (max 80).
  WHY? Surfaces must be passive I/O layers.
  FIX: Move business logic to capabilities.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Surface with 15 methods (max 10) | AES019 check runs | Violation flagged | Pending Review |
| AC-002 | Surface with function body 120 lines (max 80) | AES019 check runs | Violation flagged | Pending Review |
| AC-003 | Surface declared in barrel | AES018 check runs | No violation | Pending Review |
| AC-004 | Surface NOT declared in barrel | AES018 check runs | Violation flagged | Pending Review |

## 8. Empirical Findings (Code Audit)

N/A — Pending review after vertical slicing refactoring.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Count methods, lines, depth | Regex inaccuracy affects thresholds | Conservative thresholds |
| Thresholds | 10/80/3 hardcoded | Not configurable via YAML | Plan: move to YAML config |

## 10. Appendices
- `src-rust/layer-rules/capabilities_hierarchy_checker.rs` — Full implementation (351 lines)
- `docs/ARCHITECTURE.md` — Surface layer specification
