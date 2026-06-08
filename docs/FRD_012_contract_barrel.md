# 📄 Feature Requirements Document (FRD)
**Feature Name:** Contract Barrel Import Checker (AES007)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.1  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v1.1 | 09/06/2026 | Raka | Updated to prefix-based architecture: layers are filename prefixes, not directories; updated file paths to reflect 26 feature folders | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES007 rule that enforces barrel-style imports from the contract layer. Contract-prefixed types (`contract_*`) must be imported via the feature folder's barrel (`mod.rs`) using `crate::contract::TypeName` instead of submodule paths.

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
| **Barrel** | Feature folder's `mod.rs` that re-exports all contract types (e.g., `di-containers/contract_service_aggregate.rs`) |

## 3. Feature Overview
### 3.1 Background & Problem
Contract types were imported via submodule paths like `crate::contract::source_parser_port::ISourceParserPort`, creating coupling to internal module structure. The contract barrel (re-exported via `contract_service_aggregate.rs`) exists specifically to be the sole API surface — bypassing it defeats this purpose.

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
❌ Wrong:    use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;

Scan line: "use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate"
  └── Submodule path instead of barrel → VIOLATION (contract_-prefixed type via submodule)

Scan line: "use crate::contract::ServiceContainerAggregate"
  └── Barrel import → OK
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
AES007 MEDIUM - src-rust/cli-commands/surface_check_command.rs:5
  AES007 CONTRACT_BARREL: Contract import must be from barrel.
  Use: 'use crate::contract::ServiceContainerAggregate'
  Instead of: 'use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate'
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File uses `crate::contract::submodule::Type` | `check_contract_barrel()` runs | AES007 MEDIUM flagged | Pending Review |
| AC-002 | File uses `crate::contract::Type` | `check_contract_barrel()` runs | No violation | Pending Review |

## 8. Empirical Findings (Code Audit)

N/A — Pending review after vertical slicing refactoring.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| Rust module system | Only applies to Rust | Pending Review By design |
| Code duplication | Logic in 2 files | Maintenance burden | Consolidate to single path |

## 10. Appendices
- `src-rust/layer-rules/capabilities_compliance_analyzer.rs:414` — `check_contract_barrel()`
- `src-rust/pipeline-jobs/agent_checking_coordinator.rs:191` — Duplicated implementation
- `src-rust/di-containers/contract_service_aggregate.rs` — Contract barrel
