# 📄 Feature Requirements Document (FRD)
**Feature Name:** Contract Barrel Import Checker (AES007)  
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
This document defines the AES007 rule that enforces barrel-style imports from the contract layer. All imports from `contract/` must go through the layer barrel (`contract/mod.rs`) using `crate::contract::TypeName` instead of submodule paths.

### 2.2 Scope
**In-Scope:**
- Detecting `crate::contract::xxx::yyy` submodule import patterns
- Rust files only (Python/JS use different module systems)
- MEDIUM severity violations

**Out-of-Scope:**
- Python/JS import style validation
- Forbidden import rules (AES001)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES007** | Rule code for contract barrel import |
| **check_contract_barrel()** | Main detection method |
| **Barrel** | `contract/mod.rs` that re-exports all contract types |

## 3. Feature Overview
### 3.1 Background & Problem
Contract types were imported via submodule paths like `crate::contract::source_parser_port::ISourceParserPort`, creating coupling to internal module structure. The contract barrel exists specifically to be the sole API surface — bypassing it defeats this purpose.

### 3.2 Business Goals
- Ensure contract barrel is the only entry point to contract types
- Standardize import patterns across the codebase
- Prevent coupling to internal contract module structure

### 3.3 Target Users
- **Developers**: Guided to use correct import style
- **Architects**: Ensure contract API surface is properly used

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be told when I use a submodule import path for contract types, so I can use the correct barrel style.

### 4.2 Use Cases & Workflow
**Detection:**
```
✅ Correct:  use crate::contract::ServiceContainerAggregate;
❌ Wrong:    use crate::contract::service_container::ServiceContainerAggregate;

Scan line: "use crate::contract::service_container::ServiceContainerAggregate"
  └── Two segments after "contract" → submodule path → VIOLATION

Scan line: "use crate::contract::ServiceContainerAggregate"
  └── One segment after "contract" → barrel style → OK
```

### 4.3 Business Rules
- Severity: MEDIUM
- Only applies to Rust files
- Pattern: `crate::contract::<submodule>::<Type>` = violation

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per line | < 1ms |

## 6. UI/UX Requirements
```
AES007 MEDIUM - src-rust/surfaces/cli_check_command.rs:5
  AES007 CONTRACT_BARREL: Contract import must be from barrel.
  Use: 'use crate::contract::ServiceContainerAggregate'
  Instead of: 'use crate::contract::service_container::ServiceContainerAggregate'
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File uses `crate::contract::submodule::Type` | `check_contract_barrel()` runs | AES007 MEDIUM flagged | ✅ |
| AC-002 | File uses `crate::contract::Type` | `check_contract_barrel()` runs | No violation | ✅ |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| Rust module system | Only applies to Rust | ✅ By design |
| Code duplication | Logic in 2 files | Maintenance burden | Consolidate to single path |

## 9. Appendices
- `src-rust/capabilities/architecture_compliance_analyzer.rs:414` — `check_contract_barrel()`
- `src-rust/agent/lint_checking_coordinator.rs:191` — Duplicated implementation
- `src-rust/contract/mod.rs` — Contract barrel
