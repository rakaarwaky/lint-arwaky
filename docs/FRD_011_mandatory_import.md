# Feature Requirements Document (FRD)

**Feature Name:** Mandatory Import Missing Detector (AES002)
**Product:** Lint Arwaky v1.10.2
**Author:** Raka
**Date:** 09/06/2026
**Version:** v1.1

## 1. Document Control

| Version | Date       | Author | Description of Changes    | Approved By   |
| ------- | ---------- | ------ | ------------------------- | ------------- |
| v1.0    | 08/06/2026 | Raka   | Initial document creation | [Stakeholder] |
| v1.1    | 09/06/2026 | Raka   | Updated to prefix-based architecture: layers are filename prefixes, not directories; updated file paths to reflect 26 feature folders | [Stakeholder] |

## 2. Introduction

### 2.1 Purpose

This document defines the AES002 rule that detects missing mandatory imports. Each layer has required dependencies — if a file does not import its required layers (and genuinely uses types from them), it is flagged as HIGH.

### 2.2 Scope

**In-Scope:**

- Reading mandatory import list from layer definition
- Resolving scope notation (`contract(protocol)`)
- Checking file content for required imports
- Skipping genuinely unreferenced types (no false positives)
- HIGH severity reporting

**Out-of-Scope:**

- Forbidden import rules (AES001 — separate FRD)
- Auto-fixing (not auto-fixable)

### 2.3 Glossary

| Term                                | Definition                                      |
| ----------------------------------- | ----------------------------------------------- |
| **AES002**                    | Rule code for mandatory import missing          |
| **check_mandatory_imports()** | Main detection method                           |
| **mandatory_import**          | Config field listing required imports per layer |
| **genuinely_unreferenced**    | Guard that prevents false positives             |

## 3. Feature Overview

### 3.1 Background & Problem

Layers could skip required imports without any warning. For example, capabilities could exist without importing `contract(protocol)`, violating the architectural contract. There was no enforcement that layers actually use their required dependencies.

### 3.2 Business Goals

- Ensure each layer imports its required dependencies
- Prevent false positives by detecting genuine references
- Make mandatory import rules configurable via YAML

### 3.3 Target Users

- **Architects**: Enforce that layers consume their required interfaces
- **Developers**: Get reminded when missing required imports

## 4. Functional Requirements

### 4.1 User Stories

- **US-001:** As a developer writing a capabilities_-prefixed file, I want to be warned if I forget to import contract(protocol), so my code follows architecture rules.
- **US-002:** As a developer, I don't want false positives when my file genuinely doesn't need a mandatory import, so I'm not annoyed by irrelevant warnings.

### 4.2 Use Cases & Workflow

**Detection Pipeline:**

```
File: layer-rules/capabilities_my_analyzer.rs
  filename starts with "capabilities_" → layer = "capabilities"
mandatory for capabilities: ["taxonomy", "contract(protocol)"]

1. resolve_scope("contract(protocol)") → layer="contract", suffixes=["protocol"]
2. Check content: does it contain "contract"? → YES Pending Review
3. Check import lines: does any import match scope? → YES Pending Review
4. Result: no violation

File: layer-rules/capabilities_no_contract.rs
  filename starts with "capabilities_" → layer = "capabilities"
mandatory for capabilities: ["taxonomy", "contract(protocol)"]

1. resolve_scope("contract(protocol)") → layer="contract", suffixes=["protocol"]
2. Check content: does it contain "contract"? → NO Pending Review
3. Check import lines: any import matching scope? → NO Pending Review
4. Check genuinely_unreferenced: does file use any contract types? → NO
5. genuinely_unreferenced? → YES → skip (no false positive) Pending Review
```

### 4.3 Business Rules

- Severity: HIGH
- If mandatory list is empty → skip (no rules for this layer)
- Skip barrel files and exception files
- If genuinely unreferenced → skip (no false positive)

## 5. Non-Functional Requirements

| ID      | Requirement         | Target |
| ------- | ------------------- | ------ |
| NFR-001 | Detection per file  | < 10ms |
| NFR-002 | False positive rate | 0%     |

## 6. UI/UX Requirements

