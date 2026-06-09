# 📄 Feature Requirements Document (FRD)
**Feature Name:** External Project Scan (`scan [path]`)  
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
This document defines the external project scan CLI command `scan [path]`. It runs all AES architecture rules plus external language-specific linter adapters (clippy for Rust, ruff for Python, eslint and tsc for JavaScript/TypeScript), producing a consolidated quality report.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli scan <path>` — AES rules + external adapter linting
- Adapter dispatch: clippy (Rust), ruff (Python), eslint + tsc (JS/TS)
- Consolidated report merging AES and external tool violations
- Score computation across all violation sources
- Language auto-detection and config loading

**Out-of-Scope:**
- Auto-fixing violations (handled by FR-057 `fix` command)
- CI exit codes with threshold (handled by FR-059 `ci` command)
- Report generation in specific formats (handled by FR-058 `report` command)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **External adapter** | Bridge that invokes a third-party linter and parses its output |
| **ClippyAdapter** | External adapter for Rust's clippy linter |
| **RuffAdapter** | External adapter for Python's ruff linter |
| **ESLintAdapter** | External adapter for JavaScript/TypeScript eslint |
| **TscAdapter** | External adapter for TypeScript compiler type checking |
| **Consolidated report** | Merged output from all AES + external sources |

## 3. Feature Overview
### 3.1 Background & Problem
The AES rules cover architectural compliance but do not replace language-specific linters. Developers had to run clippy, ruff, eslint, and `check .` separately. There was no unified view of all code quality issues in one report, making CI pipelines complex and developer feedback fragmented.

### 3.2 Business Goals
- Unify AES architecture checks with language-specific linting in one command
- Provide a single consolidated report with all violations
- Support Rust, Python, and JavaScript/TypeScript projects
- Auto-detect which adapters to run based on project language

### 3.3 Target Users
- **Developers**: Run `scan .` for a comprehensive code quality audit
- **DevOps**: Use in CI to run all checks with one command
- **Engineering Managers**: Get a single quality score covering architecture + style + correctness

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Rust developer, I want to run `scan .` to see both AES violations and clippy warnings in one report.
- **US-002:** As a Python developer, I want `scan .` to automatically run ruff on my code and merge results with AES checks.
- **US-003:** As a JS/TS developer, I want both eslint and tsc results included in the scan output.

### 4.2 Use Cases & Workflow
**Scan Pipeline:**
```
lint-arwaky-cli scan /project
  │
  ├─► 1. Detect language → Rust
  │
  ├─► 2. Run AES checks (same as check)
  │     ├── Per-file: AES003, AES004, AES005, AES006, AES009, AES014, AES015...
  │     └── Cross-file: AES001, AES002, AES007, AES010, AES011, AES020...
  │
  ├─► 3. Run external adapters
  │     ├── ClippyAdapter.invoke("/project")
  │     │   └── cargo clippy --message-format=json --all-targets
  │     ├── RuffAdapter.invoke("/project")
  │     │   └── ruff check /project --output-format json
  │     ├── ESLintAdapter.invoke("/project")
  │     │   └── eslint /project --format json
  │     └── TscAdapter.invoke("/project")
  │         └── tsc --noEmit --pretty false
  │
  ├─► 4. Merge results
  │     ├── AES violations → internal source
  │     └── External tool violations → tool-specific source
  │
  ├─► 5. Compute combined score
  │
  └─► 6. Output consolidated report
```

**Adapter Output Parsing:**
```
clippy JSON:
  {"message":"unused variable","code":"unused_variables","level":"warning",
   "file":"src/main.rs","line":42}

ruff JSON:
  {"code":"F841","message":"Local variable `x` is assigned but unused",
   "location":{"file":"src/main.py","row":15}}

eslint JSON:
  {"filePath":"src/index.ts","messages":[{"ruleId":"no-unused-vars",
    "message":"'count' is defined but never used","line":23,"severity":2}]}
