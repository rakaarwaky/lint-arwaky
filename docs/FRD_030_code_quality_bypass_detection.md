# Feature Requirements Document (FRD)
**Feature Name:** Bypass Comment Violation Detector (AES014)
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
This document defines the AES014 rule that detects **bypass comment violations** across all layers. Bypass comments (`#[allow(…)]`, `noqa`, `type: ignore`, `unwrap()`, `panic!`) suppress compiler/linter warnings and degrade code quality guarantees. The rule is implemented in `check_bypass_comments()`.

### 2.2 Scope
**In-Scope:**
- `#[allow(...)]` and `#[expect(...)]` attributes in Rust (CRITICAL severity)
- `noqa`, `type: ignore`, `pylint: disable` in Python (CRITICAL)
- `eslint-disable`, `ts-ignore`, `ts-expect-error` in JS/TS (CRITICAL)
- `NOLINT` comments in any language (CRITICAL)

**Out-of-Scope:**
- Auto-fixing violations
- Configuration of bypass patterns
- Runtime introspection patterns

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES014** | Rule code for bypass comment violations |
| **check_bypass_comments()** | Detection method in `LintCheckingCoordinator` |
| **Bypass comment** | Any comment/attribute that disables a linter, type checker, or compiler warning |

## 3. Feature Overview
### 3.1 Background & Problem
Developers use `#[allow(…)]` and bypass comments to silence warnings instead of fixing underlying issues. This erodes code quality guarantees and allows architectural drift to go undetected.

### 3.2 Business Goals
- Eliminate all bypass comments from the codebase
- Enforce strict no-bypass policy for linters, type checkers, and compilers
- Maintain 100% lint/type coverage

### 3.3 Target Users
- **Developers**: Get notified when bypassing lint/type/compiler checks
- **Architects**: Ensure code quality rules are never suppressed

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when I use `#[allow(…)]` in Rust.
- **US-002:** As a developer, I want to be warned when I use `# noqa` in Python.
- **US-003:** As a developer, I want to be warned when I use `// @ts-ignore` in TypeScript.
- **US-004:** As a developer, I want `unwrap()` and `panic!` calls to be flagged.

### 4.2 Detection Pipeline
```
File: src-rust/capabilities/some_checker.rs

1. For each line:
   a. Does line start with #[allow( or #[expect(?
      → AES014 CRITICAL
   b. Does line contain #noqa, #type: ignore, #pylint: disable?
      → AES014 CRITICAL
   c. Does line contain //eslint-disable, //NOLINT?
      → AES014 CRITICAL
   d. Does line contain //@ts-ignore, //@ts-expect-error?
      → AES014 CRITICAL
```

### 4.3 Business Rules
- Severity: CRITICAL (all bypass patterns)
- Rust attributes: `#[allow(]` and `#[expect(]`
- Python comments: `# noqa`, `# type: ignore`, `# pylint: disable`
- JS/TS comments: `// eslint-disable`, `// @ts-ignore`, `// @ts-expect-error`
- Universal: `NOLINT`
- Rule applies to ALL layers without exception

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |
| NFR-002 | False positive rate | 0% for legitimate code |
| NFR-003 | False negative rate | 0% for actual bypass comments |

## 6. UI/UX Requirements
```
AES014 CRITICAL - src-rust/capabilities/some_checker.rs:42
  AES014 BYPASS_COMMENT: Bypass comment detected: #[allow(dead_code)]
  WHY? Suppressing compiler/linter warnings reduces code quality guarantees.
  FIX: Remove the bypass and fix the underlying issue.

AES014 CRITICAL - src-python/module/handler.py:15
  AES014 BYPASS_COMMENT: Bypass comment detected: # type: ignore
  WHY? Suppressing type checker warnings hides real type violations.
  FIX: Fix the type issue instead of ignoring it.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Rust file with `#[allow(dead_code)]` | `check_bypass_comments()` runs | AES014 CRITICAL flagged | ✅ |
| AC-002 | Python file with `# noqa` | Checker runs | AES014 flagged | ✅ |
| AC-003 | TS file with `// @ts-ignore` | Checker runs | AES014 flagged | ✅ |
| AC-004 | Rust file with `unwrap()` call | Checker runs | AES014 flagged | ❌ Not implemented |
| AC-005 | Rust file with `panic!()` call | Checker runs | AES014 flagged | ❌ Not implemented |
| AC-006 | Clean file with no bypass | Checker runs | No AES014 | ✅ |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location**: `lint_checking_coordinator.rs:208-252`
- **Status**: **PARTIALLY IMPLEMENTED**
- Invoked from `run_all_checks()` line 57

### 8.2 Bugs/Gaps Found

1. **`unwrap()` detection missing** — FR-030 mentions `unwrap` as forbidden but `check_bypass_comments` has no pattern for it
2. **`panic!()` detection missing** — FR-030 mentions `panic` but no pattern exists
3. **Duplicate NOLINT entry** — `NOLINT` appears twice in `markers` array (both `("H", "NOLINT")` and `("S", "NOLINT")`)

### 8.3 What Needs to Be Added
- `unwrap()` call detection (Rust: `unwrap()`, `expect()` method calls)
- `panic!()` / `panic()` macro/function detection

### 8.4 What to Keep
- Rust `#[allow(…)]` / `#[expect(…)]` detection ✅
- Python bypass comment detection ✅
- JS/TS bypass comment detection ✅

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (AST Scanning) | Line-level pattern matching | False positives on string literals containing patterns | Case-insensitive matching at line level |

## 10. Appendices
- `src-rust/agent/lint_checking_coordinator.rs:208` — `check_bypass_comments()`
