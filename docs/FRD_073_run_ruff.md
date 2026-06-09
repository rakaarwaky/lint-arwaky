# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run Ruff (FR-073)  
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
This document defines the external tool adapter that runs `ruff check` on Python source files. Ruff is a fast Python linter written in Rust. The adapter invokes Ruff via `ICommandExecutorPort` and parses its JSON output into `LintResult` entries.

### 2.2 Scope
**In-Scope:**
- Invoking `ruff check --output-format=json` on Python files
- Parsing Ruff JSON output for lint violations
- Mapping Ruff severity to AES severity levels
- Configurable binary path via `bin_path`

**Out-of-Scope:**
- Running Ruff formatter (`ruff format`)
- Auto-fixing violations
- Installing or upgrading Ruff

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **RuffAdapter** | Infrastructure struct in `infrastructure_py_ruff.rs` implementing `ILinterAdapterPort` |
| **ruff check** | Ruff's lint command — checks Python files for rule violations |
| **ICommandExecutorPort** | Contract trait for executing external commands |

## 3. Feature Overview
### 3.1 Background & Problem
Python projects need fast linting. Ruff is the modern replacement for Flake8, pylint, and isort. Lint Arwaky integrates Ruff as a Python language adapter so Python scans include Ruff diagnostics alongside AES rules.

### 3.2 Business Goals
- Surface Ruff lint violations in unified scan output
- Support configurable binary path for CI environments
- Integrate via `ILinterAdapterPort` for consistent tool adapter pattern

### 3.3 Target Users
- **Python Developers**: Get Ruff feedback during `lint-arwaky scan`
- **DevOps/CI**: Include Ruff in automated quality gates for Python projects

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Python developer, I want Ruff violations reported when I run `scan`, so I see lint issues alongside AES rules.
- **US-002:** As a project maintainer, I want to configure a custom Ruff binary path via YAML.

### 4.2 Use Cases & Workflow
```
Input: scan /project (Python project)

1. RuffAdapter::scan() called with file path
2. Skip if file extension is not .py
3. Build command: ruff check --output-format=json <file>
4. Execute via ICommandExecutorPort
5. Parse JSON output → Vec<LintResult>
6. Return LintResultList
```

### 4.3 Business Rules
- Severity mapping: Ruff `E`/`F` errors → HIGH, all others → MEDIUM
- Timeout: 60 seconds
- Configurable `bin_path`: if set, use custom binary instead of `ruff`
- Only `.py` files are checked

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Parse Ruff JSON per file | < 200ms |
| NFR-002 | False negative rate | 0% — all Ruff diagnostics captured |

## 6. UI/UX Requirements
```
AES073 HIGH - src/app.py:42
  ruff: F841 — Local variable 'unused_var' is assigned to but never used.

AES073 MEDIUM - src/utils.py:15
  ruff: E501 — Line too long (92 > 79 characters)
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Python project with Ruff violations | Scan runs | Ruff violations as AES073 entries | Pending Review |
| AC-002 | Python project with no violations | Scan runs | No AES073 entries | Pending Review |
| AC-003 | Ruff not installed | Scan runs | Graceful error, no crash | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_py_ruff.rs` (163 lines). It parses Ruff's JSON output with `serde_json::Value`. The `resolve_executable()` method returns a custom binary path if configured, or defaults to `"ruff"`. The `map_severity()` method converts Ruff severity strings to AES `Severity` enum values.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired in orchestrator | Register via ServiceContainerAggregate |
| ruff | External Python tool | Not installed | Document as prerequisite; graceful skip |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_py_ruff.rs` — RuffAdapter implementation
- `src-rust/language-adapters/mod.rs:29` — Module export
- `src-rust/di-containers/contract_service_aggregate.rs` — ILinterAdapterPort trait
