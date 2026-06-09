# 📄 Feature Requirements Document (FRD)
**Feature Name:** Direct Command Execution — `std::process::Command`
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
Defines the command execution transport that invokes all external tools (clippy, ruff, eslint, prettier, cargo) via `std::process::Command`, providing a unified execution interface.

### 2.2 Scope
**In-Scope:** `std::process::Command` wrapper, timeout handling, stdout/stderr capture, exit code analysis, error normalization.
**Out-of-Scope:** Async execution, remote command execution, shell injection protection (args are static).

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **CommandResult** | Struct with stdout, stderr, exit code, duration |
| **cli-transport** | Feature folder for command execution layer |

## 3. Feature Overview
### 3.1 Background & Problem
Each linter adapter had its own command execution logic, duplicating error handling and output parsing across the codebase.

### 3.2 Business Goals
- Centralize command execution in a single transport layer
- Provide consistent error handling and timeout support
- Enable testable command execution via trait abstraction

### 3.3 Target Users
- Linter adapters (clippy, ruff, eslint, prettier)
- Fix orchestrators that invoke external tools

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a linter adapter, I want to execute `cargo clippy` and get structured output.
- **US-002:** As a fix orchestrator, I want to call `cargo clippy --fix` with timeout handling.

### 4.2 Use Cases & Workflow
```
CommandExecutor.execute("cargo", ["clippy", "--fix", "--allow-dirty"])
  │
  ├─► Create std::process::Command
  ├─► Set working directory
  ├─► Capture stdout + stderr
  ├─► Wait with timeout (60s default)
  └─► Return CommandResult { stdout, stderr, exit_code, duration }
```

### 4.3 Business Rules
- Default timeout: 60s (configurable via `--timeout` for each tool)
- Non-zero exit code does NOT panic — returned as `CommandResult`
- Stderr is captured but NOT treated as error (tools may warn on stderr)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Command overhead (not tool runtime) | < 10ms |
| NFR-002 | Timeout accuracy | ±100ms |

## 6. UI/UX Requirements
No direct UI. Used internally by linter adapters.

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Valid command | Execute `cargo --version` | Returns CommandResult with stdout | Pending Review |
| AC-002 | Failed command | Execute `cargo bogus` | Returns CommandResult with non-zero exit | Pending Review |
| AC-003 | Timeout hit | Execute `sleep 10` with 1s timeout | Returns timeout error | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Command executor trait | `cli-transport/contract_executor_port.rs` | Pending Review |
| Command executor impl | `cli-transport/infrastructure_executor_adapter.rs` | Pending Review |
| Command result VO | `cli-transport/taxonomy_command_result_vo.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-001 (CLI Foundation) | Entry point | Must bootstrap transport | Standard initialization |
| External tools | clippy, ruff, eslint, etc. | Tool not installed → error | Check availability before execution |

## 10. Appendices
- `src-rust/cli-transport/` — Feature folder
- `src-rust/cli-transport/contract_executor_port.rs`
