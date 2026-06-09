# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run MyPy (FR-074)  
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
This document defines the external tool adapter that runs `mypy` on Python source files for static type checking. The adapter invokes MyPy via `ICommandExecutorPort` and parses its line-based output into `LintResult` entries.

### 2.2 Scope
**In-Scope:**
- Invoking `mypy --show-error-codes <file>` on Python files
- Parsing MyPy output with regex for file, line, column, error code, and message
- Mapping MyPy error types to AES severity levels
- Configurable binary path via `bin_path`

**Out-of-Scope:**
- Running MyPy in daemon mode (`mypy daemon`)
- Type-checking stub files (`.pyi`) exclusively
- Generating MyPy configuration

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **MyPyAdapter** | Infrastructure struct in `infrastructure_py_mypy.rs` implementing `ILinterAdapterPort` |
| **mypy** | Optional static type checker for Python |
| **ILinterAdapterPort** | Contract trait for external linter adapters |

## 3. Feature Overview
### 3.1 Background & Problem
Python type annotations are increasingly common, and MyPy is the de facto type checker. Lint Arwaky needs a MyPy adapter so type errors are caught during lint scans alongside style and security checks.

### 3.2 Business Goals
- Surface MyPy type errors in unified scan output
- Support per-file type checking
- Provide structured error details (error code, line, column)

### 3.3 Target Users
- **Python Developers**: Catch type errors during lint scans
- **DevOps/CI**: Enforce type safety in Python projects

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Python developer, I want MyPy type errors reported when I run `scan`, so I catch type mismatches early.
- **US-002:** As a CI maintainer, I want type errors to fail the scan, so typed code stays correct.

### 4.2 Use Cases & Workflow
```
Input: scan /project (Python project)

1. MyPyAdapter::scan() called with file path
2. Skip if file extension is not .py
3. Build command: mypy --show-error-codes <file>
4. Execute via ICommandExecutorPort
5. Parse output with regex → extract line/column/code/message
6. Return Vec<LintResult>
```

### 4.3 Business Rules
- Severity: HIGH for error-type messages, MEDIUM for notes
- Regex pattern: captures `file:line:col: error_type: message [error-code]`
- Configurable `bin_path`: defaults to `"mypy"`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Parse MyPy output per file | < 500ms |
| NFR-002 | Regex extraction accuracy | 100% of standard MyPy output lines captured |

## 6. UI/UX Requirements
```
AES074 HIGH - src/services/user.py:25
  mypy: arg-type — Argument 1 to "create_user" has incompatible type "str"; expected "int"

AES074 MEDIUM - src/models/base.py:12
  mypy: misc — Cannot determine type of "id" in untyped function
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Python file with type errors | Scan runs | AES074 entries reported | Pending Review |
| AC-002 | Python file with no type errors | Scan runs | No AES074 entries | Pending Review |
| AC-003 | MyPy not installed | Scan runs | Graceful error handling | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_py_mypy.rs` (182 lines). Output parsing uses two regex patterns: one for standard errors (`file:line:col: error: message`) and one for MyPy's extended format with error codes. The `map_severity()` function differentiates error-type messages from notes/warnings.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired in orchestrator | Register via ServiceContainerAggregate |
| mypy | External Python tool | Not installed | Document as prerequisite; graceful skip |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_py_mypy.rs` — MyPyAdapter implementation
- `src-rust/language-adapters/mod.rs:25` — Module export
- `src-rust/di-containers/contract_service_aggregate.rs` — ILinterAdapterPort trait
