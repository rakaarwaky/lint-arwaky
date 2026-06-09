# 📄 Feature Requirements Document (FRD)
**Feature Name:** Show Enclosing Scope — Scope Analysis
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
Defines the enclosing scope feature that shows the function, class, or module scope containing each violation, providing developers with better code navigation context.

### 2.2 Scope
**In-Scope:** Scope detection for violations, function/class name extraction, scope nesting depth, scope path display.
**Out-of-Scope:** Scope-based filtering, scope violation rules.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Enclosing scope** | The function/class/module that contains a given line |
| **Scope path** | Hierarchical path like `MyModule::MyStruct::my_method` |

## 3. Feature Overview
### 3.1 Background & Problem
Violations showed file and line number but not the surrounding scope (function/class), making it hard to understand context without opening the file.

### 3.2 Business Goals
- Display enclosing function/class for each violation
- Enable scope-based violation grouping
- Reduce context-switching for developers

### 3.3 Target Users
- Developers reviewing lint reports

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to see which function contains a violation, so I can understand the context.

### 4.2 Use Cases & Workflow
```
Input:  violation at file.rs:42
Output: "AES001 at file.rs:42 (fn validate_imports → file.rs:10-85)"
```

### 4.3 Business Rules
- Scopes are identified by scanning for `fn`, `impl`, `struct`, `class`, `def` keywords
- Nesting: outermost scope first, innermost last
- Scopes displayed as a path: `Module::Struct::method`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Scope lookup per violation | < 5ms |

## 6. UI/UX Requirements
```
 AES001 | layer-rules/capabilities_checker.rs:42
        │ Containing scope: layer_rules::ImportChecker::check_imports (line 10-85)
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Violation inside function | Lint report generated | Scope shows function name | Pending Review |
| AC-002 | Violation inside nested scope | Lint report generated | Full scope path shown | Pending Review |
| AC-003 | Violation at module level | Lint report generated | "module level" scope shown | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Scope detector | `semantic-analysis/capabilities_scope_detector.rs` | Pending Review |
| Scope reporter | `semantic-analysis/capabilities_scope_reporter.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Requires source parser for scope boundary detection | Regex scope detection may miss edge cases | Line-based heuristic fallback |

## 10. Appendices
- `src-rust/semantic-analysis/` — Feature folder