```

### 4.3 Business Rules
- Language detection determines which external adapters run
- External tool violations are prefixed with tool name (e.g., `clippy:`)
- AES and external violations are scored separately and combined
- If external tool is not installed → warning logged, scan continues
- External tool violations do NOT trigger CRITICAL auto-fail (AES-only)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Full scan of 1000-file Rust project | < 30s (clippy dominant) |
| NFR-002 | Adapter invocation overhead | < 100ms per adapter |
| NFR-003 | Parsing clippy/ruff/eslint/tsc output | < 500ms |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli scan /project
🔍 Scanning /project (language: Rust)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

=== AES Architecture Rules ===
Score: 92.0 / 100  ✅
CRITICAL: 0 | HIGH: 1 | MEDIUM: 3 | LOW: 1

=== External Linters ===
🔧 clippy: 4 warnings, 2 errors
🔧 rustfmt: 3 formatting issues

=== Combined Summary ===
Total violations: 14
  AES: 5  |  clippy: 6  |  rustfmt: 3

Recommendation: Fix 2 clippy errors before proceeding.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Rust project with AES + clippy violations | `scan /project` runs | Both AES and clippy violations reported | Pending Review |
| AC-002 | Python project with ruff violations | `scan /project` runs | AES + ruff violations reported | Pending Review |
| AC-003 | JS/TS project with eslint violations | `scan /project` runs | AES + eslint (+ tsc) violations | Pending Review |
| AC-004 | External tool not installed | `scan /project` runs | Warning logged, scan continues | Pending Review |
| AC-005 | No violations anywhere | `scan /project` runs | Perfect score, clean report | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI scan command | `cli-commands/surface_core_command.rs` | — | **FULLY IMPLEMENTED** — CLI dispatch |
| ClippyAdapter | `language-adapters/infrastructure_clippy_adapter.rs` | — | **FULLY IMPLEMENTED** — invokes clippy, parses JSON |
| RuffAdapter | `language-adapters/infrastructure_ruff_adapter.rs` | — | **FULLY IMPLEMENTED** — invokes ruff, parses JSON |
| ESLintAdapter | `language-adapters/infrastructure_eslint_adapter.rs` | — | **FULLY IMPLEMENTED** — invokes eslint, parses JSON |
| TscAdapter | `language-adapters/infrastructure_tsc_adapter.rs` | — | **FULLY IMPLEMENTED** — invokes tsc, parses output |
| Scan orchestrator | `code-analysis/agent_scan_orchestrator.rs` | — | **FULLY IMPLEMENTED** — runs AES + adapters, merges results |
| Merged result VO | `shared-common/taxonomy_merged_result_vo.rs` | — | **FULLY IMPLEMENTED** |

### 8.2 Bugs Found

1. **ClippyAdapter uses hardcoded `--message-format=json`** — fails on older clippy versions that don't support this flag
   - **Impact**: Scan fails on CI runners with older Rust toolchains
   - **Fix**: Add version detection fallback to `--message-format=human`

2. **TscAdapter only returns errors, not warnings** — `tsc --noEmit` only emits errors by default; `--pretty false` doesn't affect this
   - **Impact**: TypeScript warnings are silently dropped from scan results
   - **Fix**: Add `--noErrorTruncation` flag, parse both error and warning severities

3. **No adapter timeout** — if clippy hangs on a large project, the scan hangs indefinitely
   - **Impact**: CI pipeline blocks forever
   - **Fix**: Add 60s timeout per adapter invocation

### 8.3 What Needs to Be Added

- **Clippy version detection**: Fall back to human-readable parsing for older versions
- **TSC warning parsing**: Capture warnings, not just errors
- **Adapter timeout**: 60s timeout with graceful error handling
- **Cancel support**: Wire scan pipeline into job cancellation (FR-067)

### 8.4 What to Keep

- **Consolidated report** ✅ — AES + external violations merged in single output
- **Language detection** ✅ — correct adapter selection per language
- **Graceful missing-tool handling** ✅ — warns, doesn't crash
- **JSON output parsing** ✅ — clippy, ruff, eslint parsers all working

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli scan test-project-rust/` detects intentional AES + clippy violations
- `lint-arwaky-cli scan test-project-python/` runs ruff and reports violations
- `lint-arwaky-cli scan test-project-javascript/` runs eslint and tsc
- Pending Review: Adapter timeout and tsc warning capture

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (check) | AES checks are prerequisite for scan | If check fails, scan also fails | Scan wraps check, propagates errors |
| External tools | clippy, ruff, eslint, tsc must be installed | Tool missing = partial results | Graceful handling per adapter |
| External tool versions | CLI flags may differ across versions | JSON parsing failures | Adapter version detection |

## 10. Appendices
- `src-rust/cli-commands/surface_core_command.rs` — CLI scan command
- `src-rust/code-analysis/agent_scan_orchestrator.rs` — Scan orchestrator
- `src-rust/language-adapters/infrastructure_clippy_adapter.rs` — Clippy adapter
- `src-rust/language-adapters/infrastructure_ruff_adapter.rs` — Ruff adapter
- `src-rust/language-adapters/infrastructure_eslint_adapter.rs` — ESLint adapter
- `src-rust/language-adapters/infrastructure_tsc_adapter.rs` — TSC adapter
- `src-rust/shared-common/taxonomy_merged_result_vo.rs` — Merged result VO
