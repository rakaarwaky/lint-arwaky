# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run cargo-audit (FR-072)  
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
This document defines the external tool adapter that runs `cargo audit` on Rust projects to scan dependencies for known security vulnerabilities. Results are reported as `LintResult` entries with HIGH severity.

### 2.2 Scope
**In-Scope:**
- Invoking `cargo audit` on Rust project Cargo.lock
- Parsing JSON output for vulnerability advisories
- Reporting each advisory as a HIGH severity LintResult
- Working directory resolution (walk up to Cargo.toml)

**Out-of-Scope:**
- Running `cargo audit fix` to auto-update dependencies
- Scanning non-Rust dependency files
- License compliance reporting

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **CargoAuditAdapter** | Infrastructure struct in `infrastructure_rs_audit.rs` implementing `ILinterAdapterPort` |
| **cargo audit** | Cargo subcommand that audits Cargo.lock for crates with security vulnerabilities |
| **Advisory** | A security vulnerability report from RustSec Advisory Database |

## 3. Feature Overview
### 3.1 Background & Problem
Rust projects rely on third-party crates that may contain security vulnerabilities. `cargo audit` checks the `Cargo.lock` against the RustSec Advisory Database. Lint Arwaky needs to integrate this check so security vulnerabilities are surfaced in scan results.

### 3.2 Business Goals
- Detect vulnerable dependencies during scan
- Provide advisory details (crate, version, severity, patched version)
- Fail the scan when CRITICAL/HIGH severity advisories exist

### 3.3 Target Users
- **Rust Developers**: Know when dependencies have known vulnerabilities
- **Security Engineers**: Audit supply chain security as part of CI

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Rust developer, I want vulnerable dependencies flagged when I run `lint-arwaky scan`, so I can update them.
- **US-002:** As a security engineer, I want each advisory reported with its severity, so I can prioritize fixes.

### 4.2 Use Cases & Workflow
```
Input: scan /project (Rust project with Cargo.lock)

1. CargoAuditAdapter::scan() called
2. Resolve working directory (find Cargo.toml)
3. Build command: cargo audit --json
4. Execute via ICommandExecutorPort
5. Parse JSON output → extract advisories
6. Each advisory → LintResult { severity: HIGH, code: advisory.id }
```

### 4.3 Business Rules
- Severity: HIGH for all advisories
- Reports advisory ID, crate name, affected version, patched version, and description
- If `cargo audit` is not installed → graceful skip (no crash)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Full audit scan | < 30s |
| NFR-002 | JSON parse accuracy | 100% of advisories captured |

## 6. UI/UX Requirements
```
AES072 HIGH - Cargo.lock
  cargo-audit: RUSTSEC-2024-0001 — serde 1.0.100 has a vulnerability in derive macros.
  Advisory: https://rustsec.org/advisories/RUSTSEC-2024-0001
  Patched version: >= 1.0.200
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Rust project with known vulnerable dependency | Scan runs | AES072 HIGH reported per advisory | Pending Review |
| AC-002 | Rust project with no vulnerabilities | Scan runs | No AES072 entries | Pending Review |
| AC-003 | No Cargo.lock present | Scan runs | Graceful skip or warning | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_rs_audit.rs` (154 lines). Unlike the other Rust adapters, `CargoAuditAdapter` does **not** depend on `ICommandExecutorPort` — it only requires `IPathNormalizationPort`. The `scan()` method is a stub that currently returns `Ok(LintResultList::default())` and has not yet been wired to actually execute `cargo audit`.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired in orchestrator | Register via ServiceContainerAggregate |
| cargo-audit | External Rust tool | Not installed (`cargo install cargo-audit`) | Document as prerequisite; graceful skip |
| Cargo.lock | Required input file | Missing in some projects | Graceful skip with warning |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_rs_audit.rs` — CargoAuditAdapter implementation
- `src-rust/language-adapters/mod.rs:37` — Module export
- `src-rust/di-containers/contract_service_aggregate.rs` — ILinterAdapterPort trait
