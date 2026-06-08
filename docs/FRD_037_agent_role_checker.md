# Feature Requirements Document (FRD)
**Feature Name:** Agent Role Violation Detector (AES021)  
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
This document defines the AES021 rule that enforces behavioral mandates per agent role. Each agent role (container, orchestrator, coordinator, registry, manager) has specific constraints. The rule has two implementations: (1) a **dead-code full implementation** in `ArchRoleChecker` covering 6 sub-role checks, and (2) an **active simplistic check** in `lint_checking_coordinator` that only flags files > 300 lines.

### 2.2 Scope
**In-Scope (active):**
- Agent file line count > 300 detected
- HIGH severity reporting

**In-Scope (full implementation — dead code):**
- Container: no domain logic, must implement ServiceContainerAggregate, lazy/eager init only
- Orchestrator: stateless execution, single execution goal, no `any` type
- Coordinator: high-level policy only, coordinates multiple orchestrators
- Registry: CRUD only, no decision logic, thread/async safe
- Manager: no domain data storage, owns system health transitions, lifecycle tracking only

**Out-of-Scope:**
- Auto-fixing violations
- Dynamic threshold configuration per role

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES021** | Rule code for agent role violation |
| **check_agent_roles()** | Full role checker in `ArchRoleChecker` (DEAD CODE) |
| **check_agent_role()** | Active simple checker in `lint_checking_coordinator` |
| **ServiceContainerAggregate** | Contract interface that all containers must implement |

## 3. Feature Overview
### 3.1 Background & Problem
Agent files in the AES architecture have specific role-based behavioral mandates. Containers must only wire dependencies, orchestrators must be stateless, coordinators must handle high-level policy, registries must be CRUD-only, and managers must track lifecycle. Without enforcement, agents accumulate logic that violates their architectural role.

### 3.2 Business Goals
- Enforce role-specific behavioral constraints per agent role
- Prevent agents from growing beyond a manageable size (>300 lines)
- Detect architectural violations early in the development cycle

### 3.3 Target Users
- **Developers**: Get feedback when agent logic violates role constraints
- **Architects**: Configure role-specific violation messages via YAML

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when my agent file exceeds 300 lines.
- **US-002:** As a developer, I want to be warned when my container contains domain logic.
- **US-003:** As a developer, I want to be warned when my orchestrator holds mutable state.
- **US-004:** As a developer, I want to be warned when my registry has decision logic.

### 4.2 Use Cases & Workflow
**Active Pipeline (line count only):**
```
File: pipeline-jobs/agent_large_orchestrator.rs

1. Does filename have agent_ prefix? → YES
2. Count total lines
3. If count > 300 → AES021 HIGH
```

**Full Pipeline (dead code — ArchRoleChecker):**
```
File: pipeline-jobs/agent_stateful_orchestrator.rs

1. Detect agent role: suffix → orchestrator
2. Load LayerDefinition for role scope
3. Apply role-specific checks:
   - Container: no_domain_logic, must_implement_ServiceContainerAggregate, lazy_eager_init_only
   - Orchestrator: stateless_execution, single_execution_goal
   - Coordinator: high_level_policy_only, coordinates_multiple_orchestrators
   - Registry: crud_only, no_decision_logic, thread_async_safe
   - Manager: no_domain_data_storage, owns_system_health_transitions, lifecycle_tracking_only
```

### 4.3 Business Rules
- Severity: HIGH
- Active rule: file > 300 lines with `agent_` prefix
- Full rule (dead code): per-role behavioral checks from config
- Container must implement `ServiceContainerAggregate` contract
- Orchestrator must be stateless (no mutable fields outside constructor)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms (line count) / < 50ms (full check) |
| NFR-002 | False positive rate | 0% for valid agent files |
| NFR-003 | False negative rate | 0% for violating agent files |