```
AES002 HIGH - src-rust/layer-rules/capabilities_my_file.rs
  Missing required import: 'contract(protocol)'.
  WHY? Capabilities require contract(protocol) for use-case interfaces.
  FIX: Add 'use crate::contract::IArchLintProtocol' or similar protocol import.
```

## 7. Acceptance Criteria

| ID     | Given                                               | When                               | Then                        | Status |
| ------ | --------------------------------------------------- | ---------------------------------- | --------------------------- | ------ |
| AC-001 | Capabilities file missing contract(protocol) import | `check_mandatory_imports()` runs | AES002 HIGH flagged         | Pending Review     |
| AC-002 | Capabilities file has taxonomy + contract imports   | `check_mandatory_imports()` runs | No violation                | Pending Review     |
| AC-003 | File genuinely doesn't use any contract types       | `check_mandatory_imports()` runs | Skipped (no false positive) | Pending Review     |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `check_mandatory_imports()` | `layer-rules/capabilities_import_checker.rs:212` | 82 | **FULLY IMPLEMENTED** |
| `resolve_scope()` | `capabilities_import_checker.rs:29` | 22 | **FULLY IMPLEMENTED** |
| `import_matches_scope()` | `capabilities_import_checker.rs:54` | — | **FULLY IMPLEMENTED** |
| `barrel_has_suffix_match()` | `capabilities_import_checker.rs` | — | **FULLY IMPLEMENTED** |
| Invocation | `code-analysis/agent_checking_coordinator.rs:139` | — | **FULLY IMPLEMENTED** |

Key features:
- `genuinely_unreferenced` guard prevents false positives when mandatory types aren't needed
- Barrel-verified suffix matching checks that imported types originate from correct suffix files
- Scope resolution handles full notation: `contract(protocol)`, `taxonomy(entity,error,event)`

### 8.2 Bugs Found

1. **`content.contains(layer)` false positives** (line 239, 255)
   - Content match uses substring contains — matches comments, strings, and documentation
   - A file might "contain" the layer name in a comment without actually importing it
   - **Impact**: Rare false negatives on genuinely_unreferenced check (conservative, low risk)

2. **`barrel_has_suffix_match()` adds overhead** — called for every mandatory import check
   - Reads barrel files (`mod.rs`/`__init__.py`) on every invocation
   - **Impact**: Performance overhead for large projects with many mandatory imports

3. **`read_import_lines()` reads file twice** — once in `check_mandatory_imports()` line 226, once for content at line 227
   - `read_import_lines()` re-reads the file internally
   - `fs::read_to_string()` reads it again for content matching
   - **Impact**: Double file I/O per file

### 8.3 What Needs to Be Added

- **Stricter content matching**: Check for `use crate::layer::...` pattern instead of generic `contains()`
- **Cache import lines**: Pass pre-parsed imports to avoid double file reads
- **Unit tests**: No dedicated tests for mandatory import scope matching edge cases

### 8.4 What to Keep

- **Genuinely-unreferenced guard** ✅ — prevents forcing dead imports
- **Barrel-verified suffix matching** ✅ — ensures contract(protocol) actually checks protocol files
- **Scope resolution** ✅ — `contract(protocol)` correctly expands to suffix filtering
- **YAML-driven messages** ✅ — custom violation messages supported

### 8.5 Empirical Evidence from Test Projects

- Capabilities files in test projects missing `contract(protocol)` trigger AES002
- Files genuinely not using contract types are correctly skipped
- No fixture tests the `barrel_has_suffix_match()` fallback path
- Pending Review: All acceptance criteria

## 9. Dependencies & Risks

| Dependency                       | Description              | Risk                                          | Mitigation         |
| -------------------------------- | ------------------------ | --------------------------------------------- | ------------------ |
| FR-001 (Architecture)            | Layer definitions needed | Architecture changes break rules              | Config-driven      |
| `genuinely_unreferenced` logic | Content matching         | False negative (fails to detect actual usage) | Conservative check |

## 10. Appendices

- `src-rust/layer-rules/capabilities_import_checker.rs:133` — `check_mandatory_imports()`
- `src-rust/shared-common/taxonomy_layer_vo.rs` — `mandatory_import` config field
- `docs/RULES_AES.md` — Mandatory import matrix
