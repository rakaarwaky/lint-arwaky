# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run TSC (FR-080)  
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
This document defines the external tool adapter that runs TypeScript compiler type checking (`tsc --noEmit`) on TypeScript files. The adapter invokes TSC via `ICommandExecutorPort` and parses its line-based error output into structured `LintResult` entries.

### 2.2 Scope
**In-Scope:**
- Invoking `tsc --noEmit --pretty false` on TS/TSX files
- Parsing TSC error output with regex for file, line, column, error code, and message
- Supporting two TSC output formats (standard and extended)
- Local binary resolution via `resolve_js_cmd()`

**Out-of-Scope:**
- Type-checking plain JS files with `--checkJs`
- Generating declaration files or emitting JS output
- TSC configuration generation

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **TSCAdapter** | Infrastructure struct in `infrastructure_js_linter.rs` implementing `ILinterAdapterPort` |
| **tsc --noEmit** | TypeScript compiler command that type-checks without producing output files |
| **resolve_js_cmd()** | Helper that resolves local `node_modules/.bin` binary or falls back to `npx`/`bunx` |

## 3. Feature Overview
### 3.1 Background & Problem
TypeScript projects require type checking to catch type errors at compile time. Lint Arwaky integrates `tsc --noEmit` so TypeScript type errors are surfaced during scans alongside lint and format checks.

### 3.2 Business Goals
- Surface TypeScript type errors in unified scan output
- Support standard TSC error output format parsing
- Integrate via `ILinterAdapterPort` for consistent tool adapter pattern

### 3.3 Target Users
- **TypeScript Developers**: Catch type errors during `lint-arwaky scan`
- **DevOps/CI**: Enforce type safety in TypeScript projects

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a TypeScript developer, I want TSC type errors reported when I run `scan`, so I catch type mismatches early.
- **US-002:** As a CI maintainer, I want type errors to fail the scan with HIGH severity.

### 4.2 Use Cases & Workflow
```
Input: scan /project (TypeScript project)

1. TSCAdapter::scan() called with file path
2. Skip if extension is not .ts or .tsx
3. Resolve local tsc binary (node_modules/.bin/)
4. Build command: <resolved> --noEmit --pretty false <file>
5. Execute via ICommandExecutorPort
6. Parse output with two regex patterns → extract file/line/col/code/message
7. Return Vec<LintResult> (severity: HIGH)
```

### 4.3 Business Rules
- Severity: HIGH for all TSC errors
- Two regex patterns:
  - Pattern 1: `file(line,col): error TSxxxx: message`
  - Pattern 2: `file:line:col - error TSxxxx: message`
- Timeout: 60 seconds

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Parse TSC output per file | < 500ms |
| NFR-002 | Regex extraction accuracy | 100% of standard TSC errors captured |

## 6. UI/UX Requirements
```
AES080 HIGH - src/services/user.ts:25
  tsc: TS2322 — Type 'string' is not assignable to type 'number'.

AES080 HIGH - src/models/base.ts:12
  tsc: TS18046 — 'value' is of type 'unknown'.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | TypeScript file with type errors | Scan runs | AES080 HIGH entries reported per error | Pending Review |
| AC-002 | TypeScript file with no errors | Scan runs | No AES080 entries | Pending Review |
| AC-003 | No tsconfig.json found | Scan runs | Graceful error handling | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_js_linter.rs` (lines 205–329). TSC output parsing uses two regex patterns to handle different TSC output formats. The `--pretty false` flag is passed to ensure machine-parseable output. Errors are always mapped to `Severity::HIGH`. Working directory resolution walks up to find `package.json` or config files.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired in orchestrator | Register via ServiceContainerAggregate |
| typescript (tsc) | External Node.js tool | Not installed | Document as prerequisite; graceful skip |
| tsconfig.json | Required configuration file | Missing in some projects | TSC will report its own error |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_js_linter.rs:205` — TSCAdapter implementation
- `src-rust/language-adapters/mod.rs:9` — Module export
- `src-rust/di-containers/contract_service_aggregate.rs` — ILinterAdapterPort trait
