# Feature Requirements Document (FRD)
**Feature Name:** Dead Inheritance Bypass Detector (AES016)
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
This document defines the AES016 rule that detects **dead inheritance bypass patterns** — unit structs (declared with `;` instead of `{}`) and empty `impl` blocks (`impl … for … {}`). These patterns indicate incomplete design or placeholder code. The rule is implemented in `check_dead_inheritance()`.

### 2.2 Scope
**In-Scope:**
- Unit structs: `struct Foo;` (struct with no fields, terminated by semicolon)
- Empty impl blocks: `impl Foo for Bar {}` (no methods declared)
- MEDIUM severity

**Out-of-Scope:**
- Auto-fixing violations
- Empty enums or unions
- Traits without methods

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES016** | Rule code for dead inheritance violation |
| **check_dead_inheritance()** | Detection method in `LintCheckingCoordinator` |
| **Unit struct** | Struct declared with `;` (no fields) — likely placeholder |
| **Empty impl block** | `impl … for … {}` with nothing inside |

## 3. Feature Overview
### 3.1 Background & Problem
Unit structs and empty impl blocks are often left behind during refactoring or created as placeholders that never get filled in. They bypass the architectural requirement that every struct must have meaningful fields and every trait implementation must provide concrete behavior.

### 3.2 Business Goals
- Eliminate unit structs that are architectural fragments
- Enforce that every `impl` block contains at least one method
- Keep the codebase free of dead/incomplete patterns

### 3.3 Target Users
- **Developers**: Get notified when creating incomplete structs or empty trait implementations
- **Reviewers**: Detect dead code patterns during code review

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when I declare a struct with `;` (unit struct).
- **US-002:** As a developer, I want to be warned when I write an empty `impl` block.

### 4.2 Detection Pipeline
```
File: src-rust/capabilities/some_checker.rs

1. For each line:
   a. Does line match `struct NAME;` pattern?
      → AES016 MEDIUM (unit struct)
   b. Does line match `impl ... for ... {}` pattern?
      → AES016 MEDIUM (empty impl)
```

### 4.3 Business Rules
- Severity: MEDIUM
- Rust only (Python/JS inheritance patterns not checked)
- Line-based detection — only single-line patterns matched
- Tuple structs with `()` are NOT flagged (e.g., `struct Foo(u32);` is valid)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 2ms |
| NFR-002 | False positive rate | 0% for legitimate patterns |
| NFR-003 | False negative rate | Minimal — multi-line patterns missable |

## 6. UI/UX Requirements
```
AES016 MEDIUM - src-rust/capabilities/some_checker.rs:15
  AES016 DEAD_INHERITANCE: Unit struct — possibly dead inheritance.
  WHY? Unit structs (terminated with ;) indicate incomplete design.
  FIX: Remove if unused, or replace with a proper struct definition.

AES016 MEDIUM - src-rust/capabilities/some_checker.rs:42
  AES016 DEAD_INHERITANCE: Empty impl block.
  WHY? Empty impl blocks are placeholders that bypass architectural intent.
  FIX: Remove or implement at least one method.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Rust file with `struct Foo;` | `check_dead_inheritance()` runs | AES016 flagged | ✅ |
| AC-002 | Rust file with `impl Trait for Foo {}` | Checker runs | AES016 flagged | ✅ |
| AC-003 | Rust file with `struct Foo { x: u32 }` | Checker runs | No AES016 | ✅ |
| AC-004 | Rust file with `impl Trait for Foo { fn bar() {} }` | Checker runs | No AES016 | ✅ |
| AC-005 | Multi-line empty impl block | Checker runs | AES016 flagged | ❌ Multi-line not supported |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location**: `lint_checking_coordinator.rs:320-342`
- **Status**: **PARTIALLY IMPLEMENTED** — single-line patterns only
- Invoked from `run_all_checks()` line 60

### 8.2 Bugs/Gaps Found

1. **Multi-line impl block not supported** — If `impl Trait for Foo` is on one line and `{}` is on the next, it's missed
2. **No struct field check** — Only checks `ends_with(';')` which catches unit structs but also edge cases like macros ending in `;`

### 8.3 What Needs to Be Added
- Multi-line empty impl block detection (match `impl` + `for` + next line `{}`)
- Proper AST-like matching to avoid false positives from macro invocations

### 8.4 What to Keep
- Single-line struct detection ✅
- Single-line impl detection ✅
- MEDIUM severity ✅

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Line scanning) | Line-based regex-free detection | Multi-line patterns missed | Add multi-line support |

## 10. Appendices
- `src-rust/agent/lint_checking_coordinator.rs:320` — `check_dead_inheritance()`
