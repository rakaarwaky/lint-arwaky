# Feature Requirements Document (FRD)
**Feature Name:** Missing VO Construction Detector (AES032)  
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
This document defines the AES032 rule that detects capability method calls that are missing required request/data Value Object parameters. The rule is implemented in `_check_missing_vo_construction()` within `DispatchRoutingChecker` in `cli-transport/capabilities_routing_processor.rs`, with a secondary string/numeric literal detector in `check_missing_vo()` in `agent_checking_coordinator.rs`. AES032 ensures that every capability method invocation passes a typed Value Object argument rather than raw strings or numbers.

### 2.2 Scope
**In-Scope:**
- Analysis of Python capability method calls for missing arguments (empty parens)
- Detection of direct string literal assignments in capability/infrastructure files
- Detection of direct numeric literal assignments in capability/infrastructure files
- MEDIUM severity reporting

**Out-of-Scope:**
- Deep type checking of VO parameter types (e.g., checking if the VO is the correct type)
- Detection of missing VO in non-capability layers (taxonomy, surface, agent)
- Auto-fixing calls to insert VO parameters

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES032** | Rule code for missing VO construction violation |
| **_check_missing_vo_construction()** | Call-argument checker in `capabilities_routing_processor.rs` |
| **check_missing_vo()** | Literal-value checker in `agent_checking_coordinator.rs` |
| **Value Object (VO)** | Typed domain object that encapsulates a concept with validation |
| **MethodArgsVO** | Helper struct for extracting arguments between parentheses |

## 3. Feature Overview
### 3.1 Background & Problem
Capability methods in the AES architecture expect typed Value Object arguments (e.g., `ProcessOrderRequest`) rather than raw strings, numbers, or empty argument lists. Calling `await executor.process_order()` instead of `await executor.process_order(request_vo)` bypasses domain validation and couples the dispatch layer to raw data formats. This anti-pattern makes the system fragile to changes in domain logic and allows invalid data to propagate deeper into the system.

### 3.2 Business Goals
- Enforce Value Object usage in all capability method calls
- Prevent raw string/numeric data from entering the capability layer
- Detect empty argument lists that indicate skipped domain validation
- Provide clear messages identifying the specific call site

### 3.3 Target Users
- **Developers**: Get warned when they call a capability method without a VO parameter or with raw literals
- **Architects**: Maintain domain encapsulation by enforcing VO gatekeeping at capability boundaries

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when I call `self.some_executor.method()` without any arguments (missing VO parameter).
- **US-002:** As a developer, I want to be warned when I assign a raw string literal to a variable in capability code (should be a VO).
- **US-003:** As a developer, I want to be warned when I assign a raw numeric literal to a variable in capability code.

### 4.2 Use Cases & Workflow
**Primary Pipeline (VO call checker — Python):**
```
File: capabilities/order_executor.py

1. Content ends with ".py"? → YES
2. For each line, extract method calls: (?:await\s+)?self\.\w+\.(\w+)\s*\(
3. Extract arguments between parentheses with brace-depth parsing
4. If arguments are empty → AES032 MEDIUM flagged
```

**Secondary Pipeline (literal checker — all files):**
```
File: capabilities/order_executor.rs

1. Is layer == "capabilities" or "infrastructure"? → YES
2. For each line:
   a. Does line match "let X = 'string_literal'" pattern?
      → YES → AES032 MEDIUM: "Direct string literal"
   b. Does line match "let X = 12345" pattern (numeric)?
      → YES → AES032 MEDIUM: "Direct numeric literal"
```

### 4.3 Business Rules
- Severity: MEDIUM
- Primary checker: Python-only, detects empty argument parens on `self.some_executor.method()`
- Secondary checker: All languages, detects raw string/numeric literals in assignments
- Secondary checker only runs on `capabilities` and `infrastructure` layers
- Barrel files and non-capability layers are skipped

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |
| NFR-002 | False positive rate | < 2% (may flag OK patterns like `await self.log.info("done")`) |
| NFR-003 | False negative rate | 0% for empty paren calls |

