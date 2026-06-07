# 📄 Feature Requirements Document (FRD)
**Feature Name:** Contract Suffix Mismatch Detector (AES008)  
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
This document defines the AES008 rule that enforces contract-specific suffixes. Every file in the `contract/` layer must use `_port`, `_protocol`, or `_aggregate` suffix to declare its architectural role.

### 2.2 Scope
**In-Scope:**
- Files in `src-rust/contract/` directory
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
- **US-001:** As a developer adding a file to contract/, I want to be told if my filename doesn't end with _port, _protocol, or _aggregate, so I follow the naming convention.

### 4.2 Use Cases & Workflow
**Detection:**
```
File: src-rust/contract/helpers.rs
  Layer: contract
  suffix_policy: strict
  allowed_suffix: ["port", "protocol", "aggregate"]

  get_stem("helpers.rs") → "helpers"
  get_suffix("helpers") → "helpers"

  "helpers" NOT in allowed_suffix
  layer == "contract" → emit AES008 code
```

**Valid files:**
```
service_container_aggregate.rs → suffix "aggregate" ✅
source_parser_port.rs          → suffix "port" ✅
arch_rule_protocol.rs          → suffix "protocol" ✅
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
AES008 HIGH - src-rust/contract/project_helpers.rs
  AES008 SUFFIX_MISMATCH: File is missing a required strict suffix.
  WHY? Contract files must use _port, _protocol, or _aggregate suffix.
  FIX: Add one of: port, protocol, aggregate.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Contract file without port/protocol/aggregate suffix | `check_domain_suffixes()` runs | AES008 HIGH flagged | ✅ |
| AC-002 | Contract file with valid suffix | `check_domain_suffixes()` runs | No violation | ✅ |
| AC-003 | mod.rs in contract/ | `check_domain_suffixes()` runs | Skipped (barrel) | ✅ |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `allowed_suffix` for contract layer | Must include port, protocol, aggregate | Configured by default in YAML |

## 9. Appendices
- `src-rust/capabilities/architecture_naming_checker.rs:186` — AES008 code path
- `src-rust/taxonomy/layer_definition_vo.rs` — `allowed_suffix` config field
- `docs/ARCHITECTURE.md` — Contract layer specification