## 6. UI/UX Requirements
```
AES021 HIGH - src-rust/pipeline-jobs/agent_large_orchestrator.rs
  AES021 AGENT_ROLE: Agent file >300 lines (found: 358).
  WHY? Large agent files violate the Single Responsibility Principle.
  FIX: Delegate logic to capabilities or infrastructure.

AES021 HIGH - src-rust/pipeline-jobs/agent_stateful_orchestrator.rs
  AES021 AGENT_ROLE: Orchestrator contains mutable state — violates stateless mandate.
  WHY? Orchestrators are 'Conductors' and must not hold internal execution state.
  FIX: Move state to a Registry component.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Agent file > 300 lines | `check_agent_role()` runs | AES021 HIGH flagged | Pending Review |
| AC-002 | Agent file ≤ 300 lines | `check_agent_role()` runs | No AES021 | Pending Review |
| AC-003 | Container with domain logic | Full checker runs | AES021 flagged | Pending Review Dead code |
| AC-004 | Orchestrator with mutable state | Full checker runs | AES021 flagged | Pending Review Dead code |
| AC-005 | File without agent_ prefix | Active checker runs | Skipped | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Two Implementations — Only One Active

#### 8.1.1 Line-Count Checker (ACTIVE) — `lint_checking_coordinator.rs:449-462`
```rust
fn check_agent_role(file: &str, content: &str, violations: &mut Vec<LintResult>) {
    if !file.starts_with("agent_") { return; }
    if content.lines().count() > 300 {
        violations.push(Self::mk(file, 0, "AES021", Severity::HIGH, "..."));
    }
}
```
**Status**: Pending Review Active, called in `run_all_checks()` line 61.

#### 8.1.2 ArchRoleChecker (DEAD CODE) — `src-rust/role-rules/capabilities_role_checker.rs:30-646`
- `check_agent_roles()` — 320+ lines, 6 sub-role checks
- Detects state, domain logic, control flow count, contract implementation, etc.
- **Status**: Pending Review **Never called** by any active code path. Implements `IRoleCheckerProtocol` but the trait is never invoked.

### 8.2 Bugs Found

1. **CATASTROPHIC: ArchRoleChecker is dead code** (`src-rust/role-rules/capabilities_role_checker.rs:30-646`)
   - Full implementation exists (320+ lines, 6 role-specific checker methods)
   - `check_agent_roles()` is the entry point but has zero callers
   - **Impact**: 13 AES021 violation messages in YAML config are unreachable
   - **Fix**: wire `ArchRoleChecker` into the coordinator pipeline

2. **Active check only measures line count**
   - `lint_checking_coordinator.rs:449-462` only checks `content.lines().count() > 300`
   - Does NOT check: statelessness, domain logic, contract implementation, control flow
   - **Impact**: stateful orchestrators and logic-heavy containers pass undetected

### 8.3 What Needs to Be Added
- **Wire ArchRoleChecker into coordinator**: call `check_agent_roles()` from `run_all_checks()` or the second loop
- **Test fixtures**: verify full role-based checks detect stateful containers, orchestrators, etc.

### 8.4 What to Keep
- **Line-count threshold check** Pending Review (simple and effective)
- **ArchRoleChecker implementation** Pending Review (sophisticated logic, just needs wiring)
- **YAML config structure** Pending Review (per-role violation messages are already correct)

### 8.5 Empirical Evidence from Test Projects
- `test-project-rust/src-rust/pipeline-jobs/agent_large_orchestrator.rs` — 358 lines → flagged AES021 Pending Review
- `test-project-rust/src-rust/pipeline-jobs/agent_stateful_orchestrator.rs` — mutable `counter` field → NOT flagged Pending Review
- `test-project-python/src-python/pipeline-jobs/agent_stateful.py` — `self.state`, `self.executed` → NOT flagged Pending Review

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| Config YAML | Per-role behavioral flags | 13 violation messages unreachable | Wire ArchRoleChecker |
| FR-001 (Layer detection) | Agent role suffix detection | Role misclassified | Existing layer tests |
| FR-003 (AST parsing) | State detection, control flow | ArchRoleChecker uses regex | Acceptable for now |

## 10. Appendices
- `src-rust/pipeline-jobs/agent_checking_coordinator.rs:449` — Active line-count checker
- `src-rust/role-rules/capabilities_role_checker.rs:30` — Dead-code full checker
- `src-rust/role-rules/capabilities_role_checker.rs:261` — `_check_stateless_execution()`
- `src-rust/role-rules/capabilities_role_checker.rs:314` — `_check_high_level_policy_only()`
- `src-rust/role-rules/capabilities_role_checker.rs:352` — `_check_coordinates_multiple_orchestrators()`
- `src-rust/role-rules/capabilities_role_checker.rs:426` — `_check_no_domain_logic()`
- `src-rust/role-rules/capabilities_role_checker.rs:117` — `_check_must_implement_contract_lazy()`
- `src-rust/role-rules/capabilities_role_checker.rs:466` — `_check_lazy_eager_init_only()`
- `lint_arwaky.config.rust.yaml:333` — Agent_Container_Wiring_Mandate config
- `lint_arwaky.config.rust.yaml:351` — Agent_Orchestrator_Stateless_Mandate config
- `lint_arwaky.config.rust.yaml:369` — Agent_Coordinator_Strategic_Mandate config
- `lint_arwaky.config.rust.yaml:382` — Agent_Registry_Inventory_Mandate config
- `lint_arwaky.config.rust.yaml:400` — Agent_Manager_Lifecycle_Mandate config
