# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run Prettier (FR-079)  
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
This document defines the external tool adapter that runs Prettier formatting check on JavaScript/TypeScript files. The adapter invokes `prettier --check` via `ICommandExecutorPort` and reports files with formatting issues.

### 2.2 Scope
**In-Scope:**
- Invoking `prettier --check` on `.js`, `.jsx`, `.ts`, `.tsx`, `.json`, `.css`, `.md`, `.yml` files
- Detecting formatting violations from stderr `[warn]` output
- Auto-fixing with `prettier --write` via `apply_fix()`
- Local binary resolution via `resolve_js_cmd()`

**Out-of-Scope:**
- Installing Prettier or its plugins
- Configuration generation
- Running on non-web file types

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **PrettierAdapter** | Infrastructure struct in `infrastructure_js_linter.rs` implementing `ILinterAdapterPort` |
| **prettier --check** | Prettier's check-only mode — exits with code 1 if files are not formatted |
| **prettier --write** | Prettier's auto-format mode — writes formatted output to files |

## 3. Feature Overview
### 3.1 Background & Problem
JS/TS projects use Prettier as their code formatter. Lint Arwaky integrates Prettier to detect formatting inconsistencies during scans and optionally fix them.

### 3.2 Business Goals
- Detect unformatted JS/TS files during scan
- Support auto-fix via `apply_fix()` method
- Integrate via `ILinterAdapterPort` for consistent tool adapter pattern

### 3.3 Target Users
- **JS/TS Developers**: Verify formatting without running `prettier` separately
- **DevOps/CI**: Enforce consistent formatting in JS/TS projects

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a JS/TS developer, I want formatting issues reported when I run `scan`, so I know which files need formatting.
- **US-002:** As a CI maintainer, I want misformatted files to trigger a (LOW severity) warning, so the pipeline documents style issues without blocking.

### 4.2 Use Cases & Workflow
```
Input: scan /project (JS/TS project)

1. PrettierAdapter::scan() called with file path
2. Skip if extension not in allowed list (.ts, .tsx, .js, .jsx, .json, .css, .md, .yml, .yaml)
3. Resolve local Prettier binary (node_modules/.bin/)
4. Build command: <resolved> --check <file>
5. Execute via ICommandExecutorPort
6. Check for "[warn]" in combined stdout+stderr
7. If warnings found → LintResult { severity: LOW }
```

### 4.3 Business Rules
- Severity: LOW for formatting issues
- One `LintResult` per unformatted file (not per issue)
- Local binary resolution: `node_modules/.bin/prettier` → `bunx prettier` → `npx prettier`
- Timeout: 60 seconds

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Per-file Prettier check | < 500ms |
| NFR-002 | False negative rate | 0% for files with formatting issues |

## 6. UI/UX Requirements
```
AES079 LOW - src/app.ts:1
  prettier: Code style issues found. Run Prettier to fix.

AES079 LOW - src/utils.ts:1
  prettier: Code style issues found. Run Prettier to fix.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | JS/TS file with formatting issues | Scan runs | AES079 LOW reported | Pending Review |
| AC-002 | All files properly formatted | Scan runs | No AES079 entries | Pending Review |
| AC-003 | `apply_fix()` called | Fix runs | `prettier --write` executed on file | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_js_linter.rs` (lines 82–203). Unlike ESLint, Prettier's output is parsed by checking for `[warn]` in the combined stdout+stderr rather than JSON parsing. The `apply_fix()` method runs `prettier --write` on the given path. Working directory resolution walks up to find `package.json` or config files.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired in orchestrator | Register via ServiceContainerAggregate |
| prettier | External Node.js tool | Not installed | Document as prerequisite; graceful skip |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_js_linter.rs:82` — PrettierAdapter implementation
- `src-rust/language-adapters/mod.rs:9` — Module export
- `src-rust/di-containers/contract_service_aggregate.rs` — ILinterAdapterPort trait
