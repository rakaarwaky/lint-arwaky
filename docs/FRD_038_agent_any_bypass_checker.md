# Feature Requirements Document (FRD)
**Feature Name:** Agent Any-Bypass Detector (AES024)  
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
This document defines the AES024 rule that detects forbidden type-escape patterns in agent orchestrator code. The rule has two distinct implementations: (1) an **active wildcard-import checker** that detects `::*` patterns in agent files, and (2) a **dead-code `any`-type checker** in `ArchRoleChecker` that detects `Any`, `any`, and raw pointer type annotations.

### 2.2 Scope
**In-Scope (active):**
- Wildcard imports (`use crate::capabilities::*;`) in files with `agent_` prefix
- HIGH severity reporting

**In-Scope (dead code):**
- `any` type annotations in orchestrator method signatures and field declarations
- Raw pointer type usage

**Out-of-Scope:**
- Dynamic language features (Python duck typing)
- Runtime type introspection patterns
- Auto-fixing violations

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES024** | Rule code for agent any-bypass violation |
| **check_agent_any_bypass()** | Active wildcard-import checker in `lint_checking_coordinator` |
| **_check_forbid_any_type()** | Dead-code any-type checker in `ArchRoleChecker` |
| **Wildcard import** | Import using `*` syntax (e.g., `use crate::capabilities::*` |

## 3. Feature Overview
### 3.1 Background & Problem
The agent orchestrator is the system's brain and must maintain strict type safety. Wildcard imports bypass explicit type resolution by pulling in all symbols from a module. The `any` type annotation (TypeScript's `any`, Python's `Any`, Rust's raw pointers) bypasses the type system entirely. Both patterns degrade architectural integrity and should be forbidden in orchestrator code.

### 3.2 Business Goals
- Prevent wildcard imports in agent code that obscure dependency relationships
- Eliminate `any` type annotations that bypass compile-time type checking
- Enforce explicit, type-safe code in the orchestration layer

### 3.3 Target Users
- **Developers**: Get warned when using wildcard imports or `any` types in agent files
- **Architects**: Maintain type safety guarantees in the orchestration layer

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when I use a wildcard import in an agent file.
- **US-002:** As a developer, I want to be warned when I use `any` type annotations in an orchestrator.
- **US-003:** As an architect, I want the rule toggleable via YAML `forbid_any_type` flag.

### 4.2 Use Cases & Workflow
**Active Pipeline (wildcard imports):**
```
File: pipeline-jobs/agent_wildcard_orchestrator.rs

1. Does filename have agent_ prefix? → YES
2. For each line:
   a. Does line contain ":*:" or "::* }"? → YES (wildcard)
   b. Flag AES024 HIGH
```

**Dead-Code Pipeline (any types — ArchRoleChecker):**
```
File: pipeline-jobs/agent_any_mess.py

1. Does filename have agent_ prefix? → YES
2. Is suffix orchestrator? → YES
3. Load LayerDefinition → forbid_any_type: true
4. Scan for patterns: ": Any", "-> Any", "Any["
5. If found → AES024 HIGH
```

### 4.3 Business Rules
- Severity: HIGH
- Active rule: wildcard imports (`::*`) detected in any file with `agent_` prefix
- Dead-code rule: `any` type annotations in orchestrator scope
- Active rule is scope-independent (checks all agent files regardless of role)
- Dead-code rule uses config `forbid_any_type` per role scope

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |
| NFR-002 | False positive rate | 0% for valid agent files |
| NFR-003 | False negative rate | 0% for violating agent files |

