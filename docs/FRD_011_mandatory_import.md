# Feature Requirements Document (FRD)

**Feature Name:** Mandatory Import Missing Detector (AES002)
**Product:** Lint Arwaky v1.10.2
**Author:** Raka
**Date:** 08/06/2026
**Version:** v1.0

## 1. Document Control

| Version | Date       | Author | Description of Changes    | Approved By   |
| ------- | ---------- | ------ | ------------------------- | ------------- |
| v1.0    | 08/06/2026 | Raka   | Initial document creation | [Stakeholder] |

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

- **US-001:** As a developer in capabilities/, I want to be warned if I forget to import contract(protocol), so my code follows architecture rules.
- **US-002:** As a developer, I don't want false positives when my file genuinely doesn't need a mandatory import, so I'm not annoyed by irrelevant warnings.

### 4.2 Use Cases & Workflow

**Detection Pipeline:**

```
File: capabilities/my_analyzer.rs
mandatory for capabilities: ["taxonomy", "contract(protocol)"]

1. resolve_scope("contract(protocol)") → layer="contract", suffixes=["protocol"]
2. Check content: does it contain "contract"? → YES ✅
3. Check import lines: does any import match scope? → YES ✅
4. Result: no violation

File: capabilities/no_contract.rs
mandatory for capabilities: ["taxonomy", "contract(protocol)"]

1. resolve_scope("contract(protocol)") → layer="contract", suffixes=["protocol"]
2. Check content: does it contain "contract"? → NO ❌
3. Check import lines: any import matching scope? → NO ❌
4. Check genuinely_unreferenced: does file use any contract types? → NO
5. genuinely_unreferenced? → YES → skip (no false positive) ✅
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
AES002 HIGH - src-rust/capabilities/my_file.rs
  Missing required import: 'contract(protocol)'.
  WHY? Capabilities require contract(protocol) for use-case interfaces.
  FIX: Add 'use crate::contract::IArchLintProtocol' or similar protocol import.
```

## 7. Acceptance Criteria

| ID     | Given                                               | When                               | Then                        | Status |
| ------ | --------------------------------------------------- | ---------------------------------- | --------------------------- | ------ |
| AC-001 | Capabilities file missing contract(protocol) import | `check_mandatory_imports()` runs | AES002 HIGH flagged         | ✅     |
| AC-002 | Capabilities file has taxonomy + contract imports   | `check_mandatory_imports()` runs | No violation                | ✅     |
| AC-003 | File genuinely doesn't use any contract types       | `check_mandatory_imports()` runs | Skipped (no false positive) | ✅     |

## 8. Dependencies & Risks

| Dependency                       | Description              | Risk                                          | Mitigation         |
| -------------------------------- | ------------------------ | --------------------------------------------- | ------------------ |
| FR-001 (Architecture)            | Layer definitions needed | Architecture changes break rules              | Config-driven      |
| `genuinely_unreferenced` logic | Content matching         | False negative (fails to detect actual usage) | Conservative check |

## 9. Appendices

- `src-rust/capabilities/architecture_import_checker.rs:133` — `check_mandatory_imports()`
- `src-rust/taxonomy/layer_definition_vo.rs` — `mandatory_import` config field
- `docs/RULES_AES.md` — Mandatory import matrix
