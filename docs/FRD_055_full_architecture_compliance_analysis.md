# 📄 Feature Requirements Document (FRD)
**Feature Name:** Full Architecture Compliance Analysis (`check [path]`)  
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
This document defines the full architecture compliance analysis CLI command `check [path] [--git-diff]`. It runs all AES rules (FR-001 through FR-050) against a target codebase and produces a consolidated architecture compliance report with score, violation counts, and per-rule breakdowns.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli check <path>` — run all 31 AES rules
- `lint-arwaky-cli check <path> --git-diff` — run only on files changed in working tree
- Score computation (0–100) with CRITICAL auto-fail
- Per-rule violation breakdown with counts by severity
- Config loading for the target project
- Language auto-detection (Rust, Python, JavaScript/TypeScript)

**Out-of-Scope:**
- External tool linters (clippy, ruff, eslint — handled by `scan` command in FR-056)
- Auto-fixing violations (handled by `fix` command in FR-057)
- CI exit codes (handled by `ci` command in FR-059)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES rule** | Architecture Enforcement Specification rule (AES001–AES033) |
| **ArchitectureGovernanceEntity** | Score + violations + compliance status |
| **LintResult** | Single violation with file, line, severity, message |
| **CRITICAL auto-fail** | Any CRITICAL violation → run fails regardless of score |
| **--git-diff** | Flag to only check files with uncommitted changes |

## 3. Feature Overview
### 3.1 Background & Problem
Developers needed a single command to audit their entire codebase against all AES rules. Previously, individual checkers had to be run separately. There was no consolidated view of architecture health, no score, and no way to filter by changed files for faster iteration.

### 3.2 Business Goals
- Provide a single-entry audit command for all AES rules
- Enable fast re-checks with `--git-diff` for changed files only
- Produce a clear, actionable compliance report
- Support all three target languages (Rust, Python, JS/TS)

### 3.3 Target Users
- **Developers**: Run `check .` before commits to catch violations early
- **Architecture Engineers**: Review overall codebase health
- **AI Agents**: Use MCP to trigger checks autonomously

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `lint-arwaky-cli check .` to audit my entire project against all 31 AES rules in one command.
- **US-002:** As a developer, I want to use `--git-diff` to only check files I've changed, so the check runs faster during development.
- **US-003:** As an architect, I want a per-rule breakdown with violation counts by severity, so I can prioritize which violations to address.

### 4.2 Use Cases & Workflow
**Check Pipeline:**
```
lint-arwaky-cli check /project
  │
  ├─► 1. Detect project language
  │     /project/src-rust/ → Rust
  │
  ├─► 2. Load project config
  │     lint_arwaky.config.rust.yaml → ArchitectureConfig
  │
  ├─► 3. Walk source tree
  │     ├── Normal mode: all files
  │     └── --git-diff mode: only git-dirty files
  │
  ├─► 4. Run per-file checks (22 rules)
  │     ├── AES003 naming, AES004/005 file size
  │     ├── AES006 primitive, AES009 struct/trait
  │     ├── AES014 bypass, AES015 unused
  │     ├── AES030–AES033 edge cases
  │     └── ...
  │
  ├─► 5. Run cross-file checks (9 rules)
  │     ├── AES001 forbidden import, AES002 mandatory
  │     ├── AES007 barrel, AES010/AES011 suffix
  │     ├── AES012 completeness, AES013 internal
  │     ├── AES018 hierarchy, AES019 passive
  │     └── AES020 circular dependency
  │
  ├─► 6. Compute score
  │     Score = 100 - deductions
  │     CRITICAL flag → auto-fail
  │
  └─► 7. Output report
        Text table with severity groups
```

**--git-diff Workflow:**
```
lint-arwaky-cli check /project --git-diff
  1. Run `git diff --name-only HEAD` (or `jj status` equivalent)
  2. Filter source files (*.rs, *.py, *.js, *.ts, *.tsx)
  3. Run checks only on those files + cross-file validation
  4. Report: "12 files changed, 3 violations found"
