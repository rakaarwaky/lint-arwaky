# 📄 Feature Requirements Document (FRD)
**Feature Name:** Import Layer Violation Detector (AES001)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.1  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v1.1 | 09/06/2026 | Raka | Updated workflow to prefix-based layer detection; updated file paths | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES001 rule that detects and flags forbidden cross-layer imports. Each layer has a list of forbidden import targets defined in YAML — any file importing from a forbidden target is flagged as CRITICAL.

### 2.2 Scope
**In-Scope:**
- Parsing import statements from Rust (`use`), Python (`import`/`from`), JS/TS (`import`/`require`)
- Extracting target module from import paths
- Resolving target layer from module path
- Matching against per-layer forbidden list
- CRITICAL severity with auto-fail

**Out-of-Scope:**
- Rules AES002–AES033 (separate FRDs)
- Auto-fixing import violations (not auto-fixable)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES001** | Rule code for forbidden import layer violation |
| **check_forbidden_imports()** | Main detection method |
| **extract_module_from_line()** | Parses import to get root module |
| **detect_module_layer()** | Resolves target layer from module path |
| **resolve_scope()** | Parses `contract(protocol)` → layer + suffixes |

## 3. Feature Overview
### 3.1 Background & Problem
Before AES001, any module could import any other module — capabilities imported infrastructure, surfaces imported capabilities directly. There were no cross-layer guards, import rules existed only in documentation, and violations were only caught during manual code review.

### 3.2 Business Goals
- Prevent architectural layer violations automatically
- Enforce forbidden import matrix at lint time
- Provide clear, actionable violation messages
- Auto-fail on CRITICAL violations

### 3.3 Target Users
- **Developers**: Get immediate feedback when importing from wrong layer
- **Architects**: Enforce architecture rules without manual reviews

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be notified immediately when I import from a forbidden layer, so I can fix it before committing.
- **US-002:** As an architect, I want the forbidden import matrix to be configurable in YAML, so I can adapt rules per project.

### 4.2 Use Cases & Workflow
**Detection Pipeline:**
```
File: layer-rules/capabilities_import_checker.rs
Layer (from prefix `capabilities_`): capabilities

1. Parse import: "use crate::infrastructure_fs_scanner::FileSystemScanner"
2. Extract target layer from import prefix: "infrastructure_"
3. Look up forbidden list for capabilities:
   → ["infrastructure", "surfaces", "agent", "capabilities"]
4. "infrastructure" IS in forbidden list
5. Flag AES001 CRITICAL
```

### 4.3 Business Rules
- Severity: CRITICAL (auto-fail if any found)
- Forbidden list from `LayerDefinition.forbidden_import.values`
- Scope resolution handles `contract(protocol)` notation

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 10ms |
| NFR-002 | False positives | 0% for simple imports |

## 6. UI/UX Requirements
```
AES001 CRITICAL - src-rust/layer-rules/capabilities_import_checker.rs:42
  Layer 'capabilities' (prefix `capabilities_`) is importing from forbidden layer 'infrastructure' (prefix `infrastructure_`).
  WHY? Cross-layer leakage breaks architectural decoupling.
  FIX: Use ports/protocols from the contract layer instead.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Capabilities file imports infrastructure | `check_forbidden_imports()` runs | AES001 CRITICAL flagged | Pending Review |
| AC-002 | Surface file imports capabilities | `check_forbidden_imports()` runs | AES001 CRITICAL flagged | Pending Review |
| AC-003 | Contract file imports taxonomy | `check_forbidden_imports()` runs | No violation (allowed) | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `ArchImportRuleChecker` | `layer-rules/capabilities_import_checker.rs` | 557 | **FULLY IMPLEMENTED** |
| `check_forbidden_imports()` | `capabilities_import_checker.rs:294` | 64 lines | **FULLY IMPLEMENTED** |
| `check_mandatory_imports()` | `capabilities_import_checker.rs:212` | 82 lines | **FULLY IMPLEMENTED** |
| `check_legacy_import_rules()` | `capabilities_import_checker.rs:358` | 200 lines | **FULLY IMPLEMENTED** |
| `resolve_scope()` | `capabilities_import_checker.rs:29` | 22 lines | **FULLY IMPLEMENTED** — parses `contract(protocol)` notation |
| `import_matches_scope()` | `capabilities_import_checker.rs:54` | — | **FULLY IMPLEMENTED** |
| `detect_layer()` | `layer-rules/capabilities_compliance_analyzer.rs:166` | — | **FULLY IMPLEMENTED** — prefix-based detection |
| Invocation | `code-analysis/agent_checking_coordinator.rs:139-141` | — | **FULLY IMPLEMENTED** — called in `run_all_checks()` |

### 8.2 Bugs Found

1. **`detect_module_layer()` uses substring matching, not prefix** (`capabilities_compliance_analyzer.rs:237`)
   ```rust
   if import_path.contains("::taxonomy::") { return Some("taxonomy".into()); }
   ```
   - Uses `contains("::taxonomy::")` which matches any occurrence, including type names
   - A type named `TaxonomyConfig` in a fully qualified path could trigger false match
   - **Should use**: prefix-based check on the first module segment after `crate::`

2. **Forbidden import check emits AES001 for ALL violations** (`capabilities_import_checker.rs:339`)
   - Surface hierarchy violations (AES018/AES019) are caught by the same checker
   - But the error code is hardcoded as `AES001` instead of reading from config
   - **Impact**: Surface tier violations display wrong error code

3. **`import_matches_scope()` uses `lower.contains()`** — fragile for partial matches
   - `lower.contains(&format!("::{}::", layer))` matches substrings
   - Module path `some::infrastructure_extra::` would match `infrastructure` scope
   - **Impact**: False positives on module paths containing layer names as substrings

### 8.3 What Needs to Be Added

- **Prefix-based module detection**: Use first-segment extraction, not substring contains
- **Error code from config**: Read violation error codes from YAML instead of hardcoding AES001
- **Unit tests**: `resolve_scope()`, `import_matches_scope()`, `detect_module_layer()` have no dedicated tests

### 8.4 What to Keep

- **Three-method interface** ✅ — `check_forbidden`, `check_mandatory`, `check_legacy` cleanly separated
- **Scope resolution** ✅ — handles `contract(protocol)`, `taxonomy(entity,error,event)`, `contract(port|protocol|aggregate)`
- **Config-driven forbidden lists** ✅ — fully loaded from YAML layer definitions
- **Prefix-based layer detection** ✅ — `ArchComplianceAnalyzer::detect_layer()` uses filename prefix

### 8.5 Empirical Evidence from Test Projects

- `test-project-rust/` fixtures trigger AES001 for capabilities→infrastructure imports
- `test-project-rust/` fixtures trigger AES023 for surface→infrastructure imports
- `test-project-python/` fixtures trigger AES001 for cross-layer violations
- No fixture tests `detect_module_layer()` substring matching bug (section 8.2 item 1)
- Pending Review: All acceptance criteria

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Import parsing quality | Regex fails on complex imports | Document limitations |
| Config YAML | Forbidden list from config | Missing config = no rules | `default_aes_config()` fallback |

## 10. Appendices
- `src-rust/layer-rules/capabilities_import_checker.rs` — `check_forbidden_imports()`
- `src-rust/shared-common/taxonomy_layer_vo.rs` — `LayerDefinition.forbidden_import`
- `docs/RULES_AES.md` — Forbidden import matrix