## 6. UI/UX Requirements
```
AES024 HIGH - src-rust/pipeline-jobs/agent_wildcard_orchestrator.rs
  AES024 AGENT_ANY_BYPASS: Wildcard import detected in agent layer.
  WHY? Wildcard imports bypass explicit type resolution in orchestrator code.
  FIX: Import specific types explicitly instead of using *.

AES024 HIGH - src-rust/pipeline-jobs/agent_any_mess.py
  AES024 AGENT_ANY_BYPASS: 'any' type annotation detected in orchestrator logic.
  WHY? Using 'any' bypasses the type system and hides architectural coupling.
  FIX: Replace with a concrete domain type or interface.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Agent file with `::*` wildcard | `check_agent_any_bypass()` runs | AES024 HIGH flagged | Pending Review |
| AC-002 | Agent file without wildcard | `check_agent_any_bypass()` runs | No AES024 | Pending Review |
| AC-003 | Python agent with `: Any` annotation | Full checker runs | AES024 flagged | Pending Review Dead code |
| AC-004 | JS agent with `: any` annotation | Full checker runs | AES024 flagged | Pending Review Dead code |
| AC-005 | File without agent_ prefix | Checker runs | Skipped | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Two Implementations — Checking Different Things

#### 8.1.1 Wildcard Import Checker (ACTIVE) — `lint_checking_coordinator.rs:344-361`
```rust
fn check_agent_any_bypass(file: &str, content: &str, violations: &mut Vec<LintResult>) {
    if !file.starts_with("agent_") { return; }
    let wc1 = format!("{}*{}", ":", ":");
    let wc2 = format!("{}* {}", "::", "}");
    if line.trim().contains(&wc1) || line.trim().contains(&wc2) {
        violations.push(Self::mk(file, i + 1, "AES024", Severity::HIGH, "..."));
    }
}
```
**Status**: Pending Review Active. Checks for `::*` patterns (wildcard imports). Called in `run_all_checks()` line 64.

#### 8.1.2 Any-Type Checker (DEAD CODE) — `src-rust/role-rules/capabilities_role_checker.rs:576-612`
```rust
fn _check_forbid_any_type(&self, ...) {
    // Scans for ": Any", "-> Any", "Any[" patterns
    // Called from _apply_agent_role_checks (lines 112-114)
    // Entire chain is dead code
}
```
**Status**: Pending Review **Never called**. The entire `check_agent_roles()` → `_apply_agent_role_checks()` → `_check_forbid_any_type()` chain is unreachable.

### 8.2 Bugs Found

1. **Check mismatch** — active check is wildcard imports, docs say `any` types
   - Config violation message (line 365): "`any` type (or raw pointers) detected in the agent orchestrator logic"
   - Active code (line 344): detects `::*` wildcard imports only
   - **Impact**: documentation/UX doesn't match actual behavior

2. **`_check_forbid_any_type()` is dead code** (`src-rust/role-rules/capabilities_role_checker.rs:576-612`)
   - Correctly detects `Any` type annotations via regex
   - Called from `_apply_agent_role_checks` which is called from `check_agent_roles` — never invoked
   - **Impact**: Python/JS test fixtures with `: Any` go undetected

3. **Wildcard checker is heuristic-based**
   - `format!("{}*{}", ":", ":")` produces `:*:` which catches `use crate::foo::*;` but also may match false positives like comments containing `:*:`
   - `format!("{}* {}", "::", "}")` produces `::* }` which may miss `use crate::foo::* ;` (space before semicolon)
   - **Impact**: fragile pattern matching

### 8.3 What Needs to Be Added
- **Wire `_check_forbid_any_type`** by activating `ArchRoleChecker` in the coordinator pipeline
- **Fix wildcard regex** to properly target `use` statements only
- **Add test fixtures** for `any` type annotations in orchestrator files

### 8.4 What to Keep
- **Active wildcard import checker** Pending Review (catches real violations)
- **`_check_forbid_any_type` implementation** Pending Review (just needs wiring)
- **Config YAML `forbid_any_type` flag** Pending Review (correct structure)

### 8.5 Empirical Evidence from Test Projects
- `test-project-rust/src-rust/pipeline-jobs/agent_wildcard_orchestrator.rs` — `use crate::capabilities::*;` → flagged AES024 Pending Review
- `test-project-python/src-python/pipeline-jobs/agent_any_mess.py` — `data: Any`, `event: Any` → NOT flagged Pending Review
- `test-project-javascript/src-javascript/pipeline-jobs/agent_any_mess.ts` — `const violatingAnyVar: any` → NOT flagged Pending Review

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| Config YAML `forbid_any_type` | Enable/disable flag per scope | Flag not consumed by active checker | Wire to ArchRoleChecker |
| FR-003 (AST parsing) | Type annotation detection | Heuristic pattern matching | Improve regex precision |
| ArchRoleChecker | Full checker circuit | Dead code | Wire into coordinator |

## 10. Appendices
- `src-rust/pipeline-jobs/agent_checking_coordinator.rs:344` — Active wildcard checker
- `src-rust/role-rules/capabilities_role_checker.rs:576` — Dead-code any-type checker
- `lint_arwaky.config.rust.yaml:355` — `forbid_any_type` config for orchestrators
- `test-project-rust/src-rust/pipeline-jobs/agent_wildcard_orchestrator.rs` — Wildcard fixture
- `test-project-python/src-python/pipeline-jobs/agent_any_mess.py` — Any-type fixture
