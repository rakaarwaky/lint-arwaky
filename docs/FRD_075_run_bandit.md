# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run Bandit (FR-075)  
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
This document defines the external tool adapter that runs `bandit -r` on Python source files for security vulnerability scanning. The adapter invokes Bandit via `ICommandExecutorPort` and parses its JSON output into `LintResult` entries.

### 2.2 Scope
**In-Scope:**
- Invoking `bandit -r --format=json <path>` on Python project directories
- Parsing Bandit JSON output for security issues
- Mapping Bandit severity (HIGH, MEDIUM, LOW) to AES severity
- Configurable binary path via `bin_path`

**Out-of-Scope:**
- Running Bandit on non-Python files
- Skipping specific Bandit tests (configure in `.bandit` file)
- Auto-fixing security issues

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **BanditAdapter** | Infrastructure struct in `infrastructure_py_bandit.rs` implementing `ILinterAdapterPort` |
| **bandit** | Security linter for Python code that detects common security issues |
| **ICommandExecutorPort** | Contract trait for executing external commands |

## 3. Feature Overview
### 3.1 Background & Problem
Python applications are prone to security issues like SQL injection, command injection, and hardcoded passwords. Bandit is the standard Python security linter. Lint Arwaky integrates Bandit to surface security vulnerabilities in scanned Python projects.

### 3.2 Business Goals
- Detect security vulnerabilities in Python code during scan
- Surface Bandit findings with severity, confidence, and CWE references
- Integrate via the standard `ILinterAdapterPort` contract

### 3.3 Target Users
- **Python Developers**: Identify security issues early
- **Security Engineers**: Audit Python code for OWASP Top 10 vulnerabilities

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Python developer, I want Bandit security issues reported when I run `scan`, so I can fix vulnerabilities before deployment.
- **US-002:** As a security engineer, I want each finding to include its CWE reference and confidence level.

### 4.2 Use Cases & Workflow
```
Input: scan /project (Python project)

1. BanditAdapter::scan() called with directory path
2. Build command: bandit -r --format=json <path>
3. Execute via ICommandExecutorPort
4. Parse JSON output → extract results array
5. Map severity and confidence → LintResult entries
```

### 4.3 Business Rules
- Severity mapping: Bandit HIGH → AES HIGH, MEDIUM → MEDIUM, LOW → LOW
- All findings are reported regardless of confidence
- Timeout: 120 seconds for large projects
- Configurable `bin_path`: defaults to `"bandit"`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Full project scan | < 60s |
| NFR-002 | JSON parse accuracy | 100% of findings captured |

## 6. UI/UX Requirements
```
AES075 HIGH - src/views.py:104
  bandit: B108 — hardcoded_password — Possible hardcoded password 'secret123'
  CWE: CWE-259, Confidence: HIGH

AES075 MEDIUM - src/db.py:22
  bandit: B608 — sql_injection — Possible SQL injection via raw string concatenation
  CWE: CWE-89, Confidence: MEDIUM
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Python project with security issues | Scan runs | AES075 entries reported per finding | Pending Review |
| AC-002 | Python project with no findings | Scan runs | No AES075 entries | Pending Review |
| AC-003 | Bandit not installed | Scan runs | Graceful error handling | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_py_bandit.rs` (143 lines). It parses Bandit's JSON output which includes `issue_text`, `severity`, `confidence`, `code`, `cwe`, and location fields. The `map_severity()` method converts Bandit severity strings to the AES `Severity` enum.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired in orchestrator | Register via ServiceContainerAggregate |
| bandit | External Python tool | Not installed | Document as prerequisite; graceful skip |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_py_bandit.rs` — BanditAdapter implementation
- `src-rust/language-adapters/mod.rs:21` — Module export
- `src-rust/di-containers/contract_service_aggregate.rs` — ILinterAdapterPort trait
