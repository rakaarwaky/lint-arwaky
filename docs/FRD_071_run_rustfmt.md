# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run rustfmt (FR-071)  
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
This document defines the external tool adapter that runs `cargo fmt --all --check` on Rust source files to verify formatting compliance. Results are reported as `LintResult` entries when formatting violations are detected.

### 2.2 Scope
**In-Scope:**
- Invoking `cargo fmt --all --check` on Rust projects
- Detecting formatting violations from stderr output
- Configurable binary path via `bin_path` option
- Working directory resolution (walk up to Cargo.toml)

**Out-of-Scope:**
- Auto-fixing formatting issues (the adapter does not write files)
- Running rustfmt on non-Rust files
- Installing rustfmt component

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **RustFmtAdapter** | Infrastructure struct in `infrastructure_rs_fmt.rs` implementing `ILinterAdapterPort` |
| **cargo fmt --all --check** | Rust formatting check command — fails if any file is not formatted |
| **ICommandExecutorPort** | Contract trait for executing external commands |

## 3. Feature Overview
### 3.1 Background & Problem
Consistent code formatting is essential for Rust project maintainability. Rustfmt is the official Rust formatter. Lint Arwaky needs a adapter that runs `cargo fmt --check` and surfaces formatting issues in the unified scan output.

### 3.2 Business Goals
- Detect unformatted Rust files during scan
- Integrate formatting checks into the standard lint pipeline
- Support custom rustfmt binary paths via YAML config

### 3.3 Target Users
- **Rust Developers**: Verify formatting without running `cargo fmt` separately
- **DevOps/CI**: Enforce formatting in automated pipelines

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Rust developer, I want formatting violations reported during `lint-arwaky scan`, so I can fix formatting in one pass.
- **US-002:** As a CI maintainer, I want the scan to fail when formatting errors exist, so unformatted code never merges.

### 4.2 Use Cases & Workflow
```
Input: scan /project (Rust project)

1. RustFmtAdapter::scan() called
2. Resolve working directory (search for Cargo.toml)
3. Build command: cargo fmt --all --check
4. Execute via ICommandExecutorPort
5. Parse stderr: extract file paths from "Diff in" lines
6. Return LintResult (MEDIUM severity)
```

### 4.3 Business Rules
- Severity: MEDIUM for formatting violations
- One `LintResult` per unformatted file, line 1, column 0
- Timeout: 60 seconds
- Returns empty `LintResultList` on success (exit 0)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Command execution | < 10s for medium projects |
| NFR-002 | Output parsing accuracy | 100% of "Diff in" lines captured |

## 6. UI/UX Requirements
```
AES071 MEDIUM - src/main.rs:1
  rustfmt: File is not formatted. Run `cargo fmt` to fix formatting issues.

AES071 MEDIUM - src/lib.rs:1
  rustfmt: File is not formatted. Run `cargo fmt` to fix formatting issues.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Rust project with unformatted files | Scan runs | AES071 MEDIUM reported per file | Pending Review |
| AC-002 | Rust project with all files formatted | Scan runs | No AES071 entries | Pending Review |
| AC-003 | No Cargo.toml found | Scan runs | Graceful error or skip | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_rs_fmt.rs` (171 lines). Working directory resolution walks up to 10 parent directories looking for `Cargo.toml`, `lint_arwaky.config.yaml`, or `.git`. The `scan()` method captures stderr output and parses `Diff in <file>` lines to identify unformatted files.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired in orchestrator | Register via ServiceContainerAggregate |
| cargo fmt | External Rust component | Not installed (`rustup component add rustfmt`) | Document as prerequisite |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_rs_fmt.rs` — RustFmtAdapter implementation
- `src-rust/language-adapters/mod.rs:39` — Module export
- `src-rust/di-containers/contract_service_aggregate.rs` — ILinterAdapterPort trait
