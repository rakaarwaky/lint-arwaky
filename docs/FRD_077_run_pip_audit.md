# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run pip-audit (FR-077)  
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
This document defines the external tool adapter that runs `pip-audit` on Python projects to scan dependencies (from `requirements.txt` or `pyproject.toml`) for known security vulnerabilities.

### 2.2 Scope
**In-Scope:**
- Invoking `pip-audit` on Python project manifests
- Parsing output for vulnerability advisories
- Reporting each advisory as a LintResult entry
- Configurable binary path via `bin_path`

**Out-of-Scope:**
- `pip-audit --fix` to auto-update dependencies
- License compliance auditing
- Scanning non-Python dependency files

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **DependencyAdapter** | Infrastructure struct in `infrastructure_py_analysis.rs` implementing `ILinterAdapterPort` |
| **pip-audit** | Python tool that scans project dependencies for known vulnerabilities (PyPI Advisory Database) |
| **ILinterAdapterPort** | Contract trait for external linter adapters |

## 3. Feature Overview
### 3.1 Background & Problem
Python projects depend on PyPI packages that may contain security vulnerabilities. `pip-audit` scans project manifests against the PyPI Advisory Database. Lint Arwaky needs a pip-audit adapter so dependency vulnerabilities are surfaced during Python project scans.

### 3.2 Business Goals
- Detect vulnerable Python dependencies during scan
- Provide advisory details (package, version, patched version)
- Integrate via `ILinterAdapterPort` for consistent tool adapter pattern

### 3.3 Target Users
- **Python Developers**: Know when Python dependencies have known vulnerabilities
- **Security Engineers**: Audit Python supply chain as part of CI

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Python developer, I want vulnerable pip packages flagged when I run `lint-arwaky scan`, so I can upgrade them.
- **US-002:** As a security engineer, I want each advisory to include its CVE/PVE ID and patched version.

### 4.2 Use Cases & Workflow
```
Input: scan /project (Python project with requirements.txt)

1. DependencyAdapter::scan() called
2. Build command: pip-audit
3. Execute via ICommandExecutorPort
4. Parse output → extract advisory records
5. Each advisory → LintResult { severity: HIGH, code: advisory.id }
```

### 4.3 Business Rules
- Severity: HIGH for all vulnerability advisories
- Reports package name, affected version, patched version, and advisory ID
- Timeout: 120 seconds
- Configurable `bin_path`: defaults to `"pip-audit"`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Full dependency audit | < 60s |
| NFR-002 | Output parse accuracy | 100% of advisories captured |

## 6. UI/UX Requirements
```
AES077 HIGH - requirements.txt
  pip-audit: PVE-2024-61234 — requests 2.28.0 has a header injection vulnerability
  Patched version: >= 2.31.0
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Python project with known vulnerable dependency | Scan runs | AES077 HIGH reported per advisory | Pending Review |
| AC-002 | Python project with no vulnerabilities | Scan runs | No AES077 entries | Pending Review |
| AC-003 | No requirements.txt or pyproject.toml | Scan runs | Graceful skip | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_py_analysis.rs` as `DependencyAdapter` (lines 148–185). The `name()` method returns `AdapterName::raw("pip-audit")`. The `scan()` method is currently a stub returning `Ok(LintResultList::default())` — actual `pip-audit` execution has not yet been implemented. The adapter is referenced in the CLI dev command adapter listing at `cli-commands/surface_dev_command.rs:105`.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired in orchestrator | Register via ServiceContainerAggregate |
| pip-audit | External Python tool | Not installed | Document as prerequisite; graceful skip |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_py_analysis.rs:148` — DependencyAdapter implementation
- `src-rust/language-adapters/mod.rs:17` — Module export
- `src-rust/cli-commands/surface_dev_command.rs:105` — Adapter listing reference
