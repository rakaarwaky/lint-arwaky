# 📄 Feature Requirements Document (FRD)
**Feature Name:** File Minimum Size Checker (AES005)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.1  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v1.1 | 09/06/2026 | Raka | Updated to prefix-based architecture: layers are filename prefixes, not directories; updated file paths for 26 feature folders | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES005 rule that detects files below the minimum allowed line count. The rule is implemented in `check_line_counts()` within `ArchMetricChecker`. AES005 ensures no file falls below the `min_lines` threshold (global default: 10).

### 2.2 Scope
**In-Scope:**
- Checking per-file line count against the `min_lines` threshold
- Exception handling for barrel files (`__init__.py`, `mod.rs`) and YAML-defined exceptions
- Custom violation messages from YAML (`min_lines_violation_message`)
- HIGH severity reporting

**Out-of-Scope:**
- Maximum line count checking (AES004 — separate FRD)
- Per-layer threshold differentiation (currently global only)
- Auto-fixing violations

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES005** | Rule code for file below minimum line limit |
| **check_line_counts()** | Main detection method in `ArchMetricChecker` |
| **min_lines** | Configurable minimum line threshold (default: 10) |
| **count_lines()** | Helper that counts lines in a file |

## 3. Feature Overview
### 3.1 Background & Problem
Before AES005, files could be created with just 1-2 lines without any warning. Excessively small files clutter the project structure and indicate that logic should be merged into a related module. The AES architecture requires meaningful, appropriately-sized components.

### 3.2 Business Goals
- Prevent tiny files that clutter the project structure
- Encourage merging small logic into relevant modules
- Make thresholds configurable via YAML

### 3.3 Target Users
- **Developers**: Get automatic feedback when a file is too small
- **Architects**: Configure `min_lines` threshold per project via YAML

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when my file is too small, so I can merge it with a related module.
- **US-002:** As an architect, I want the `min_lines` threshold configurable in YAML, so I can adapt it per project standards.

### 4.2 Use Cases & Workflow
**Detection Pipeline:**
```
File: infrastructure/tiny.py

1. Extract basename from file path
2. Is basename == "__init__.py" or "mod.rs"? → SKIP (barrel)
3. Is basename in exceptions list? → SKIP
4. count = count_lines(file) → count lines
5. If count < min_lines (10) → AES005 HIGH
```

### 4.3 Business Rules
- Severity: HIGH
- Barrel files (`__init__.py`, `mod.rs`) are skipped
- Exception list from YAML: `["main.rs", "lib.rs", "mod.rs", "python_taxonomy_bridge.rs", "js_taxonomy_bridge.rs"]`
- If `min_lines` <= 0, rule is skipped
- Custom YAML message takes priority; default used if empty

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |
| NFR-002 | False positive rate | 0% for valid files |
| NFR-003 | False negative rate | 0% for violating files |

## 6. UI/UX Requirements
```
AES005 HIGH - src-rust/infrastructure/tiny.py
  AES005 FILE_TOO_SHORT: File contains fewer than 10 lines of code.
  WHY? Excessively small files clutter the project structure.
  FIX: Merge this logic into a related module (min: 10).
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | 1-line file (tiny.py) | `check_line_counts()` runs | AES005 HIGH flagged | Pending Review |
| AC-002 | File with 10+ lines | `check_line_counts()` runs | No AES005 | Pending Review |
| AC-003 | Barrel file (`__init__.py` / `mod.rs`) | `check_line_counts()` runs | Skipped | Pending Review |
| AC-004 | Exception-listed file | `check_line_counts()` runs | Skipped | Pending Review |
| AC-005 | Unreadable file (count_lines fails) | `count_lines()` runs | Returns 0 → **false positive AES005** | Pending Review **BUG** |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location**: `src-rust/layer-rules/capabilities_metric_checker.rs:75-128`
- **AES005 portion**: `capabilities_metric_checker.rs:99-112`
- **Status**: **FULLY IMPLEMENTED** — not a stub
- Invoked from `agent_checking_coordinator.rs:98`

### 8.2 Bugs Found

1. **`count_lines()` returns 0 on read failure** (CRITICAL — `capabilities_metric_checker.rs:38-42`)
   - **Location**: `capabilities_metric_checker.rs:41` — `unwrap_or(0)`
   - If `fs::read_to_string` fails (missing file, permission denied, etc.), returns 0
   - Since 0 < `min_lines` (10), this causes a **false positive AES005**
   - **Impact**: unreadable files get flagged as too short even if they are valid
   - **Fix**: return -1 and skip check when value is negative, or change to `Option<i64>`

2. **Global threshold with no per-layer override** (`lint_arwaky.config.rust.yaml:130`)
   - `min_lines: 10` applies to ALL layers
   - Cannot differentiate threshold for taxonomy vs infrastructure vs surfaces
   - **Fix**: support `min_lines` per scope in YAML

3. **No Rust unit tests** for `ArchMetricChecker`
   - No `#[cfg(test)]` module in `capabilities_metric_checker.rs`
   - **Fix**: add unit tests for `count_lines()` and `check_line_counts()`, especially read-error edge case

### 8.3 What Needs to Be Added
- **Error handling**: don't return 0 on read failure. Use `Option<i64>` instead
- **Per-layer threshold**: support `min_lines` config per individual scope/layer
- **Skip check if count_lines fails**: add guard in `check_line_counts()` before threshold evaluation
- **Unit tests**: at minimum for `count_lines()` with various conditions (normal, missing, empty file)

### 8.4 What to Keep
- **Barrel file exclusion logic** Pending Review (lines 83-86)
- **Exceptions list handling** Pending Review (lines 93-95)
- **Clear, actionable default messages** Pending Review (lines 104-109)
- **Custom YAML message support** Pending Review (lines 101-102)
- **Skip if min_lines <= 0** Pending Review (line 100)
- **Coordinator pipeline integration** Pending Review (agent_checking_coordinator.rs:98)

### 8.5 Empirical Evidence from Test Projects
- **AES005**: `test-project-python/src-python/infrastructure/tiny.py` (1 line) → flagged Pending Review
- **AES005**: TEST.md line 60 mentions Rust AES005 from `invalid_import_vo`, `removal_types`, `missing_import_analyzer`
  - `test-project-rust/src-rust/shared-common/bare_entity.rs` (2 lines) — will be flagged Pending Review
  - `test-project-rust/src-rust/shared-common/bypass_comment_entity.rs` — needs line count verification

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | File content reading | File unreadable → false positive AES005 | Fix: return Option instead of 0 |
| Config YAML | Threshold from config | `min_lines: 0` or negative | Skip if <= 0 Pending Review |
| Test fixtures | File < 10 lines | Rust fixtures exist (bare_entity.rs = 2 lines) | Pending Review sufficient |

## 10. Appendices
- `src-rust/layer-rules/capabilities_metric_checker.rs:75` — `check_line_counts()`
- `src-rust/layer-rules/capabilities_metric_checker.rs:38` — `count_lines()` (BUG: unwrap_or(0))
- `src-rust/shared-common/taxonomy_layer_vo.rs` — `min_lines` field
- `src-rust/pipeline-jobs/agent_checking_coordinator.rs:98` — Invocation
- `lint_arwaky.config.rust.yaml:130` — min_lines config
- `test-project-python/src-python/infrastructure/tiny.py` — Test fixture (1 line)
- `test-project-rust/src-rust/shared-common/bare_entity.rs` — Test fixture (2 lines)
