# 📄 Feature Requirements Document (FRD)
**Feature Name:** Contract Suffix Mismatch Detector (AES008)  
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
This document defines the AES008 rule that enforces contract-specific suffixes. Every `contract_`-prefixed file must use `_port`, `_protocol`, or `_aggregate` suffix to declare its architectural role.

### 2.2 Scope
**In-Scope:**
- Files with `contract_` filename prefix
- Required suffixes: `_port`, `_protocol`, `_aggregate`
- Skipping barrel files (`mod.rs`)
- HIGH severity violations

**Out-of-Scope:**
- Other layers' suffix rules (AES010, AES011 — separate FRDs)
- Content validation of contract files

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES008** | Rule code for contract suffix mismatch |
| **check_domain_suffixes()** | Main detection method (AES008 path at line 186) |
| **_port** | Outbound interface suffix |
| **_protocol** | Inbound interface suffix |
| **_aggregate** | Composition facade suffix |

## 3. Feature Overview
### 3.1 Background & Problem
Files in the contract layer had ambiguous names like `contract_helpers.rs` or `types.rs` that didn't communicate their architectural role. The architecture mandates exactly three roles: ports (outbound), protocols (inbound), and aggregates (composition). Every file must declare which role it serves.

### 3.2 Business Goals
- Every contract file clearly identifies its role via suffix
- Enforce exactly three allowed suffixes in contract layer
- Provide clear naming guidance

### 3.3 Target Users
- **Developers**: Use the correct suffix when adding contract files
- **Architects**: Maintain clear contract layer structure

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer adding a contract_-prefixed file, I want to be told if my filename doesn't end with _port, _protocol, or _aggregate, so I follow the naming convention.

### 4.2 Use Cases & Workflow
**Detection:**
```
File: contract_helpers.rs
  filename starts with "contract_" → layer: contract
  suffix_policy: strict
  allowed_suffix: ["port", "protocol", "aggregate"]

  get_stem("contract_helpers.rs") → "contract_helpers"
  get_suffix("contract_helpers") → "helpers"

  "helpers" NOT in allowed_suffix
  layer == "contract" → emit AES008 code
```

**Valid files:**
```
service_container_aggregate.rs → suffix "aggregate" Pending Review
source_parser_port.rs          → suffix "port" Pending Review
arch_rule_protocol.rs          → suffix "protocol" Pending Review
```

### 4.3 Business Rules
- Severity: HIGH
- Same code path as AES010 but emits AES008 code for contract layer
- Skip mod.rs barrel files

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |

## 6. UI/UX Requirements
```
AES008 HIGH - src-rust/di-containers/contract_project_helpers.rs
  AES008 SUFFIX_MISMATCH: File is missing a required strict suffix.
  WHY? Contract files must use _port, _protocol, or _aggregate suffix.
  FIX: Add one of: port, protocol, aggregate.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Contract file without port/protocol/aggregate suffix | `check_domain_suffixes()` runs | AES008 HIGH flagged | Pending Review |
| AC-002 | Contract file with valid suffix | `check_domain_suffixes()` runs | No violation | Pending Review |
| AC-003 | Barrel file (mod.rs) | `check_domain_suffixes()` runs | Skipped (barrel) | Pending Review |

## 8. Empirical Findings (Code Audit)

N/A — Pending review after vertical slicing refactoring.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `allowed_suffix` for contract layer | Must include port, protocol, aggregate | Configured by default in YAML |

## 10. Appendices
- `src-rust/layer-rules/capabilities_naming_checker.rs:186` — AES008 code path
- `src-rust/shared-common/taxonomy_layer_vo.rs` — `allowed_suffix` config field
- `docs/ARCHITECTURE.md` — Contract layer specification
