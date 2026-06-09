# Feature Requirements Document (FRD)
**Feature Name:** Forbidden Inheritance Detector (AES026)
**Product:** Lint Arwaky v1.10.2
**Author:** Raka
**Date:** 09/06/2026
**Version:** v1.0

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 09/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES026 rule that detects **forbidden inheritance patterns** in the contract layer. A Contract Aggregate (`contract_` + `_aggregate`) must NOT inherit from a Contract Port (`_port`) or Contract Protocol (`_protocol`). The Aggregate is a composition contract — inheriting from Port/Protocol violates its role as a facade. Enforced by `ContractRoleChecker` in `role-rules/`.

### 2.2 Scope
**In-Scope:**
- Contract Aggregate files (`contract_*_aggregate.rs`) that inherit from Port or Protocol types
- `impl PortName for AggregateName` patterns detected in aggregate files
- HIGH severity reporting
- Rust and Python contract files

**Out-of-Scope:**
- Auto-fixing violations
- Non-contract files inheriting from port/protocol (capabilities must implement protocols — that's AES027)
- Inheritance between two aggregates
- Inheritance between two ports

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES026** | Rule code for forbidden inheritance violation |
| **ContractRoleChecker** | Checker struct in `role-rules/capabilities_contractrole_checker.rs` |
| **check_aggregate()** | Method on `ContractRoleChecker` that enforces AES026 |
| **Contract Aggregate** | A composition contract that aggregates ports/protocols as fields, not inheritance |
| **forbidden_inheritance** | Config field in `LayerDefinition` listing forbidden import scopes |

## 3. Feature Overview
### 3.1 Background & Problem
The AES architecture defines three contract roles: Port (interface), Protocol (behavior contract), and Aggregate (composition facade). Aggregates compose ports and protocols through fields (composition), not inheritance. When an aggregate inherits from a port or protocol, it breaks the facade pattern and couples the aggregate to a specific interface rather than abstracting over it.

### 3.2 Business Goals
- Enforce composition-over-inheritance in the contract layer
- Prevent aggregates from becoming coupled to specific port/protocol implementations
- Maintain clean separation between contract roles

### 3.3 Target Users
- **Developers**: Get warned when writing `impl PortTrait for MyAggregate` in a contract aggregate file
- **Architects**: Ensure contract aggregates remain pure composition facades

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when my Aggregate file implements a Port trait.
- **US-002:** As a developer, I want to be warned when my Aggregate file implements a Protocol trait.
- **US-003:** As an architect, I want the forbidden inheritance patterns configurable via YAML.

### 4.2 Detection Pipeline
```
File: src-rust/role-rules/capabilities_contractrole_checker.rs

1. Identify file layer → is this a contract_aggregate?
2. Load LayerDefinition.forbidden_inheritance patterns from YAML config
3. If patterns list is empty → skip (nothing to check)
4. For each line in file:
   a. Does line start with "use "?
   b. For each forbidden_inheritance pattern:
      - Resolve scope: layer(suffixes) e.g. "contract(_port|_protocol)"
      - Does import line contain the forbidden layer?
      - Does it contain the forbidden suffix?
      - If YES → extract the imported trait name
5. For each extracted trait name:
   a. Does content contain "impl <TraitName> for "?
   b. If YES → AES026 HIGH (forbidden inheritance)
```

### 4.3 Business Rules
| Rule | Severity | Condition |
|------|----------|-----------|
| Aggregate implements Port | HIGH | `contract_*_aggregate` file has `impl PortName for AggregateName` |
| Aggregate implements Protocol | HIGH | `contract_*_aggregate` file has `impl ProtocolName for AggregateName` |
| Config-driven patterns | — | Patterns defined in `LayerDefinition.forbidden_inheritance` from YAML |
| Scope resolution | — | Patterns may include layer+suffix filters (e.g., `contract(_port)`) |

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 20ms |
| NFR-002 | False positive rate | 0% for legitimate aggregate composition |
| NFR-003 | False negative rate | 0% for actual forbidden inheritance |

## 6. UI/UX Requirements
```
AES026 HIGH - src-rust/contract/user/aggregate_aggregate.rs:25
  AES026 FORBIDDEN_INHERITANCE: 'UserPort' implemented from forbidden source.
  WHY? Contract Aggregate inherits from Port. Aggregate is a composition contract;
       inheriting from Port/Protocol violates its role as a facade.
  FIX: Change inheritance to aggregation. Define Ports as fields within the struct.

AES026 HIGH - src-rust/contract/order/aggregate_aggregate.rs:42
  AES026 FORBIDDEN_INHERITANCE: 'PaymentProtocol' implemented from forbidden source.
  WHY? Contract Aggregate inherits from Protocol. Aggregate must remain agnostic
       to implementation.
  FIX: Use composition — embed protocol references as fields instead.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Contract aggregate with `impl PortTrait for MyAgg` | `check_aggregate()` runs | AES026 HIGH flagged | Pending Review |
| AC-002 | Contract aggregate with `impl ProtocolTrait for MyAgg` | Checker runs | AES026 HIGH flagged | Pending Review |
| AC-003 | Contract aggregate with no forbidden impl | Checker runs | No AES026 | Pending Review |
| AC-004 | Non-aggregate file with `impl PortTrait for X` | Checker runs | Skipped (not aggregate) | Pending Review |
| AC-005 | Capabilities file implementing protocol | Checker runs | No AES026 (allowed, this is AES027 territory) | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

#### 8.1.1 ContractRoleChecker — `src-rust/role-rules/capabilities_contractrole_checker.rs`
- **Location**: `src-rust/role-rules/capabilities_contractrole_checker.rs:1-57`
- **Status**: **FULLY IMPLEMENTED**
- `check_aggregate()` method (lines 13-41) — reads forbidden inheritance patterns from `LayerDefinition`, scans `use` statements, resolves scope with suffix filters, then checks for `impl Trait for Struct` patterns
- `check_port()` — returns empty vec (placeholder, not implemented): line 10
- `check_protocol()` — returns empty vec (placeholder, not implemented): line 11
- `resolve_scope()` — helper to parse `layer(suffix1|suffix2)` patterns: lines 43-56
- **Called from**: `agent_checking_coordinator.rs:156` — wired into the coordinator pipeline

#### 8.1.2 Coordinator Integration — `agent_checking_coordinator.rs:156`
```rust
contract_checker.check_aggregate(file, &c, def, &mut violations);
```
Called inside the layer-dependent loop after layer detection. `def` is a `LayerDefinition` loaded from config.

### 8.2 Bugs/Gaps Found

1. **check_port() and check_protocol() are empty** — `ContractRoleChecker` has `check_port()` and `check_protocol()` methods that return `vec![]`. These are never called and have no logic. If port/protocol-specific checks are needed in the future, they need implementation.
2. **Pattern matching is fragile** — `contains()` on lowercased import lines may match substrings. E.g., `contract(port)` would match `contract_support` if it contains "port" in the full path.
3. **No Python support** — The checker only analyzes `use` statements (Rust). Python `from ... import ...` statements are not parsed for AES026.

### 8.3 What Needs to Be Added
- Port/protocol-specific check implementations if needed
- Python import statement parsing for forbidden inheritance patterns
- More precise scoping to avoid false positive substring matches

### 8.4 What to Keep
- `check_aggregate()` implementation Pending Review (core logic works)
- `resolve_scope()` helper Pending Review (correct pattern parsing)
- Config-driven `forbidden_inheritance` patterns Pending Review

### 8.5 Empirical Evidence from Test Projects
- `test-project-rust/src-rust/contract/forbidden_inherit_aggregate.rs` — Aggregate implements Port trait → flagged AES026 Pending Review

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (AST scanning) | Line-level import analysis | Substring matching may cause false positives | Improve pattern resolution |
| Config YAML `forbidden_inheritance` | Layer-specific forbidden patterns | Missing patterns means no enforcement | Audit config completeness |
| ContractRoleChecker | Full implementation | No known dead-code issues | Already wired in coordinator |

## 10. Appendices
- `src-rust/role-rules/capabilities_contractrole_checker.rs:1-57` — ContractRoleChecker full implementation
- `src-rust/code-analysis/agent_checking_coordinator.rs:156` — `check_aggregate()` call site
- `src-rust/shared-common/taxonomy_definition_vo.rs:74` — `forbidden_inheritance` field
- `src-rust/shared-common/taxonomy_violation_constant.rs:47` — AES026 violation message constant
- `test-project-rust/src-rust/contract/forbidden_inherit_aggregate.rs` — Forbidden inheritance fixture
