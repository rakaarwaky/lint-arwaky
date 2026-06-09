# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run Radon (FR-076)  
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
This document defines the external tool adapter that runs `radon cc` on Python source files for cyclomatic complexity analysis. The adapter invokes Radon via `ICommandExecutorPort` and reports functions/methods exceeding configurable complexity thresholds.

### 2.2 Scope
**In-Scope:**
- Invoking `radon cc --json <file>` on Python files
- Parsing Radon JSON output for complexity scores per function/method
- Flagging functions exceeding the threshold as LintResult entries
- Configurable complexity threshold (Count) and binary path

**Out-of-Scope:**
- Radon's `raw` metrics (LOC, comments, etc.)
- Radon's `hal` (Halstead complexity) analysis
- Auto-fixing complex functions

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **ComplexityAdapter** | Infrastructure struct in `infrastructure_py_analysis.rs` implementing `ILinterAdapterPort` |
| **radon cc** | Radon command for cyclomatic complexity computation |
| **Cyclomatic complexity** | Metric measuring number of independent paths through source code |

## 3. Feature Overview
### 3.1 Background & Problem
High cyclomatic complexity makes Python code hard to test and maintain. Radon computes complexity metrics for Python. Lint Arwaky integrates Radon to surface overly complex functions during scans, enforcing a configurable complexity threshold.

### 3.2 Business Goals
- Flag functions exceeding complexity threshold
- Support threshold configuration per project via YAML
- Integrate via `ILinterAdapterPort` for consistent tool adapter pattern

### 3.3 Target Users
- **Python Developers**: Get complexity warnings for overly complex functions
- **Engineering Managers**: Enforce complexity standards across Python projects

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Python developer, I want complex functions flagged when I run `scan`, so I know which methods to refactor.
- **US-002:** As an architect, I want the complexity threshold configurable via YAML (`threshold` field).

### 4.2 Use Cases & Workflow
```
Input: scan /project (Python project)

1. ComplexityAdapter::scan() called with file path
2. Build command: radon cc --json <file>
3. Execute via ICommandExecutorPort
4. Parse JSON output → extract per-function complexity
5. Compare each score against threshold
6. Flag functions exceeding threshold as LintResult (HIGH)
```

### 4.3 Business Rules
- Default threshold: 10 (moderate complexity)
- Severity: HIGH for any function exceeding threshold
- Reports complexity score, function name, and line number
- Configurable `threshold` via YAML in `ComplexityAdapter` constructor

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Per-file Radon analysis | < 500ms |
| NFR-002 | Configurable threshold | Read from `Count` value passed at construction |

## 6. UI/UX Requirements
```
AES076 HIGH - src/services/payment.py:45
  radon: process_refund — Cyclomatic complexity 15 exceeds threshold 10

AES076 HIGH - src/utils/parser.py:120
  radon: parse_document — Cyclomatic complexity 22 exceeds threshold 10
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Python file with complex functions | Scan runs | AES076 flagged for each complex function | Pending Review |
| AC-002 | Python file with all functions under threshold | Scan runs | No AES076 entries | Pending Review |
| AC-003 | Threshold configured to 15 | Scan runs | Only functions with complexity > 15 flagged | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_py_analysis.rs` (185 lines) as `ComplexityAdapter`. The struct holds a `_threshold: Count` field but the `scan()` method is currently a stub returning `Ok(LintResultList::default())`. The `name()` method returns `AdapterName::raw("radon")`.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired in orchestrator | Register via ServiceContainerAggregate |
| radon | External Python tool | Not installed | Document as prerequisite; graceful skip |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_py_analysis.rs:18` — ComplexityAdapter struct
- `src-rust/language-adapters/mod.rs:17` — Module export
- `src-rust/di-containers/contract_service_aggregate.rs` — ILinterAdapterPort trait
