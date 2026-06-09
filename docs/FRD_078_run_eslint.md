# 📄 Feature Requirements Document (FRD)
**Feature Name:** Run ESLint (FR-078)  
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
This document defines the external tool adapter that runs ESLint on JavaScript/TypeScript files. The adapter invokes ESLint via `npx eslint --format json` (or `bunx eslint` if Bun is available), and parses the JSON output into structured `LintResult` entries.

### 2.2 Scope
**In-Scope:**
- Invoking ESLint on `.js`, `.jsx`, `.ts`, `.tsx` files
- Parsing ESLint JSON output for lint violations
- Mapping ESLint severity (2=error, 1=warning) to AES severity
- Auto-fixing with `eslint --fix` via `apply_fix()`
- Local `node_modules/.bin/eslint` resolution

**Out-of-Scope:**
- Installing ESLint or its plugins
- Running ESLint on non-JS/TS files
- Configuration generation (expects existing `.eslintrc.*`)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **ESLintAdapter** | Infrastructure struct in `infrastructure_js_linter.rs` implementing `ILinterAdapterPort` |
| **resolve_js_cmd()** | Helper that resolves local `node_modules/.bin` binary or falls back to `npx`/`bunx` |
| **ILinterAdapterPort** | Contract trait for external linter adapters |

## 3. Feature Overview
### 3.1 Background & Problem
JavaScript/TypeScript projects rely on ESLint for linting. Lint Arwaky needs an ESLint adapter so JS/TS projects scanned by the tool receive ESLint diagnostics alongside AES rules.

### 3.2 Business Goals
- Surface ESLint violations in unified scan output
- Support auto-fix via `apply_fix()` method
- Integrate via `ILinterAdapterPort` for consistent tool adapter pattern

### 3.3 Target Users
- **JS/TS Developers**: Get ESLint feedback during `lint-arwaky scan`
- **DevOps/CI**: Include ESLint in automated quality gates

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a JS/TS developer, I want ESLint violations reported when I run `scan`, so I see lint issues alongside AES rules.
- **US-002:** As a project maintainer, I want `--fix` to apply ESLint auto-fixable rules.

### 4.2 Use Cases & Workflow
```
Input: scan /project (JS/TS project)

1. ESLintAdapter::scan() called with file path
2. Skip if extension not .js/.jsx/.ts/.tsx
3. Resolve local ESLint binary (node_modules/.bin/eslint)
4. Build command: <resolved> --format json <file>
5. Execute via ICommandExecutorPort
6. Parse JSON array → extract messages with ruleId, line, column, severity
7. Return Vec<LintResult>
```

### 4.3 Business Rules
- Severity mapping: ESLint severity 2 → AES HIGH, severity 1 → AES MEDIUM
- Resolves local `node_modules/.bin/eslint` first, then falls back to `npx eslint` or `bunx eslint`
- Timeout: 60 seconds
- `apply_fix()` runs `eslint --fix` on the file

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Parse ESLint JSON per file | < 500ms |
| NFR-002 | JSON parse accuracy | 100% of messages captured |

## 6. UI/UX Requirements
```
AES078 HIGH - src/app.ts:42
  eslint: no-unused-vars — 'unusedVar' is assigned but never used.

AES078 MEDIUM - src/utils.js:15
  eslint: semi — Missing semicolon.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | JS/TS project with ESLint violations | Scan runs | AES078 entries reported per violation | Pending Review |
| AC-002 | JS/TS project with no violations | Scan runs | No AES078 entries | Pending Review |
| AC-003 | No ESLint config found | Scan runs | Graceful error handling | Pending Review |

## 8. Empirical Findings
The adapter is implemented in `src-rust/language-adapters/infrastructure_js_linter.rs` (lines 331–490). The `resolve_js_cmd()` helper checks for local `node_modules/.bin/eslint`, then checks for Bun availability (`bun --version`), falling back to `npx eslint`. ESLint output is parsed with `serde_json::Value` from a JSON array of file results. The `apply_fix()` method runs `eslint --fix` on the given file path.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Architecture Compliance) | Full compliance analysis pipeline | Adapter not wired in orchestrator | Register via ServiceContainerAggregate |
| eslint | External Node.js tool | Not installed | Document as prerequisite; graceful skip |

## 10. Appendices
- `src-rust/language-adapters/infrastructure_js_linter.rs:331` — ESLintAdapter implementation
- `src-rust/language-adapters/mod.rs:9` — Module export
- `src-rust/di-containers/contract_service_aggregate.rs` — ILinterAdapterPort trait