```

### 4.3 Business Rules
- Normal mode: scan all files in `src/` or `src-rust/`/`src-python/`/`src-javascript/`
- `--git-diff` mode: only files with unstaged/uncommitted changes
- Score starts at 100, deductions per violation per severity
- Any CRITICAL violation → run fails (exit code 1)
- Output grouped by severity (CRITICAL first, then HIGH, MEDIUM, LOW)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Full check of 1000-file project | < 5s |
| NFR-002 | --git-diff check (10 changed files) | < 500ms |
| NFR-003 | Exit code accuracy | 100% (0 = pass, 1 = fail) |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli check /project
📊 Architecture Compliance Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Project: /project
Language: Rust
Config: lint_arwaky.config.rust.yaml (22 rules active)

Score: 87.5 / 100  ⚠️  Below threshold (90)

┌──────────┬───────┬──────┐
│ Severity │ Count │      │
├──────────┼───────┤      │
│ CRITICAL │     0 │ ✅   │
│ HIGH     │     3 │ ⚠️   │
│ MEDIUM   │     5 │ ⚠️   │
│ LOW      │     2 │ ℹ️   │
└──────────┴───────┴──────┘

=== HIGH ===
AES011 - src-rust/layer-rules/capabilities_import_checker.rs:12 - Suffix mismatch
AES023 - src-rust/cli-commands/surface_check_command.rs:42 - Direct infra import
AES016 - src-rust/orphan-detector/taxonomy_orphan_vo.rs:8 - Dead inheritance

=== MEDIUM ===
AES007 - src-rust/cli-commands/surface_command_handler.rs:5 - Barrel import style
AES015 - src-rust/primitive-checker/capabilities_primitive_checker.rs:33 - Unused import
...
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Target project with all clean AES rules | `check /project` runs | Score = 100, exit 0 | Pending Review |
| AC-002 | Target project with 5 violations | `check /project` runs | Score < 100, violations listed | Pending Review |
| AC-003 | CRITICAL violation present | `check /project` runs | auto-fail, exit 1 | Pending Review |
| AC-004 | `--git-diff` with 3 changed files | `check /project --git-diff` runs | Only 3 files checked | Pending Review |
| AC-005 | Unsupported language project | `check /project` runs | Error: unsupported language | Pending Review |
| AC-006 | 31 AES rules all executed | Full check completes | All codes present in output | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI check command | `cli-commands/surface_check_command.rs` | 89 | **FULLY IMPLEMENTED** — CLI dispatch with path and --git-diff |
| LintCheckingCoordinator | `code-analysis/agent_checking_coordinator.rs` | 767 | **FULLY IMPLEMENTED** — 16 inline checker methods |
| Pipeline execution | `pipeline-jobs/agent_pipeline_execution.rs` | — | **FULLY IMPLEMENTED** — async dispatch |
| Score computation | `shared-common/taxonomy_score_vo.rs` | — | **FULLY IMPLEMENTED** |
| Language detection | `config-system/infrastructure_detector_provider.rs` | 40 | **FULLY IMPLEMENTED** |
| Config loading | `config-system/agent_loading_orchestrator.rs` | 82 | **PARTIALLY IMPLEMENTED** — config-discard bug |
| --git-diff filtering | `cli-commands/surface_check_command.rs` | — | **FULLY IMPLEMENTED** — uses VCS diff |

### 8.2 Bugs Found

1. **Config-discard bug** (`agent_loading_orchestrator.rs:55-57`) — parsed YAML config is discarded in favor of hardcoded defaults
   - **Impact**: All project-specific rule configurations are non-functional during `check`
   - See FRD-002 Empirical Findings for full details

2. **Inline checkers bypass dedicated implementations** — the coordinator has inline versions of checks that duplicate logic in dedicated checker files
   - **Impact**: Inconsistent behavior between `check` and individual checker runs

### 8.3 What Needs to Be Added

- **Config-discard fix**: Wire parsed config instead of hardcoded defaults
- **Checker consolidation**: Delegate from coordinator inline methods to dedicated checker implementations
- **Cross-project check**: Support for multi-project workspaces

### 8.4 What to Keep

- **CLI structure** ✅ — clean `check` command with `<path>` and `--git-diff`
- **Pipeline execution** ✅ — async dispatch with progress reporting
- **Score computation** ✅ — correct deductions and CRITICAL auto-fail
- **Report formatting** ✅ — severity-grouped output with counts

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli check .` runs successfully against own codebase
- `lint-arwaky-cli check test-project-rust/` detects intentional violations
- `--git-diff` correctly filters to changed files only (verified with git status)
- Pending Review: All acceptance criteria after config-discard fix

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-001 to FR-050 | All 31 AES rule checkers | Missing checker = incomplete audit | Coordinator must register all codes |
| FR-003 (Parsing) | Source code parsing | Parser misses imports → wrong score | Test with known violation fixtures |
| FR-002 (Config) | Config loading | Config discard bug breaks customization | Fix in progress |

## 10. Appendices
- `src-rust/cli-commands/surface_check_command.rs` — CLI check command
- `src-rust/code-analysis/agent_checking_coordinator.rs` — Coordinator with all 31 rules
- `src-rust/pipeline-jobs/agent_pipeline_execution.rs` — Pipeline dispatch
- `src-rust/shared-common/taxonomy_score_vo.rs` — Score computation