## 6. UI/UX Requirements
```
AES032 MEDIUM - src-rust/cli-transport/capabilities/order_executor.py
  AES032 MISSING_VO_CONSTRUCTION: Capability call 'self.order_executor.process_order()' missing required request/data VO parameter. Capability methods expect a typed Value Object argument.
  WHY? Capability methods expect typed VO arguments. Calling without them bypasses domain validation and creates runtime fragility.
  FIX: Pass the appropriate request or data VO to every capability method call.

AES032 MEDIUM - src-rust/infrastructure/order_repository.rs
  AES032 MISSING_VO: Direct string literal.
  WHY? Raw string literals in capability/infrastructure code bypass VO-based domain validation.
  FIX: Create a proper Value Object for the data instead of using a raw string.

AES032 MEDIUM - src-rust/capabilities/payment_handler.rs
  AES032 MISSING_VO: Direct numeric literal.
  WHY? Raw numeric literals in capability/infrastructure code bypass VO-based domain validation.
  FIX: Create a proper Value Object for the data instead of using a raw number.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Python capability file with `await self.exec.process_order()` | `_check_file_vo_construction()` runs | AES032 MEDIUM flagged | Pending Review |
| AC-002 | Python capability file with `await self.exec.process_order(request_vo)` | `_check_file_vo_construction()` runs | No AES032 | Pending Review |
| AC-003 | Capability file with `let name = "Raka"` literal | `check_missing_vo()` runs | AES032 MEDIUM flagged | Pending Review |
| AC-004 | Capability file with `let count = 42` numeric literal | `check_missing_vo()` runs | AES032 MEDIUM flagged | Pending Review |
| AC-005 | Taxonomy file with string literal | `check_missing_vo()` runs | Skipped (not cap/infra layer) | Pending Review |
| AC-006 | Non-Python file with capability calls | `_check_file_vo_construction()` runs | Skipped (Python-only) | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location (VO call)**: `src-rust/cli-transport/capabilities_routing_processor.rs:354-435`
- **Location (literal)**: `src-rust/code-analysis/agent_checking_coordinator.rs:589-617`
- **Status**: **FULLY IMPLEMENTED** — both paths are active
- VO call checker uses regex `(?:await\s+)?self\.\w+\.(\w+)\s*\(` and brace-depth argument parsing
- Literal checker matches `let X = "string"` and `let X = 123` patterns in cap/infra layers

### 8.2 Bugs Found

1. **VO call checker is Python-only** (`routing_processor.rs:372`)
   - `if !path.ends_with(".py") { return; }`
   - Rust capability calls like `executor.process_order()` are NOT checked
   - **Impact**: Rust projects get zero AES032 from the call checker
   - **Fix**: add Rust method call analysis (parse `\.\w+\(` patterns)

2. **Literal checker is heuristic-based** (`agent_checking_coordinator.rs:597-613`)
   - Matches `let X = "string"` but not `let X = format!(...)`, `let X = some_var`, `let X = vec![...]`
   - Does not check function call arguments, struct fields, or return values
   - **Impact**: many real violations go undetected (false negatives)
   - **Fix**: implement proper AST-based analysis or expand heuristic patterns

3. **Literal checker does not exclude test files**
   - Test fixtures often use string literals for assertions
   - **Impact**: false positives in test files
   - **Fix**: skip `_test.rs`, `_spec.rs`, `test_*.py` files or add an exceptions list

4. **Call checker regex is narrow** (`routing_processor.rs:381`)
   - Only matches `self.some_executor.method()` pattern
   - Does NOT match: `executor.method()`, `self.method()`, `cap.method(args)`
   - **Impact**: many capability call patterns bypass detection
   - **Fix**: broaden regex to match any method call on a capability-target variable

### 8.3 What Needs to Be Added
- **Rust call analysis**: parse `.method_name(` patterns in .rs files for the call checker
- **AST-based literal detection**: replace heuristic with proper AST traversal
- **Test file exclusion**: add filter for test files in the literal checker
- **Broader call regex**: match more call patterns beyond `self.some_executor.method()`

### 8.4 What to Keep
- **Brace-depth argument extraction** Pending Review (line 413-435, correctly handles nested parens)
- **Cap/Infra layer gate** Pending Review (line 590-593, correct scope)
- **Integration with routing processor pipeline** Pending Review (line 134-136)

### 8.5 Empirical Evidence from Test Projects
- `test-project-python/src-python/capabilities/` — No existing fixture with empty method calls found
- `test-project-rust/src-rust/capabilities/` — No existing fixture with raw literals found
- **No test fixture exercises AES032** — needs to be created

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Method call extraction | Python-only regex for call checker | Extend to Rust call patterns |
| capabilities_routing_processor | Shared with AES030/AES031 | Changes affect 3 rules | Isolate per-rule tests |
| Config YAML | Severity/MEDIUM | Not configurable | Add `severity` field |
| Test fixtures | Empty-method-call and raw-literal fixtures | None exist | Create test fixtures per language |

## 10. Appendices
- `src-rust/cli-transport/capabilities_routing_processor.rs:354` — `_check_missing_vo_construction()`
- `src-rust/cli-transport/capabilities_routing_processor.rs:365` — `_check_file_vo_construction()`
- `src-rust/cli-transport/capabilities_routing_processor.rs:413` — `_extract_args()`
- `src-rust/code-analysis/agent_checking_coordinator.rs:589` — `check_missing_vo()`
- `lint_arwaky.config.rust.yaml:260` — AES032 config message
