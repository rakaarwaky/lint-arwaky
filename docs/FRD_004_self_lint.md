# 📄 Feature Requirements Document (FRD)
**Feature Name:** Self-Lint Target (`lint-arwaky-cli check .`)  
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
This document defines the self-lint feature that runs all 31 AES rules against the project's own `src-rust/` directory. It enables dogfooding — the linter audits itself — and provides CI gates, quality scores, and structured reporting.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli check .` — run all AES rules on the project
- `lint-arwaky-cli scan <path>` — run AES + external adapaters
- `lint-arwaky-cli ci <path> --threshold <N>` — CI mode with exit code
- `lint-arwaky-cli report <path> --format <format>` — reporting
- Score computation and CRITICAL auto-fail
- Report formats: text, JSON, SARIF, JUnit

**Out-of-Scope:**
- External tool linting (clippy, ruff, eslint — handled by FR-070 to FR-080)
- Auto-fixing violations (covered by FR-005)
- Quality trends (covered by FR-006)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Self-lint** | Running AES rules against the project's own code |
| **ArchitectureGovernanceEntity** | Score + violations + compliance status |
| **LintResult** | Single violation with file, line, severity, message |
| **LintCheckingCoordinator** | Agent that orchestrates all 31 rule checks |
| **CRITICAL auto-fail** | Any CRITICAL violation → run fails regardless of score |

## 3. Feature Overview
### 3.1 Background & Problem
The project could not verify its own architectural compliance. Architecture violations were only caught during code review. There was no quantitative measure of codebase health, no CI gate to prevent merging violations, and no structured violation reporting.

### 3.2 Business Goals
- Dogfooding: the linter must pass on its own code
- CI integration: PRs blocked if score drops below threshold
- Visibility: violations reported in text, JSON, SARIF, JUnit formats
- Score tracking: quantitative health metric (0-100)

### 3.3 Target Users
- **Developers**: Run `check .` before committing
- **CI/CD Pipelines**: Run `ci . --threshold 80` in CI
- **Architecture Engineers**: Review violations in structured reports

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `lint-arwaky-cli check .` to see all architecture violations in my code, so I can fix them before pushing.
- **US-002:** As a CI pipeline, I want to run `lint-arwaky-cli ci . --threshold 80` that exits non-zero if the score is below 80, so I can block bad PRs.
- **US-003:** As an architect, I want violations in SARIF format, so I can integrate with GitHub Code Scanning.

### 4.2 Use Cases & Workflow
**Self-Lint Pipeline:**
```
lint-arwaky-cli check .
  │
  ├─► 1. Find source directory
  │     src-rust/ → Rust
  │
  ├─► 2. Load config
  │     lint_arwaky.config.rust.yaml → ArchitectureConfig
  │
  ├─► 3. Run all checks
  │     LintCheckingCoordinator.run_all_checks()
  │     │
  │     ├── Per-file checks (22 rules):
  │     │   ├── AES003 naming, AES004/005 file size
  │     │   ├── AES006 primitive, AES009 struct/trait
  │     │   ├── AES011 suffix, AES014 bypass
  │     │   ├── AES015 unused, AES016 dead inheritance
  │     │   ├── AES021 agent role, AES022 surface role
  │     │   ├── AES023 surface import, AES024 any-bypass
  │     │   ├── AES025 MCP schema, AES026 inheritance
  │     │   ├── AES027 mandatory trait, AES030–AES033
  │     │   └── ...
  │     │
  │     └── Cross-file checks (9 rules):
  │         ├── AES001 forbidden import, AES002 mandatory
  │         ├── AES007 barrel, AES010/AES011 suffix
  │         ├── AES012 completeness, AES013 internal
  │         ├── AES018 hierarchy, AES019 passive
  │         └── AES020 circular dependency
  │
  ├─► 4. Compute score
  │     Score = 100
  │     LOW -1 | MEDIUM -2 | HIGH -3 | CRITICAL -5
  │     Any CRITICAL → auto-fail
  │
  └─► 5. Report
        Text / JSON / SARIF / JUnit
```

**Score Computation:**
```
Start: 100.0
  AES001 CRITICAL → -5   → 95.0
  AES003 LOW      → -1   → 94.0
  AES011 HIGH     → -3   → 91.0
  Result: 91.0/100
  Has CRITICAL? → Yes → FAIL (auto-fail)
```

### 4.3 Business Rules
- Score starts at 100, deducted per violation
- CRITICAL auto-fail: any CRITICAL → run fails regardless of score
- CI mode: exit code 1 if score < threshold
- Report formats: text (default), JSON, SARIF 2.1.0, JUnit XML

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Full self-lint of `src-rust/` | < 5s |
| NFR-002 | CI exit code accuracy | 100% reliable |
| NFR-003 | Report generation (all formats) | < 1s after check |

## 6. UI/UX Requirements
CLI output grouped by severity:
```
Score: 87.5 / 100
CRITICAL: 0 | HIGH: 3 | MEDIUM: 5 | LOW: 2

=== HIGH ===
AES011 - src-rust/capabilities/my_file.rs:12 - Suffix mismatch
AES023 - src-rust/surfaces/cli_check.rs:42 - Direct infra import

=== MEDIUM ===
AES007 - src-rust/surfaces/command.rs:5 - Barrel import style
...
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | `lint-arwaky-cli check .` runs | Pipeline completes | Violations printed, exit 0 | ✅ |
| AC-002 | CRITICAL violation exists | Score computed | auto-fail, exit 1 | ✅ |
| AC-003 | `ci . --threshold 80` with score 75 | CI check runs | exit 1 | ✅ |
| AC-004 | `ci . --threshold 80` with score 85 | CI check runs | exit 0 | ✅ |
| AC-005 | `report --format json` | Report generated | Valid JSON output | ✅ |
| AC-006 | `report --format sarif` | Report generated | SARIF 2.1.0 compliant | ✅ |
| AC-007 | 31 AES rules all executed | `run_all_checks()` completes | All codes present in output | ✅ |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-001 (Architecture) | Layer definitions needed for rule checking | Architecture changes break rules | Config-driven |
| FR-002 (Config) | Rule configuration from YAML | Missing config = default | Built-in fallback |
| FR-003 (Parsing) | Source parsing for all file analysis | Parser limitations affect accuracy | Document limitations |
| 10 capability checkers | Rule implementations | Checker bugs cause false positives | Unit tests for each checker |

## 9. Appendices
- `src-rust/agent/architecture_lint_orchestrator.rs` — Orchestration entry
- `src-rust/agent/lint_checking_coordinator.rs` — 31 rule coordinator
- `src-rust/surfaces/cli_check_command.rs` — CLI command
- `src-rust/cli_main_entry.rs` — CLI routing
- `docs/RULES_AES.md` — Full rule catalog
