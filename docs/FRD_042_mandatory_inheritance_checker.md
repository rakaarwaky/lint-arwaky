# Feature Requirements Document (FRD)
**Feature Name:** Mandatory Inheritance Checker (AES027)
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
This document defines the AES027 rule that enforces **mandatory contract inheritance** across architecture layers. Files with `infrastructure_`, `capabilities_`, or `agent_` prefix that import contract types (`_port`, `_protocol`, `_aggregate`) must implement at least one of them. Two implementations exist: (1) a full multi-file checker in `MandatoryInheritanceChecker`, and (2) an active single-file heuristic in `check_mandatory_inheritance()`.

### 2.2 Scope
**In-Scope:**
- `infrastructure_` files that import `_port` contracts — must implement at least one
- `capabilities_` files that import `_protocol` contracts — must implement at least one
- `agent_` files that import `_aggregate` contracts — must implement at least one
- CRITICAL severity for violations
- Rust `use` + `impl Trait for Struct` patterns
- Python `from x import Y` + `class X(Y)` patterns (full checker only)

**Out-of-Scope:**
- Auto-fixing violations
- `surface_` and `taxonomy_` layers (they don't implement contracts)
- Contract layer itself (`contract_` prefix files)
- Barrel files (`__init__.py`, `mod.rs`)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES027** | Rule code for mandatory inheritance violation |
| **MandatoryInheritanceChecker** | Full checker in `role-rules/capabilities_inheritance_checker.rs` |
| **check_mandatory_inheritance()** | Active heuristic in `coordinator` |
| **Contract import** | Import of a type with `_port`, `_protocol`, or `_aggregate` suffix |

## 3. Feature Overview
### 3.1 Background & Problem
Layers that import contracts must provide an implementation. Otherwise the contract is dead — imported but never fulfilled. Infrastructure must implement ports, capabilities must implement protocols, and agents must implement aggregates. This rule ensures that every imported contract has at least one implementing struct/class in the same file.

### 3.2 Business Goals
- Ensure every contract import is backed by an implementation
- Eliminate dead contract imports
- Enforce the layer-to-contract mapping: infra→_port, capabilities→_protocol, agent→_aggregate

### 3.3 Target Users
- **Developers**: Get warned when importing a contract without implementing it
- **Architects**: Ensure the contract layer is always fulfilled by concrete implementations

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when my infrastructure file imports a `_port` but doesn't implement it.
- **US-002:** As a developer, I want to be warned when my capabilities file imports a `_protocol` but doesn't implement it.
- **US-003:** As a developer, I want to be warned when my agent file imports a `_aggregate` but doesn't implement it.
- **US-004:** As a developer, I want Python `from contract import PortProto` + missing `class X(PortProto)` to be flagged.

### 4.2 Detection Pipeline
**Full Pipeline (MandatoryInheritanceChecker):**
```
File: src-rust/role-rules/capabilities_inheritance_checker.rs

1. For each file in project:
   a. Skip barrel files (__init__.py, mod.rs)
   b. Detect file layer via config path matching
   c. Is layer in [infrastructure, capabilities, agent]? → NO → skip
   d. Read file content
   e. Extract contract imports:
      - Match `from <module> import <names>`
      - Is imported name contract-like (_port, _protocol, _aggregate suffix)?
      → Collect list of imported contract names
   f. No contract imports? → skip (surface-like file)
   g. Extract class bases:
      - Match `class <Name>(<Base>)`
      → Collect list of base class names
   h. Does any class base match a contract import?
      → If NO → AES027 CRITICAL
```

**Active Pipeline (coordinator):**
```
File: src-rust/code-analysis/agent_checking_coordinator.rs

1. For each line:
   a. Does line start with "use " and contain "_protocol::"?
   b. Extract last segment as trait name
   c. Is name starting with 'I' or ending with "Protocol" or "Port"?
   d. If YES → collect as imported contract trait
2. For each collected trait:
   a. Does content contain "impl <TraitName> for "?
   b. If NO → AES027 HIGH
```

### 4.3 Business Rules
| Rule | Severity | Condition |
|------|----------|-----------|
| Infra imports port without impl | CRITICAL | `infrastructure_` file imports `_port` but no struct implements it |
| Capabilities imports protocol without impl | CRITICAL | `capabilities_` file imports `_protocol` but no struct implements it |
| Agent imports aggregate without impl | CRITICAL | `agent_` file imports `_aggregate` but no struct implements it |
| Layer mapping | — | infrastructure→_port, capabilities→_protocol, agent→_aggregate |
| Barrel files | — | `__init__.py` and `mod.rs` exempt (no logic) |
| Active checker | HIGH | Rust `use` + `impl` heuristic only |

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file (full) | < 50ms |
| NFR-002 | Detection per file (active) | < 10ms |
| NFR-003 | False positive rate | 0% for correctly implemented contracts |
| NFR-004 | False negative rate | < 5% for complex multi-line patterns |

## 6. UI/UX Requirements
```
AES027 CRITICAL - src-rust/infrastructure/db/adapter_port.rs:0
  AES027 MANDATORY_INHERITANCE_VIOLATION: File imports contracts (UserPort, OrderPort)
  but no class inherits from any of them. Layer 'infrastructure' must implement its
  contract via inheritance.
  WHY? Layers that import contracts must provide an implementation. Otherwise the
       contract is dead — imported but never fulfilled.
  FIX: Add 'impl UserPort for YourStruct' to implement the imported contract.

AES027 CRITICAL - src-python/capabilities/payment/processor_protocol.py:0
  AES027 MANDATORY_INHERITANCE_VIOLATION: File imports contracts (PaymentProtocol)
  but no class inherits from any of them.
  WHY? Capabilities must implement the protocols they import.
  FIX: Make at least one class inherit from PaymentProtocol.

AES027 HIGH - src-rust/agent/workflow/orchestrator_aggregate.rs:15
  AES027 MANDATORY_INHERITANCE: Trait 'WorkflowAggregate' not implemented.
  WHY? Active checker heuristic: imported trait has no impl block.
  FIX: Add 'impl WorkflowAggregate for YourStruct'.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Infra file imports port without impl | `MandatoryInheritanceChecker` runs | AES027 CRITICAL flagged | Pending Review |
| AC-002 | Capabilities file imports protocol with impl | Checker runs | No AES027 | Pending Review |
| AC-003 | Agent file imports aggregate without impl | Checker runs | AES027 CRITICAL flagged | Pending Review |
| AC-004 | Python file with `from contract import X` + no class inheritance | Checker runs | AES027 CRITICAL flagged | Pending Review |
| AC-005 | Barrel file (`__init__.py`) with contract imports | Checker runs | Skipped | Pending Review |
| AC-006 | Surface file importing contract | Checker runs | Skipped (not in scope layers) | Pending Review |
| AC-007 | Rust file importing protocol without impl | Active `check_mandatory_inheritance()` runs | AES027 HIGH flagged | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Two Implementations — Full vs Heuristic

#### 8.1.1 MandatoryInheritanceChecker (FULL) — `src-rust/role-rules/capabilities_inheritance_checker.rs`
- **Location**: `src-rust/role-rules/capabilities_inheritance_checker.rs:1-212`
- **Status**: **FULLY IMPLEMENTED** — 212 lines
- Python-focused: parses `from x import Y` patterns (lines 90-118) and `class X(Y)` patterns (lines 120-139)
- Layer detection via config path matching (lines 63-79)
- Implements `IArchInheritanceProtocol` trait (lines 200-211)
- Barrel file exemption (lines 152-154)
- **Integration status**: Need to verify if this is called from the coordinator

#### 8.1.2 Active Heuristic Checker — `agent_checking_coordinator.rs:534-561`
```rust
fn check_mandatory_inheritance(file: &str, content: &str, violations: &mut Vec<LintResult>) {
    let mut imported: Vec<String> = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("use ") && t.contains("_protocol::") {
            if let Some(name) = t.split("::").last() {
                let c = name.trim_end_matches(';').trim();
                if c.starts_with('I') || c.ends_with("Protocol") || c.ends_with("Port") {
                    imported.push(c.to_string());
                }
            }
        }
    }
    for t in &imported {
        if !content.contains(&format!("impl {} for ", t)) {
            violations.push(Self::mk(file, 0, "AES027", Severity::HIGH, "..."));
        }
    }
}
```
**Status**: **ACTIVE** — Called in `run_all_checks()` line 81. Only checks Rust `use` + `_protocol::` imports. Limited to `_protocol` suffix — does NOT check `_port` or `_aggregate` imports.

### 8.2 Bugs/Gaps Found

1. **Active checker only checks `_protocol`** — The heuristic at line 538 filters for `_protocol::` but ignores `_port` and `_aggregate` imports (lines 80-81 of AGENTS.md states infra→_port, caps→_protocol, agent→_aggregate).
2. **Severity mismatch** — Full checker uses CRITICAL (line 51 of capabilities_inheritance_checker.rs), active checker uses HIGH (line 553 of coordinator).
3. **Full checker may be unwired** — `MandatoryInheritanceChecker` implements `IArchInheritanceProtocol` but there's no evidence of it being invoked through the coordinator pipeline. The active `check_mandatory_inheritance` in the coordinator is a standalone function.
4. **Active checker has no layer scoping** — Does not verify what layer the file belongs to before flagging. A surface file importing `_protocol` would be flagged even though surfaces don't need to implement contracts.

### 8.3 What Needs to Be Added
- **Layer scoping** to the active checker — only flag infrastructure/capabilities/agent layers
- **`_port` and `_aggregate` import detection** in the active heuristic
- **Wire full MandatoryInheritanceChecker** into the coordinator pipeline for Python support and CRITICAL severity
- **Align severities** between the two implementations

### 8.4 What to Keep
- **Full MandatoryInheritanceChecker** Pending Review (comprehensive Python + Rust support)
- **Active heuristic** Pending Review (fast Rust protocol check, just needs broader scope)

### 8.5 Empirical Evidence from Test Projects
- `test-project-python/src-python/contract/orphan_protocol.rs` — trait defined but no struct implements it → NOT flagged Pending Review (active checker is Rust-only)
- No Rust test fixture currently exists for AES027 violations

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (AST scanning) | Import + implementation detection | Python pattern matching is fragile | Improve with proper AST |
| MandatoryInheritanceChecker | Full implementation | May be dead code | Wire into coordinator |
| Layer detection (FR-001) | File prefix → layer mapping | Active checker has no layer filter | Add layer scoping to active checker |

## 10. Appendices
- `src-rust/role-rules/capabilities_inheritance_checker.rs:1-212` — MandatoryInheritanceChecker full implementation
- `src-rust/code-analysis/agent_checking_coordinator.rs:534` — Active heuristic `check_mandatory_inheritance()`
- `src-rust/code-analysis/agent_checking_coordinator.rs:81` — Active checker call site
- `src-rust/layer-rules/contract_inheritance_protocol.rs:9` — `IArchInheritanceProtocol` trait
- `test-project-python/src-python/contract/orphan_protocol.rs` — Orphan protocol fixture
