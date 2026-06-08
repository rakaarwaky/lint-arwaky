# 📄 Feature Requirements Document (FRD)
**Feature Name:** File Size Limit Checker (AES004)  
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
This document defines the AES004 rule that detects files exceeding the maximum allowed line count. The rule is implemented in `check_line_counts()` within `ArchMetricChecker`. AES004 ensures no file exceeds the `max_lines` threshold (global default: 700).

### 2.2 Scope
**In-Scope:**
- Checking per-file line count against the `max_lines` threshold
- Exception handling for barrel files (`__init__.py`, `mod.rs`) and YAML-defined exceptions
- Custom violation messages from YAML (`max_lines_violation_message`)
- HIGH severity reporting

**Out-of-Scope:**
- Minimum line count checking (AES005 — separate FRD)
- Per-layer threshold differentiation (currently global only)
- Auto-fixing violations

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES004** | Rule code for file exceeding maximum line limit |
| **check_line_counts()** | Main detection method in `ArchMetricChecker` |
| **max_lines** | Configurable maximum line threshold (default: 700) |
| **count_lines()** | Helper that counts lines in a file |

## 3. Feature Overview
### 3.1 Background & Problem
Before AES004, files had no size limit enforcement. Files could grow to thousands of lines, violating the Single Responsibility Principle and becoming difficult to maintain or test. The AES architecture requires focused, measurable file sizes.

### 3.2 Business Goals
- Prevent oversized files that are hard to maintain and test
- Provide clear, actionable violation messages
- Make thresholds configurable via YAML

### 3.3 Target Users
- **Developers**: Get automatic feedback when a file is too large
- **Architects**: Configure `max_lines` threshold per project via YAML

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when my file exceeds the maximum line limit, so I can split it into smaller modules.
- **US-002:** As an architect, I want the `max_lines` threshold configurable in YAML, so I can adapt it per project standards.

### 4.2 Use Cases & Workflow
**Detection Pipeline:**
```
File: layer-rules/capabilities_metric_checker.rs

1. Extract basename from file path
2. Is basename == "__init__.py" or "mod.rs"? → SKIP (barrel)
3. Is basename in exceptions list? → SKIP
4. count = count_lines(file) → count lines
5. If count > max_lines (700) → AES004 HIGH
```

### 4.3 Business Rules
- Severity: HIGH
- Barrel files (`__init__.py`, `mod.rs`) are skipped
- Exception list from YAML: `["main.rs", "lib.rs", "mod.rs", "python_taxonomy_bridge.rs", "js_taxonomy_bridge.rs"]`
- If `max_lines` <= 0, rule is skipped
- Custom YAML message takes priority; default used if empty

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |
| NFR-002 | False positive rate | 0% for valid files |
| NFR-003 | False negative rate | 0% for violating files |

## 6. UI/UX Requirements
```
AES004 HIGH - src-rust/layer-rules/massive_file.rs
  AES004 FILE_TOO_LARGE: File exceeds the 700-line limit.
  WHY? Large files violate the Single Responsibility Principle.
  FIX: Split the module into smaller, more focused files (max: 700).
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | 504-line file (massive_domain_entity.rs) | `check_line_counts()` runs | No AES004 (504 < 700) | Pending Review |
| AC-002 | File > 700 lines (no fixture exists) | `check_line_counts()` runs | AES004 HIGH flagged | Pending Review (logic correct) |
| AC-003 | Barrel file (`__init__.py` / `mod.rs`) | `check_line_counts()` runs | Skipped | Pending Review |
| AC-004 | Exception-listed file | `check_line_counts()` runs | Skipped | Pending Review |
| AC-005 | Unreadable file (count_lines fails) | `count_lines()` runs | Returns 0 — no overflow | Pending Review OK for AES004 |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location**: `src-rust/layer-rules/capabilities_metric_checker.rs:75-128`
- **AES004 portion**: `capabilities_metric_checker.rs:114-127`
- **Status**: **FULLY IMPLEMENTED** — not a stub
- Invoked from `agent_checking_coordinator.rs:98`

### 8.2 Bugs Found

1. **Global threshold with no per-layer override** (`lint_arwaky.config.rust.yaml:131`)
   - `max_lines: 700` applies to ALL layers
   - Cannot differentiate threshold for taxonomy vs capabilities vs surfaces
   - **Fix**: support `max_lines` per scope in YAML

2. **No Rust unit tests** for `ArchMetricChecker`
   - No `#[cfg(test)]` module in `capabilities_metric_checker.rs`
   - Testing relies entirely on test-project fixtures
   - **Fix**: add unit tests for `count_lines()` and `check_line_counts()`

### 8.3 What Needs to Be Added
- **Per-layer threshold**: support `max_lines` config per individual scope/layer
- **Unit tests**: at minimum for `count_lines()` and `check_line_counts()`
- **Rust AES004 test fixture**: TEST.md mentions `extremely_large_vo` (line 59) but that file **does not exist** in test-project-rust. Create a file > 700 lines for Rust.

### 8.4 What to Keep
- **Barrel file exclusion logic** Pending Review (lines 83-86)
- **Clear, actionable default messages** Pending Review (lines 119-124)
- **Custom YAML message support** Pending Review (lines 116-117)
- **Coordinator pipeline integration** Pending Review (agent_checking_coordinator.rs:98)

### 8.5 Empirical Evidence from Test Projects
- `test-project-rust/src-rust/shared-common/massive_domain_entity.rs` (504 lines) → no overflow (504 < 700) Pending Review
- `test-project-python/src-python/taxonomy/large_domain_entity.py` — needs line count check
- **AES004 in TEST.md**: mentions `extremely_large_vo` but **file not found** → missing fixture Pending Review

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | File content reading | File unreadable → count_lines returns 0 (OK for AES004) | Low risk |
| Config YAML | Threshold from config | Unreasonable threshold (0 or negative) | Skip if <= 0 Pending Review |
| Test fixtures | File > 700 lines | `extremely_large_vo` missing | Create new fixture |

## 10. Appendices
- `src-rust/layer-rules/capabilities_metric_checker.rs:75` — `check_line_counts()`
- `src-rust/layer-rules/capabilities_metric_checker.rs:38` — `count_lines()`
- `src-rust/shared-common/taxonomy_layer_vo.rs` — `max_lines` field
- `src-rust/pipeline-jobs/agent_checking_coordinator.rs:98` — Invocation
- `lint_arwaky.config.rust.yaml:131` — max_lines config
- `test-project-rust/src-rust/shared-common/massive_domain_entity.rs` — Test fixture (504 lines)
