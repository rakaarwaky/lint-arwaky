# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run Clippy (FR-070)  
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
This document defines the external tool adapter that runs `cargo clippy` on Rust source files. The adapter invokes Clippy via `std::process::Command` through the `ICommandExecutorPort` abstraction and parses its JSON output into `LintResult` entries.

### 2.2 Scope
**In-Scope:**
- Invoking `cargo clippy --message-format=json` on Rust projects
- Parsing Clippy JSON output into structured `LintResult` objects
- Mapping Clippy severity levels (error, warning, note, help) to AES severity
- Configurable binary path via `bin_path` option

**Out-of-Scope:**
- Installing or updating Clippy via the linter
- Auto-fixing Clippy warnings
- Running Clippy on non-Rust files

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **RustLinterAdapter** | Infrastructure struct in `infrastructure_rs_linter.rs` implementing `ILinterAdapterPort` |
| **ICommandExecutorPort** | Contract trait for executing external commands |
| **cargo clippy** | Rust linter with additional lint checks beyond rustc |

## 3. Feature Overview
### 3.1 Background & Problem
Rust projects require Clippy for enforcing idiomatic Rust style and catch common mistakes. Lint Arwaky must integrate Clippy as one of its language adapters so Rust projects scanned by the tool also receive Clippy diagnostics alongside AES rules.

### 3.2 Business Goals
- Provide Clippy lint results as part of the unified scan output
- Support configurable Clippy binary path
- Integrate via the standard `ILinterAdapterPort` contract for pluggable tool adapters

### 3.3 Target Users
- **Rust Developers**: Get Clippy feedback during `lint-arwaky scan`
- **DevOps/CI**: Include Clippy in automated quality gates

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Rust developer, I want Clippy warnings and errors to appear in the linter output when I run `scan`, so I see all Rust diagnostics in one place.
- **US-002:** As a project maintainer, I want to configure a custom Clippy binary path via YAML, so I can use a specific toolchain version.

### 4.2 Use Cases & Workflow
```
Input: scan /project (Rust project)

1. RustLinterAdapter::scan() called with file path
2. Resolve working directory (search for Cargo.toml)
3. Build command: cargo clippy --message-format=json
4. Execute via ICommandExecutorPort
5. Parse JSON output → Vec<LintResult>
6. Return LintResultList to orchestrator
```

### 4.3 Business Rules
- Severity mapping: Clippy `error` → AES HIGH, `warning` → MEDIUM, `note`/`help` → LOW
- Falls back to `cargo clippy` if no `bin_path` configured
- Timeout: 120 seconds per invocation

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Parse Clippy JSON per file | < 500ms |
| NFR-002 | False negative rate | 0% — all Clippy diagnostics captured |
| NFR-003 | Config-driven binary path | Optional `bin_path` in YAML |

## 6. UI/UX Requirements
```
AES070 HIGH - src/main.rs:15
  cargo clippy: redundant_field_names — Field 'value' is assigned the same value as the field name.

AES070 MEDIUM - src/lib.rs:42
  cargo clippy: needless_return — Remove explicit `return` as the last expression.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Rust project with Clippy violations | Scan runs | Clippy violations appear as AES070 entries | Pending Review |
| AC-002 | Rust project with no Clippy violations | Scan runs | No AES070 entries reported | Pending Review |
| AC-003 | `bin_path` configured in YAML | Scan runs | Custom binary used instead of `cargo clippy` | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_rs_linter.rs` (221 lines). It uses `serde_json::Value` to parse Clippy's JSON output stream. Working directory resolution walks up parent directories looking for `Cargo.toml`. The adapter implements `ILinterAdapterPort` and is registered in `mod.rs` as `RustLinterAdapter`.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired into scan orchestrator | Register via ServiceContainerAggregate |
| ICommandExecutorPort | Command execution abstraction | Executor not available | Injected via DI container |
| cargo clippy | External binary | Not installed on target system | Document as prerequisite |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_rs_linter.rs` — RustLinterAdapter implementation
- `src-rust/language-adapters/mod.rs:41` — Module export
- `src-rust/di-containers/contract_service_aggregate.rs` — ILinterAdapterPort trait definition
